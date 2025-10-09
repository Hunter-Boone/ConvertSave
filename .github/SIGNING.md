# Code Signing Guide

This guide explains how to set up code signing for ConvertSave releases.

## Why Code Signing?

### macOS
- **Required** for distribution outside the Mac App Store
- Prevents "App is damaged and can't be opened" errors
- Allows app to pass Gatekeeper without user intervention
- Enables notarization for better user experience

### Windows
- **Optional** but highly recommended
- Prevents SmartScreen warnings
- Shows verified publisher name
- Increases user trust

### Linux
- Not required
- No standard code signing mechanism

## macOS Code Signing

### Prerequisites

1. **Apple Developer Account** ($99/year)
   - Sign up at https://developer.apple.com/
   - Enroll in the Developer Program

2. **Developer ID Certificate**
   - Log in to https://developer.apple.com/account
   - Go to Certificates, Identifiers & Profiles
   - Create a "Developer ID Application" certificate
   - Download the certificate

### Setting Up Certificates

#### 1. Export Certificate from Keychain

On your Mac:

```bash
# Open Keychain Access
# Find your "Developer ID Application" certificate
# Right-click → Export "Developer ID Application..."
# Save as "certificate.p12"
# Set a strong password
```

#### 2. Convert to Base64

```bash
base64 -i certificate.p12 -o certificate-base64.txt
```

#### 3. Create App-Specific Password

1. Go to https://appleid.apple.com/
2. Sign in with your Apple ID
3. Security → App-Specific Passwords
4. Generate new password (save it securely)

#### 4. Find Your Team ID

1. Go to https://developer.apple.com/account
2. Membership → Team ID (10-character string)

#### 5. Find Your Signing Identity

```bash
security find-identity -v -p codesigning
```

Look for something like: `Developer ID Application: Your Name (TEAMID)`

### Adding Secrets to GitHub

Go to your GitHub repository → Settings → Secrets and variables → Actions → New repository secret

Add these secrets:

| Secret Name | Value |
|------------|-------|
| `APPLE_CERTIFICATE` | Contents of `certificate-base64.txt` |
| `APPLE_CERTIFICATE_PASSWORD` | Password you set for the P12 file |
| `APPLE_SIGNING_IDENTITY` | e.g., `Developer ID Application: Your Name (TEAMID)` |
| `APPLE_ID` | Your Apple ID email |
| `APPLE_PASSWORD` | App-specific password you generated |
| `APPLE_TEAM_ID` | Your 10-character team ID |

### Update GitHub Workflow

Update `.github/workflows/release.yml` to include signing:

```yaml
- uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
    APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
    APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
    APPLE_ID: ${{ secrets.APPLE_ID }}
    APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
    APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
  with:
    tagName: ${{ github.ref_name }}
    releaseName: 'ConvertSave v__VERSION__'
    releaseBody: 'See the assets to download this version and install.'
    releaseDraft: true
    prerelease: false
    args: ${{ matrix.args }}
```

### Configure Tauri for Signing

Update `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "macOS": {
      "signing": {
        "identity": "Developer ID Application: Your Name (TEAMID)",
        "entitlements": "entitlements.plist"
      }
    }
  }
}
```

### Create Entitlements File

Create `src-tauri/entitlements.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>com.apple.security.cs.allow-jit</key>
  <true/>
  <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
  <true/>
  <key>com.apple.security.cs.disable-library-validation</key>
  <true/>
  <key>com.apple.security.network.client</key>
  <true/>
  <key>com.apple.security.files.downloads.read-write</key>
  <true/>
  <key>com.apple.security.files.user-selected.read-write</key>
  <true/>
</dict>
</plist>
```

### Notarization

Notarization happens automatically when you provide the Apple credentials. The workflow will:

1. Build the app
2. Sign it with your certificate
3. Create a DMG
4. Upload to Apple for notarization
5. Wait for approval
6. Staple the notarization ticket
7. Include in release

This can add 5-15 minutes to the build time but is worth it for a smooth user experience.

## Windows Code Signing

### Prerequisites

1. **Code Signing Certificate**
   - Purchase from a Certificate Authority (CA)
   - Options: DigiCert, Sectigo, GlobalSign
   - Cost: $200-400/year
   - Choose "Code Signing Certificate" for Windows

### Setting Up Certificate

#### 1. Export Certificate

After receiving your certificate:

1. Install it in Windows Certificate Store
2. Open `certmgr.msc`
3. Find your certificate under Personal → Certificates
4. Right-click → All Tasks → Export
5. Choose "Yes, export the private key"
6. Select PFX format
7. Set a strong password
8. Save as `certificate.pfx`

#### 2. Convert to Base64

PowerShell:
```powershell
certutil -encode certificate.pfx certificate-base64.txt
```

Or use online tools (ensure they're trustworthy).

#### 3. Add Secrets to GitHub

| Secret Name | Value |
|------------|-------|
| `WINDOWS_CERTIFICATE` | Contents of `certificate-base64.txt` |
| `WINDOWS_CERTIFICATE_PASSWORD` | Password for PFX file |

### Update GitHub Workflow

```yaml
- uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    WINDOWS_CERTIFICATE: ${{ secrets.WINDOWS_CERTIFICATE }}
    WINDOWS_CERTIFICATE_PASSWORD: ${{ secrets.WINDOWS_CERTIFICATE_PASSWORD }}
```

### Configure Tauri for Windows Signing

Update `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

## Testing Signed Apps

### macOS

```bash
# Check if app is signed
codesign -vv --deep --strict /path/to/ConvertSave.app

# Check notarization
spctl -a -vv /path/to/ConvertSave.app

# Expected output:
# /path/to/ConvertSave.app: accepted
# source=Notarized Developer ID
```

### Windows

```powershell
# Check signature
Get-AuthenticodeSignature .\ConvertSave.exe

# Should show:
# Status: Valid
# SignerCertificate: Your certificate info
```

## Troubleshooting

### macOS: "App is damaged and can't be opened"

This means signing or notarization failed. Check:

1. Certificate is valid and not expired
2. All secrets are correctly set in GitHub
3. Signing identity matches exactly
4. Entitlements file exists and is valid

To bypass locally for testing:
```bash
xattr -cr /path/to/ConvertSave.app
```

### macOS: Notarization Fails

Common causes:
- Wrong Apple ID or app-specific password
- Certificate expired
- Missing entitlements
- Hardened runtime issues

Check logs in GitHub Actions output.

### Windows: SmartScreen Still Shows Warning

- New certificates take time to build reputation
- More downloads = better reputation
- Consider Extended Validation (EV) certificate for immediate trust

### Linux: No Signing Needed

Linux doesn't require code signing. Focus on:
- Providing clear checksums (SHA256)
- Publishing to reputable repositories
- Using verified distribution channels

## Cost Summary

| Platform | Requirement | Annual Cost |
|----------|-------------|-------------|
| macOS | Developer Account + Certificate | $99 |
| Windows | Code Signing Certificate | $200-400 |
| Linux | None | $0 |

**Total**: $300-500/year for full cross-platform signing

## Alternatives

### Without Code Signing

You can still release without signing:

**macOS**: 
- Users get warning, can bypass with right-click → Open
- App works fine but less trustworthy

**Windows**: 
- SmartScreen warning
- Users can click "More info" → "Run anyway"

**Linux**: 
- No impact, works normally

### Budget-Friendly Options

1. **Start with one platform**: Sign macOS first (most strict)
2. **Build reputation**: Let Windows SmartScreen reputation build over time
3. **Open source advantage**: Show code is open for inspection

## Security Best Practices

1. **Never commit certificates** to version control
2. **Use strong passwords** for certificate files
3. **Rotate secrets** regularly
4. **Limit access** to GitHub secrets
5. **Enable 2FA** on Apple ID and GitHub
6. **Monitor certificate expiration** dates
7. **Keep backup** of certificates securely

## Getting Help

- **macOS Signing**: https://tauri.app/v1/guides/distribution/sign-macos
- **Windows Signing**: https://tauri.app/v1/guides/distribution/sign-windows
- **Tauri Discord**: https://discord.gg/tauri
- **Apple Developer Forums**: https://developer.apple.com/forums/

---

**Note**: Code signing setup is complex but only needs to be done once per platform. Future releases will automatically use the configured signing.

