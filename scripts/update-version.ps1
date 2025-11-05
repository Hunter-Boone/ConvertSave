#!/usr/bin/env pwsh
# Script to update version across all project files

param(
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# Validate version format (x.y.z)
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "Error: Version must be in format x.y.z (e.g., 0.1.5)" -ForegroundColor Red
    exit 1
}

$RootDir = Split-Path -Parent $PSScriptRoot
$PackageJson = Join-Path $RootDir "package.json"
$CargoToml = Join-Path $RootDir "src-tauri" "Cargo.toml"
$TauriConf = Join-Path $RootDir "src-tauri" "tauri.conf.json"

Write-Host "Updating version to $NewVersion..." -ForegroundColor Cyan
Write-Host ""

# Function to update version in a file
function Update-Version {
    param($FilePath, $Pattern, $Replacement)
    
    if (Test-Path $FilePath) {
        $content = Get-Content $FilePath -Raw
        $oldContent = $content
        $content = $content -replace $Pattern, $Replacement
        
        if ($content -ne $oldContent) {
            Set-Content -Path $FilePath -Value $content -NoNewline
            Write-Host "✓ Updated: $($FilePath | Split-Path -Leaf)" -ForegroundColor Green
            return $true
        } else {
            Write-Host "✗ No change needed: $($FilePath | Split-Path -Leaf)" -ForegroundColor Yellow
            return $false
        }
    } else {
        Write-Host "✗ File not found: $FilePath" -ForegroundColor Red
        return $false
    }
}

# Update package.json
$updated1 = Update-Version -FilePath $PackageJson `
    -Pattern '("version"\s*:\s*)"[\d\.]+"' `
    -Replacement "`$1`"$NewVersion`""

# Update Cargo.toml
$updated2 = Update-Version -FilePath $CargoToml `
    -Pattern '(version\s*=\s*)"[\d\.]+"' `
    -Replacement "`$1`"$NewVersion`""

# Update tauri.conf.json
$updated3 = Update-Version -FilePath $TauriConf `
    -Pattern '("version"\s*:\s*)"[\d\.]+"' `
    -Replacement "`$1`"$NewVersion`""

Write-Host ""

# Update Cargo.lock if Cargo.toml was updated
if ($updated2) {
    Write-Host "Updating Cargo.lock..." -ForegroundColor Cyan
    Push-Location (Join-Path $RootDir "src-tauri")
    cargo update -p convertsave --quiet
    Pop-Location
    Write-Host "✓ Updated: Cargo.lock" -ForegroundColor Green
}

Write-Host ""
Write-Host "Version update complete! 🎉" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Review changes: git diff"
Write-Host "  2. Commit changes: git add . && git commit -m 'chore: bump version to v$NewVersion'"
Write-Host "  3. Create tag: git tag v$NewVersion"
Write-Host "  4. Push: git push && git push --tags"

