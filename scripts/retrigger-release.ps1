#!/usr/bin/env pwsh
# Script to delete and recreate a release tag to trigger GitHub Actions

param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

# Validate version format (x.y.z or vx.y.z)
$CleanVersion = $Version -replace '^v', ''
if ($CleanVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "Error: Version must be in format x.y.z or vx.y.z (e.g., 0.1.5 or v0.1.5)" -ForegroundColor Red
    exit 1
}

$TagName = "v$CleanVersion"

Write-Host "Re-triggering release for $TagName..." -ForegroundColor Cyan
Write-Host ""

# Check if we're in a git repository
if (-not (Test-Path ".git")) {
    Write-Host "Error: Not in a git repository" -ForegroundColor Red
    exit 1
}

# Check if there are uncommitted changes
$status = git status --porcelain
if ($status) {
    Write-Host "Warning: You have uncommitted changes:" -ForegroundColor Yellow
    Write-Host $status
    Write-Host ""
    $response = Read-Host "Continue anyway? (y/n)"
    if ($response -ne 'y') {
        Write-Host "Aborted." -ForegroundColor Yellow
        exit 0
    }
}

Write-Host "Step 1: Deleting local tag..." -ForegroundColor Cyan
git tag -d $TagName 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Deleted local tag" -ForegroundColor Green
}

Write-Host ""
Write-Host "Step 2: Deleting remote tag..." -ForegroundColor Cyan
git push origin --delete $TagName 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Deleted remote tag" -ForegroundColor Green
} else {
    Write-Host "Note: Remote tag may not exist (this is okay)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Step 3: Creating new tag..." -ForegroundColor Cyan
git tag $TagName
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Created local tag" -ForegroundColor Green
} else {
    Write-Host "✗ Failed to create tag" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Step 4: Pushing tag to trigger release..." -ForegroundColor Cyan
git push origin $TagName
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Pushed tag to origin" -ForegroundColor Green
} else {
    Write-Host "✗ Failed to push tag" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Release re-triggered successfully! 🎉" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Check GitHub Actions: https://github.com/$((git remote get-url origin) -replace '.*github.com[:/](.*)\.git', '$1')/actions"
Write-Host "  2. Wait for builds to complete (~10-15 minutes)"
Write-Host "  3. Check release page: https://github.com/$((git remote get-url origin) -replace '.*github.com[:/](.*)\.git', '$1')/releases/tag/$TagName"

