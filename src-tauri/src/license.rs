//! License validation and decryption module
//! 
//! This module handles local license validation by decrypting the encrypted
//! license blob received from the server. The license contains:
//! - Product key
//! - MAC address (tied to this device)
//! - Plan type (monthly, yearly, lifetime)
//! - Subscription end date (null for lifetime)
//! - Issue timestamp

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use scrypt::{scrypt, Params};
use serde::{Deserialize, Serialize};

/// The encryption key - MUST match LICENSE_ENCRYPTION_KEY from the website
/// Set this via environment variable at compile time: LICENSE_ENCRYPTION_KEY=your-key cargo build
/// Or replace this constant directly (less secure for open source projects)
const ENCRYPTION_KEY: &str = match option_env!("LICENSE_ENCRYPTION_KEY") {
    Some(key) => key,
    None => "PLACEHOLDER_KEY_SET_LICENSE_ENCRYPTION_KEY_ENV_VAR",
};

/// Decrypted license data structure
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

/// License validation result
#[derive(Debug, Clone, Serialize)]
pub struct LicenseStatus {
    pub valid: bool,
    pub plan_type: Option<PlanType>,
    pub days_remaining: Option<i64>,
    pub error: Option<String>,
}

/// Derive the encryption key using scrypt (matching the Node.js implementation)
fn derive_key() -> Result<[u8; 32], String> {
    let params = Params::new(15, 8, 1, 32).map_err(|e| format!("Scrypt params error: {}", e))?;
    let mut key = [0u8; 32];
    scrypt(ENCRYPTION_KEY.as_bytes(), b"salt", &params, &mut key)
        .map_err(|e| format!("Scrypt error: {}", e))?;
    Ok(key)
}

/// Decrypt the license blob
/// 
/// The encrypted format is: base64(iv[16] + authTag[16] + ciphertext)
pub fn decrypt_license(encrypted_license: &str) -> Result<LicenseData, String> {
    // Decode base64
    let data = BASE64
        .decode(encrypted_license)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    if data.len() < 33 {
        return Err("Invalid license data: too short".to_string());
    }

    // Extract components
    let iv = &data[0..16];
    let auth_tag = &data[16..32];
    let ciphertext = &data[32..];

    // Derive the key
    let key = derive_key()?;

    // Create cipher
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| format!("Cipher creation error: {}", e))?;

    // Create nonce from IV
    let nonce = Nonce::from_slice(iv);

    // Combine ciphertext and auth tag for decryption
    let mut combined = ciphertext.to_vec();
    combined.extend_from_slice(auth_tag);

    // Decrypt
    let decrypted = cipher
        .decrypt(nonce, combined.as_ref())
        .map_err(|_| "Decryption failed: invalid license or key".to_string())?;

    // Parse JSON
    let license_data: LicenseData = serde_json::from_slice(&decrypted)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(license_data)
}

/// Validate a license locally
/// 
/// Checks:
/// 1. License can be decrypted
/// 2. MAC address matches current device
/// 3. Subscription hasn't expired (for non-lifetime plans)
pub fn validate_license(encrypted_license: &str, current_mac: &str) -> LicenseStatus {
    // Try to decrypt
    let license_data = match decrypt_license(encrypted_license) {
        Ok(data) => data,
        Err(e) => {
            return LicenseStatus {
                valid: false,
                plan_type: None,
                days_remaining: None,
                error: Some(e),
            };
        }
    };

    // Check MAC address
    if license_data.mac_address != current_mac {
        return LicenseStatus {
            valid: false,
            plan_type: Some(license_data.plan_type),
            days_remaining: None,
            error: Some("License not valid for this device".to_string()),
        };
    }

    // Check subscription expiration
    let days_remaining = match (&license_data.plan_type, &license_data.subscription_end_date) {
        (PlanType::Lifetime, _) => None, // Lifetime never expires
        (_, Some(end_date_str)) => {
            match end_date_str.parse::<DateTime<Utc>>() {
                Ok(end_date) => {
                    let now = Utc::now();
                    let duration = end_date.signed_duration_since(now);
                    let days = duration.num_days();
                    
                    if days < 0 {
                        return LicenseStatus {
                            valid: false,
                            plan_type: Some(license_data.plan_type),
                            days_remaining: Some(days),
                            error: Some("Subscription has expired".to_string()),
                        };
                    }
                    
                    Some(days)
                }
                Err(_) => None,
            }
        }
        (_, None) => None,
    };

    LicenseStatus {
        valid: true,
        plan_type: Some(license_data.plan_type),
        days_remaining,
        error: None,
    }
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
    
    // Parse first MAC address from CSV output
    if let Some(line) = stdout.lines().next() {
        if let Some(mac) = line.split(',').next() {
            let mac = mac.trim_matches('"').replace('-', ":");
            if !mac.is_empty() && mac != "N/A" {
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
    use std::fs;
    
    // Try to read from /sys/class/net/
    let net_dir = "/sys/class/net";
    if let Ok(entries) = fs::read_dir(net_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            // Skip loopback and virtual interfaces
            if name_str == "lo" || name_str.starts_with("veth") || name_str.starts_with("docker") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mac_address() {
        let mac = get_mac_address();
        println!("MAC Address: {:?}", mac);
        assert!(mac.is_ok());
    }
}

