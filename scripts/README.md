# Scripts

This directory contains utility scripts for the ConvertSave project.

## Version Update Script

Automatically updates the version number across all project files.

### Files Updated:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.lock` (auto-generated)

### Usage:

#### Windows (PowerShell):

```powershell
# Using npm script (recommended)
npm run version:update 0.1.6

# Or directly
pwsh ./scripts/update-version.ps1 0.1.6
```

#### macOS/Linux (Bash):

```bash
# Make script executable (first time only)
chmod +x ./scripts/update-version.sh

# Run the script
./scripts/update-version.sh 0.1.6
```

### Version Format:

Version must follow semantic versioning: `MAJOR.MINOR.PATCH`

- Example: `0.1.6`, `1.0.0`, `2.3.4`

### After Running:

The script will show you the next steps:

1. Review changes: `git diff`
2. Commit changes: `git add . && git commit -m 'chore: bump version to v0.1.6'`
3. Create tag: `git tag v0.1.6`
4. Push: `git push && git push --tags`

## Development Scripts

### tauri-dev.sh

Development script for running Tauri in dev mode.

### tauri-dev-clean.sh

Development script for running Tauri in dev mode with a clean build.
