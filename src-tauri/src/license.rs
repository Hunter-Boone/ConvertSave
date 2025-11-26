//! License validation and management module
//!
//! Handles the complete licensing flow:
//! 1. Check for local license file
//! 2. Validate license (MAC address, expiration)
//! 3. Refresh from server if needed
//! 4. Activate new devices with product key

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use scrypt::{scrypt, Params};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// The encryption key - MUST match LICENSE_ENCRYPTION_KEY from the website
const ENCRYPTION_KEY: &str = match option_env!("LICENSE_ENCRYPTION_KEY") {
    Some(key) => key,
    None => "PLACEHOLDER_KEY_SET_LICENSE_ENCRYPTION_KEY_ENV_VAR",
};

/// API base URL for license operations
/// Set via CONVERTSAVE_API_URL env var at compile time for development
/// Default: https://convertsave.com/api/license
const API_BASE_URL: &str = match option_env!("CONVERTSAVE_API_URL") {
    Some(url) => url,
    None => "https://convertsave.com/api/license",
};

/// Grace period in days before locking app after subscription expires
const GRACE_PERIOD_DAYS: i64 = 2;

/// Decrypted license data structure (matches server's LicenseData)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseData {
    pub product_key: String,
    pub mac_address: String,
    pub plan_type: PlanType,
    pub subscription_end_date: Option<String>,
    pub issued_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlanType {
    Monthly,
    Yearly,
    Lifetime,
}

/// Overall license status returned to the frontend
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseStatus {
    pub is_valid: bool,
    pub is_activated: bool,
    pub plan_type: Option<PlanType>,
    pub days_remaining: Option<i64>,
    pub in_grace_period: bool,
    pub error: Option<String>,
    pub requires_activation: bool,
}

impl Default for LicenseStatus {
    fn default() -> Self {
        Self {
            is_valid: false,
            is_activated: false,
            plan_type: None,
            days_remaining: None,
            in_grace_period: false,
            error: None,
            requires_activation: true,
        }
    }
}

/// Response from /api/license/lookup
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct LookupResponse {
    found: bool,
    license: Option<String>,
    plan_type: Option<String>,
    subscription_end_date: Option<String>,
    error: Option<String>,
}

/// Response from /api/license/validate
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ValidateResponse {
    success: bool,
    license: Option<String>,
    plan_type: Option<String>,
    subscription_end_date: Option<String>,
    error: Option<String>,
}

/// Response from /api/license/refresh
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct RefreshResponse {
    success: bool,
    license: Option<String>,
    subscription_end_date: Option<String>,
    is_active: Option<bool>,
    error: Option<String>,
}

/// Get the path to the license file
pub fn get_license_path() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir().ok_or("Could not find data directory")?;
    let app_dir = data_dir.join("com.convertsave");
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    Ok(app_dir.join("license.dat"))
}

/// Save the encrypted license to disk
pub fn save_license(encrypted_license: &str) -> Result<(), String> {
    let path = get_license_path()?;
    fs::write(&path, encrypted_license).map_err(|e| format!("Failed to save license: {}", e))
}

/// Load the encrypted license from disk
pub fn load_license() -> Result<String, String> {
    let path = get_license_path()?;
    if !path.exists() {
        return Err("License file not found".to_string());
    }
    fs::read_to_string(&path).map_err(|e| format!("Failed to read license: {}", e))
}

/// Delete the local license file
pub fn delete_license() -> Result<(), String> {
    let path = get_license_path()?;
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete license: {}", e))?;
    }
    Ok(())
}

/// Derive the encryption key using scrypt (matching the Node.js implementation)
/// Node.js crypto.scryptSync defaults: N=16384 (2^14), r=8, p=1
fn derive_key() -> Result<[u8; 32], String> {
    let params = Params::new(14, 8, 1, 32).map_err(|e| format!("Scrypt params error: {}", e))?;
    let mut key = [0u8; 32];
    scrypt(ENCRYPTION_KEY.as_bytes(), b"salt", &params, &mut key)
        .map_err(|e| format!("Scrypt error: {}", e))?;
    Ok(key)
}

/// Decrypt the license blob
/// Format: base64(nonce[12] + authTag[16] + ciphertext)
pub fn decrypt_license(encrypted_license: &str) -> Result<LicenseData, String> {
    let data = BASE64
        .decode(encrypted_license)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    if data.len() < 29 {
        return Err("Invalid license data: too short".to_string());
    }

    let nonce = &data[0..12];      // 12 bytes nonce (standard for GCM)
    let auth_tag = &data[12..28];   // 16 bytes auth tag
    let ciphertext = &data[28..];   // rest is ciphertext

    let key = derive_key()?;
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| format!("Cipher creation error: {}", e))?;
    let nonce = Nonce::from_slice(nonce);

    // Combine ciphertext and auth tag for decryption (aes-gcm expects tag at end)
    let mut combined = ciphertext.to_vec();
    combined.extend_from_slice(auth_tag);

    let decrypted = cipher
        .decrypt(nonce, combined.as_ref())
        .map_err(|_| "Decryption failed: invalid license or key".to_string())?;

    serde_json::from_slice(&decrypted).map_err(|e| format!("JSON parse error: {}", e))
}

/// Get the current device's MAC address
#[cfg(target_os = "windows")]
pub fn get_mac_address() -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("getmac")
        .args(["/fo", "csv", "/nh"])
        .output()
        .map_err(|e| format!("Failed to get MAC address: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if let Some(mac) = line.split(',').next() {
            let mac = mac.trim_matches('"').replace('-', ":");
            if !mac.is_empty() && mac != "N/A" && !mac.contains("Media") {
                return Ok(mac.to_uppercase());
            }
        }
    }

    Err("Could not determine MAC address".to_string())
}

#[cfg(target_os = "macos")]
pub fn get_mac_address() -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("ifconfig")
        .arg("en0")
        .output()
        .map_err(|e| format!("Failed to get MAC address: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("ether ") {
            if let Some(mac) = line.split_whitespace().nth(1) {
                return Ok(mac.to_uppercase());
            }
        }
    }

    Err("Could not determine MAC address".to_string())
}

#[cfg(target_os = "linux")]
pub fn get_mac_address() -> Result<String, String> {
    let net_dir = "/sys/class/net";
    if let Ok(entries) = fs::read_dir(net_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str == "lo"
                || name_str.starts_with("veth")
                || name_str.starts_with("docker")
                || name_str.starts_with("br-")
            {
                continue;
            }

            let address_path = entry.path().join("address");
            if let Ok(mac) = fs::read_to_string(&address_path) {
                let mac = mac.trim().to_uppercase();
                if !mac.is_empty() && mac != "00:00:00:00:00:00" {
                    return Ok(mac);
                }
            }
        }
    }

    Err("Could not determine MAC address".to_string())
}

/// Validate a decrypted license
fn validate_license_data(license: &LicenseData, mac_address: &str) -> LicenseStatus {
    // Check MAC address
    if license.mac_address != mac_address {
        return LicenseStatus {
            is_valid: false,
            error: Some("License not valid for this device".to_string()),
            ..Default::default()
        };
    }

    // Lifetime licenses are always valid
    if license.plan_type == PlanType::Lifetime {
        return LicenseStatus {
            is_valid: true,
            is_activated: true,
            plan_type: Some(PlanType::Lifetime),
            days_remaining: None,
            in_grace_period: false,
            error: None,
            requires_activation: false,
        };
    }

    // Check subscription expiration
    if let Some(end_date_str) = &license.subscription_end_date {
        if let Ok(end_date) = end_date_str.parse::<DateTime<Utc>>() {
            let now = Utc::now();
            let days_remaining = (end_date - now).num_days();

            if days_remaining < -GRACE_PERIOD_DAYS {
                // Past grace period - invalid
                return LicenseStatus {
                    is_valid: false,
                    is_activated: true,
                    plan_type: Some(license.plan_type.clone()),
                    days_remaining: Some(days_remaining),
                    in_grace_period: false,
                    error: Some("Subscription has expired".to_string()),
                    requires_activation: false,
                };
            } else if days_remaining < 0 {
                // In grace period
                return LicenseStatus {
                    is_valid: true,
                    is_activated: true,
                    plan_type: Some(license.plan_type.clone()),
                    days_remaining: Some(days_remaining),
                    in_grace_period: true,
                    error: None,
                    requires_activation: false,
                };
            } else {
                // Valid subscription
                return LicenseStatus {
                    is_valid: true,
                    is_activated: true,
                    plan_type: Some(license.plan_type.clone()),
                    days_remaining: Some(days_remaining),
                    in_grace_period: false,
                    error: None,
                    requires_activation: false,
                };
            }
        }
    }

    // No end date for subscription plan - treat as valid temporarily
    // The app should try to refresh to get the actual end date
    // This can happen right after purchase before the end date syncs
    LicenseStatus {
        is_valid: true,
        is_activated: true,
        plan_type: Some(license.plan_type.clone()),
        days_remaining: Some(30), // Assume ~30 days for monthly, will refresh
        in_grace_period: false,
        error: None,
        requires_activation: false,
    }
}

/// Check license status - main entry point for the app
/// This implements the full flow described
pub async fn check_license_status() -> LicenseStatus {
    let mac_address = match get_mac_address() {
        Ok(mac) => mac,
        Err(e) => {
            return LicenseStatus {
                error: Some(format!("Failed to get device ID: {}", e)),
                ..Default::default()
            };
        }
    };

    // Step 1: Check if we have a local license file
    if let Ok(encrypted_license) = load_license() {
        // Try to decrypt and validate
        match decrypt_license(&encrypted_license) {
            Ok(license_data) => {
                let status = validate_license_data(&license_data, &mac_address);

                // If subscription expired or in grace period, try to refresh
                if !status.is_valid
                    || status.in_grace_period
                    || (status.plan_type != Some(PlanType::Lifetime)
                        && status.days_remaining.map(|d| d < 7).unwrap_or(false))
                {
                    // Try to refresh from server
                    if let Ok(refreshed) =
                        refresh_license_from_server(&encrypted_license, &mac_address).await
                    {
                        return refreshed;
                    }
                }

                return status;
            }
            Err(_) => {
                // License file corrupted, try to get from server
                let _ = delete_license();
            }
        }
    }

    // Step 2: No local license, check if MAC is already registered
    match lookup_license_by_mac(&mac_address).await {
        Ok(Some(encrypted_license)) => {
            // Save the license locally
            let _ = save_license(&encrypted_license);

            // Validate it
            match decrypt_license(&encrypted_license) {
                Ok(license_data) => validate_license_data(&license_data, &mac_address),
                Err(e) => LicenseStatus {
                    error: Some(format!("Failed to validate license: {}", e)),
                    requires_activation: true,
                    ..Default::default()
                },
            }
        }
        Ok(None) => {
            // No license found for this device - needs activation
            LicenseStatus {
                requires_activation: true,
                ..Default::default()
            }
        }
        Err(e) => LicenseStatus {
            error: Some(format!("Failed to check license: {}", e)),
            requires_activation: true,
            ..Default::default()
        },
    }
}

/// Look up license by MAC address from server
async fn lookup_license_by_mac(mac_address: &str) -> Result<Option<String>, String> {
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/lookup", API_BASE_URL))
        .json(&serde_json::json!({ "macAddress": mac_address }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let data: LookupResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    if data.found {
        Ok(data.license)
    } else {
        Ok(None)
    }
}

/// Refresh license from server to get updated subscription end date
async fn refresh_license_from_server(
    encrypted_license: &str,
    mac_address: &str,
) -> Result<LicenseStatus, String> {
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/refresh", API_BASE_URL))
        .json(&serde_json::json!({
            "license": encrypted_license,
            "macAddress": mac_address
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let data: RefreshResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    if data.success {
        if let Some(new_license) = data.license {
            // Save the new license
            save_license(&new_license)?;

            // Validate the new license
            let license_data = decrypt_license(&new_license)?;
            return Ok(validate_license_data(&license_data, mac_address));
        }
    }

    // If refresh failed but we're told license is still active, it might be network issue
    if data.is_active == Some(false) {
        return Ok(LicenseStatus {
            is_valid: false,
            error: data.error.or(Some("License deactivated".to_string())),
            ..Default::default()
        });
    }

    Err(data.error.unwrap_or("Failed to refresh license".to_string()))
}

/// Activate a new device with a product key
pub async fn activate_with_product_key(
    product_key: &str,
    device_name: Option<&str>,
) -> Result<LicenseStatus, String> {
    let mac_address = get_mac_address()?;
    let client = reqwest::Client::new();

    let mut body = serde_json::json!({
        "productKey": product_key,
        "macAddress": mac_address
    });

    if let Some(name) = device_name {
        body["deviceName"] = serde_json::Value::String(name.to_string());
    }

    let response = client
        .post(format!("{}/validate", API_BASE_URL))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let status_code = response.status();
    let data: ValidateResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    if data.success {
        if let Some(license) = data.license {
            // Save the license locally
            save_license(&license)?;

            // Validate and return status
            let license_data = decrypt_license(&license)?;
            return Ok(validate_license_data(&license_data, &mac_address));
        }
    }

    Err(data.error.unwrap_or_else(|| {
        if status_code.is_client_error() {
            "Invalid product key".to_string()
        } else {
            "Activation failed".to_string()
        }
    }))
}

/// Deactivate this device
pub async fn deactivate_device() -> Result<(), String> {
    let mac_address = get_mac_address()?;
    let encrypted_license = load_license()?;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/deactivate", API_BASE_URL))
        .json(&serde_json::json!({
            "license": encrypted_license,
            "macAddress": mac_address
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    if data["success"].as_bool() == Some(true) {
        // Delete local license
        delete_license()?;
        Ok(())
    } else {
        Err(data["error"]
            .as_str()
            .unwrap_or("Deactivation failed")
            .to_string())
    }
}
