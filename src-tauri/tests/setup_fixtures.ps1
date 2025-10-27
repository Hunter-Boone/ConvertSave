# PowerShell script for generating test fixtures on Windows
# Requires: ImageMagick (magick) and FFmpeg

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$FixturesDir = Join-Path $ScriptDir "fixtures"

Write-Host "ConvertSave Test Fixtures Setup" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Check for ImageMagick
$HaveImageMagick = $false
try {
    $null = Get-Command magick -ErrorAction Stop
    Write-Host "✓ ImageMagick found" -ForegroundColor Green
    $HaveImageMagick = $true
} catch {
    Write-Host "⚠️  ImageMagick not found. Image fixtures will need to be added manually." -ForegroundColor Yellow
    Write-Host "   Install from: https://imagemagick.org/" -ForegroundColor Yellow
}

# Check for FFmpeg
$HaveFFmpeg = $false
try {
    $null = Get-Command ffmpeg -ErrorAction Stop
    Write-Host "✓ FFmpeg found" -ForegroundColor Green
    $HaveFFmpeg = $true
} catch {
    Write-Host "⚠️  FFmpeg not found. Video fixtures will need to be added manually." -ForegroundColor Yellow
    Write-Host "   Install from: https://ffmpeg.org/" -ForegroundColor Yellow
}

Write-Host ""

# Create directories
New-Item -ItemType Directory -Force -Path "$FixturesDir\images" | Out-Null
New-Item -ItemType Directory -Force -Path "$FixturesDir\documents" | Out-Null
New-Item -ItemType Directory -Force -Path "$FixturesDir\video" | Out-Null

# Generate image fixtures
if ($HaveImageMagick) {
    Write-Host "Generating image test fixtures..." -ForegroundColor Cyan
    Push-Location "$FixturesDir\images"
    
    # Basic test images
    Write-Host "  - Creating sample.png (800x600, blue)"
    & magick -size 800x600 xc:blue sample.png
    
    Write-Host "  - Creating sample.jpg (800x600, red)"
    & magick -size 800x600 xc:red sample.jpg
    
    Write-Host "  - Creating sample2.png (800x600, green)"
    & magick -size 800x600 xc:green sample2.png
    
    Write-Host "  - Creating sample3.png (800x600, yellow)"
    & magick -size 800x600 xc:yellow sample3.png
    
    # Transparent image
    Write-Host "  - Creating transparent.png (100x100, transparent)"
    & magick -size 100x100 xc:none transparent.png
    
    # Tiny image
    Write-Host "  - Creating tiny.png (1x1, white)"
    & magick -size 1x1 xc:white tiny.png
    
    # Large 4K image (optional, can be slow)
    Write-Host "  - Creating large_4k.png (3840x2160, blue)"
    & magick -size 3840x2160 xc:blue large_4k.png
    
    # Animated GIF
    Write-Host "  - Creating animated.gif (3 frames)"
    & magick -delay 20 -loop 0 sample.png sample2.png sample3.png animated.gif
    
    Pop-Location
    Write-Host "✓ Image fixtures created" -ForegroundColor Green
} else {
    Write-Host "⊘ Skipping image fixture generation (ImageMagick not available)" -ForegroundColor Yellow
}

Write-Host ""

# Generate video fixtures
if ($HaveFFmpeg) {
    Write-Host "Generating video test fixtures..." -ForegroundColor Cyan
    Push-Location "$FixturesDir\video"
    
    # Create a 5-second test video with color bars and audio tone
    Write-Host "  - Creating sample.mp4 (5 seconds, 640x480)"
    & ffmpeg -f lavfi -i testsrc=duration=5:size=640x480:rate=30 `
             -f lavfi -i sine=frequency=1000:duration=5 `
             -c:v libx264 -pix_fmt yuv420p -c:a aac -b:a 128k `
             -y sample.mp4 2>$null
    
    # Convert to AVI
    Write-Host "  - Creating sample.avi from sample.mp4"
    & ffmpeg -i sample.mp4 -c:v mpeg4 -q:v 5 -c:a mp3 -b:a 128k `
             -y sample.avi 2>$null
    
    # Convert to MOV
    Write-Host "  - Creating sample.mov from sample.mp4"
    & ffmpeg -i sample.mp4 -c:v libx264 -pix_fmt yuv420p -c:a aac -b:a 128k `
             -y sample.mov 2>$null
    
    Pop-Location
    Write-Host "✓ Video fixtures created" -ForegroundColor Green
} else {
    Write-Host "⊘ Skipping video fixture generation (FFmpeg not available)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Test Fixtures Setup Complete!" -ForegroundColor Green
Write-Host "=============================="
Write-Host ""
Write-Host "Fixtures location: $FixturesDir"
Write-Host ""
Write-Host "Document fixtures (MD, HTML, TXT) are already included."
Write-Host ""
Write-Host "Next steps:"
Write-Host "  1. Review the generated fixtures in: $FixturesDir"
Write-Host "  2. (Optional) Replace generated images with real photos for better testing"
Write-Host "  3. Run tests: cd src-tauri; cargo test --test conversion_tests -- --ignored --nocapture"
Write-Host ""
Write-Host "For more information, see: $ScriptDir\README.md"

