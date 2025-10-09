# GitHub Actions Workflows

This directory contains automated CI/CD workflows for ConvertSave.

## Workflows

### `release.yml` - Build and Release

**Triggers:**

- Push to `main` branch → Builds all platforms, uploads artifacts
- Push tags matching `v*` → Builds all platforms, creates GitHub release
- Pull requests → Builds all platforms for testing
- Manual dispatch → Can be triggered manually from GitHub Actions UI

**What it does:**

1. **Build Job** (Runs on all platforms in parallel):

   - **Windows**: Builds `.msi` and `.exe` installers
   - **macOS**: Builds `.dmg` for both Intel and Apple Silicon
   - **Linux**: Builds `.deb`, `.rpm`, and `.AppImage` packages
   - Uploads artifacts for each platform

2. **Release Job** (Only runs on version tags):
   - Downloads all build artifacts
   - Organizes them into a release-assets directory
   - Creates a GitHub release with all installers
   - Generates release notes automatically

**Build Outputs:**

| Platform    | Files                                                                                                   |
| ----------- | ------------------------------------------------------------------------------------------------------- |
| Windows     | `ConvertSave_X.X.X_x64-setup.exe` (NSIS)<br>`ConvertSave_X.X.X_x64_en-US.msi`                           |
| macOS Intel | `ConvertSave_X.X.X_x64.dmg`<br>`ConvertSave.app` (in .dmg)                                              |
| macOS ARM   | `ConvertSave_X.X.X_aarch64.dmg`<br>`ConvertSave.app` (in .dmg)                                          |
| Linux       | `convertsave_X.X.X_amd64.AppImage`<br>`convertsave_X.X.X_amd64.deb`<br>`convertsave-X.X.X-1.x86_64.rpm` |

## How to Create a Release

1. **Update version** in:

   - `package.json`
   - `src-tauri/tauri.conf.json`
   - `src-tauri/Cargo.toml`

2. **Commit and tag:**

   ```bash
   git add .
   git commit -m "Release v1.0.0"
   git tag v1.0.0
   git push origin main
   git push origin v1.0.0
   ```

3. **Wait for builds:**

   - Check Actions tab on GitHub
   - All platforms build in parallel (~10-20 minutes)
   - Release is created automatically when all succeed

4. **Release is published:**
   - All installers are attached to the release
   - Release notes are generated automatically
   - Users can download for their platform

## Build Times

Expected build times (with cache):

- **Windows**: 8-12 minutes
- **macOS**: 15-20 minutes (builds both architectures)
- **Linux**: 8-12 minutes

First build may take longer as dependencies are cached.

## Artifacts

Even on non-tag builds (main branch, PRs), artifacts are uploaded and available for 90 days:

- Download from the Actions run page
- Useful for testing builds before release
- Each platform uploads separately

## Concurrency

The workflow uses concurrency groups to:

- Cancel in-progress builds when new commits are pushed
- Prevent multiple simultaneous builds of the same ref
- Save CI/CD minutes

## Permissions

Required permissions:

- `contents: write` - Create releases and upload assets
- `packages: write` - Future use for package registries

## Troubleshooting

### Build Fails

Check the logs in Actions tab:

- **Ubuntu**: Usually dependency issues, check apt-get step
- **macOS**: May need Xcode CLI tools or Rust targets
- **Windows**: Check Node/Rust installation

### Release Not Created

- Ensure tag starts with `v` (e.g., `v1.0.0`)
- Check that all platform builds succeeded
- Release job only runs on successful builds

### Artifacts Not Found

If you see "artifact not found" in release job:

- Check that build jobs completed successfully
- Verify artifact upload paths match download expectations
- Check artifact names are correct

## File Locations

Build outputs are in:

```
src-tauri/target/
├── release/bundle/              # Linux builds
│   ├── appimage/*.AppImage
│   ├── deb/*.deb
│   └── rpm/*.rpm
├── x86_64-apple-darwin/release/bundle/  # macOS Intel
│   ├── dmg/*.dmg
│   └── macos/*.app
├── aarch64-apple-darwin/release/bundle/ # macOS ARM
│   ├── dmg/*.dmg
│   └── macos/*.app
└── release/bundle/              # Windows builds
    ├── msi/*.msi
    └── nsis/*.exe
```

## Additional Documentation

- [`RELEASE.md`](./RELEASE.md) - Detailed release process
- [`SIGNING.md`](./SIGNING.md) - Code signing setup

## Workflow Diagram

```
Push to main ──┐
Tag vX.X.X ────┼──> Build Job ──┐
Pull Request ──┘    (3 platforms)│
                                  │
                    ┌─────────────┴─────────────┐
                    │                           │
                    ▼                           ▼
              Upload Artifacts          Release Job
              (90 day retention)    (tags only)
                                          │
                                          ▼
                                   Create GitHub
                                      Release
```

---

For questions or issues with CI/CD, check the Actions logs or open an issue.
