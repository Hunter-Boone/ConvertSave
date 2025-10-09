# Release Process

This document describes how to create releases for ConvertSave using GitHub Actions.

## Prerequisites

Before creating releases, ensure you have:

1. **GitHub Repository** with the code pushed
2. **GitHub Secrets** configured (for code signing on macOS/Windows - optional but recommended)
3. **Version number** updated in `src-tauri/tauri.conf.json` and `package.json`

## Creating a Release

### Automatic Release (Recommended)

1. **Update Version Numbers**
   
   Update the version in both files:
   - `src-tauri/tauri.conf.json` → `version` field
   - `package.json` → `version` field
   
   Example:
   ```json
   "version": "1.0.0"
   ```

2. **Commit and Tag**
   
   ```bash
   git add .
   git commit -m "Release v1.0.0"
   git tag v1.0.0
   git push origin main
   git push origin v1.0.0
   ```

3. **Wait for Build**
   
   - GitHub Actions will automatically build for all platforms:
     - Windows (x64) - `.exe` installer and `.msi`
     - macOS (Intel) - `.dmg` and `.app.tar.gz`
     - macOS (Apple Silicon) - `.dmg` and `.app.tar.gz`
     - Linux - `.AppImage`, `.deb`, and `.rpm`
   
   - Check progress at: `https://github.com/YOUR_USERNAME/ConvertSave/actions`

4. **Review and Publish**
   
   - Once complete, a draft release will be created
   - Go to the Releases page
   - Review the release notes
   - Click "Publish release"

### Manual Release

You can also trigger releases manually:

1. Go to Actions tab in GitHub
2. Select "Release" workflow
3. Click "Run workflow"
4. Select the branch and run

## Platform-Specific Notes

### Windows

**Artifacts Generated:**
- `ConvertSave_X.X.X_x64-setup.exe` - NSIS installer
- `ConvertSave_X.X.X_x64_en-US.msi` - MSI installer
- `ConvertSave_X.X.X_x64-setup.exe.sig` - Signature file (if signed)

**Code Signing (Optional):**

To sign Windows builds, add these secrets to your GitHub repository:

- `WINDOWS_CERTIFICATE` - Base64 encoded PFX certificate
- `WINDOWS_CERTIFICATE_PASSWORD` - Certificate password

To create the base64 certificate:
```powershell
certutil -encode certificate.pfx certificate.txt
```

### macOS

**Artifacts Generated:**
- `ConvertSave_X.X.X_aarch64.dmg` - ARM64 (M1/M2/M3) installer
- `ConvertSave_X.X.X_x64.dmg` - Intel installer
- `ConvertSave_X.X.X_aarch64.app.tar.gz` - ARM64 app bundle
- `ConvertSave_X.X.X_x64.app.tar.gz` - Intel app bundle

**Code Signing (Optional but Recommended):**

For macOS distribution, you should sign your app. Add these secrets:

- `APPLE_CERTIFICATE` - Base64 encoded P12 certificate
- `APPLE_CERTIFICATE_PASSWORD` - Certificate password
- `APPLE_SIGNING_IDENTITY` - Your Developer ID Application identity
- `APPLE_ID` - Your Apple ID email
- `APPLE_PASSWORD` - App-specific password
- `APPLE_TEAM_ID` - Your team ID

See `.github/workflows/SIGNING.md` for detailed setup instructions.

### Linux

**Artifacts Generated:**
- `convertsave_X.X.X_amd64.AppImage` - Universal Linux binary
- `convertsave_X.X.X_amd64.deb` - Debian/Ubuntu package
- `convertsave-X.X.X-1.x86_64.rpm` - Fedora/RHEL package

**No code signing required** for Linux builds.

## Release Assets

After a successful release, users will be able to download:

1. **Windows users**: `.exe` or `.msi` installer
2. **macOS users**: `.dmg` installer (choose Intel or ARM based on chip)
3. **Linux users**: 
   - `.AppImage` - Works on most distros
   - `.deb` - For Debian/Ubuntu
   - `.rpm` - For Fedora/RHEL

## Important Notes

### 1. Tools Are NOT Bundled

ConvertSave does **not** bundle FFmpeg or Pandoc binaries. Instead:
- Users download these tools on first launch
- This keeps the installer size small (~5-10 MB vs ~100+ MB)
- Ensures GPL license compliance
- Downloads from official sources

### 2. File Size Expectations

Expected installer sizes:
- **Windows**: 5-8 MB
- **macOS**: 4-7 MB
- **Linux**: 6-10 MB

### 3. GitHub Release Limits

- GitHub has a file size limit of 2 GB per asset
- Total release size should not exceed 10 GB
- Our releases are well within these limits

### 4. Update Mechanisms

The release workflow sets up the foundation for auto-updates:
- Windows: MSI supports updates
- macOS: DMG and app bundle
- Linux: AppImage supports auto-update

You can implement the Tauri updater plugin later for seamless updates.

## Troubleshooting

### Build Fails on macOS

If you see code signing errors but haven't set up signing:
1. Edit `.github/workflows/release.yml`
2. Add `TAURI_SIGNING_PRIVATE_KEY: ""` to skip signing temporarily

### Build Fails on Windows

- Ensure Node.js and Rust are properly installed in the workflow
- Check that all dependencies are correctly specified

### Build Fails on Linux

- WebKit2GTK errors usually mean the dependencies step failed
- Ensure `libwebkit2gtk-4.1-dev` is installed

### Frontend Build Fails

If the frontend build fails with "dist directory not found":
- Ensure `npm run build` completes successfully
- Check `vite.config.ts` configuration
- Verify output directory is set to `dist`

## Version Management

Keep versions synchronized:

```json
// package.json
{
  "version": "1.0.0"
}

// src-tauri/tauri.conf.json
{
  "version": "1.0.0"
}

// src-tauri/Cargo.toml
[package]
version = "1.0.0"
```

A mismatch may cause build issues.

## Testing Before Release

Before creating a release:

1. Test the app locally: `npm run tauri dev`
2. Build locally: `npm run tauri build`
3. Test the built executable
4. Create a pre-release or draft release first
5. Test downloaded installers on clean systems

## Release Checklist

- [ ] Update version in all files (package.json, tauri.conf.json, Cargo.toml)
- [ ] Update CHANGELOG.md with changes
- [ ] Test app locally
- [ ] Build locally and test
- [ ] Commit changes
- [ ] Create and push tag
- [ ] Wait for CI/CD to complete
- [ ] Download and test all platform installers
- [ ] Update release notes
- [ ] Publish release

## Support

For issues with the release process:
1. Check the Actions logs for detailed error messages
2. Review this document
3. Check Tauri documentation: https://tauri.app/v1/guides/building/
4. Open an issue in the repository

---

**Note**: The first release may take 15-30 minutes as all dependencies are cached. Subsequent releases are much faster (5-10 minutes).

