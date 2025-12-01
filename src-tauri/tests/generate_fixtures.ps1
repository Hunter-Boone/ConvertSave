# ConvertSave Test Fixture Generator
# This script generates test fixtures in various formats using FFmpeg and ImageMagick
# Run this script to populate the fixtures directory before running integration tests

$ErrorActionPreference = "Continue"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$fixturesDir = Join-Path $scriptDir "fixtures"
$imagesDir = Join-Path $fixturesDir "images"
$videoDir = Join-Path $fixturesDir "video"
$audioDir = Join-Path $fixturesDir "audio"
$documentsDir = Join-Path $fixturesDir "documents"

# Create directories
New-Item -ItemType Directory -Force -Path $imagesDir | Out-Null
New-Item -ItemType Directory -Force -Path $videoDir | Out-Null
New-Item -ItemType Directory -Force -Path $audioDir | Out-Null
New-Item -ItemType Directory -Force -Path $documentsDir | Out-Null

Write-Host "=== ConvertSave Test Fixture Generator ===" -ForegroundColor Cyan
Write-Host ""

# Check for required tools
$hasFFmpeg = $null -ne (Get-Command "ffmpeg" -ErrorAction SilentlyContinue)
$hasImageMagick = $null -ne (Get-Command "magick" -ErrorAction SilentlyContinue)

if (-not $hasFFmpeg) {
    Write-Host "WARNING: FFmpeg not found. Video/audio fixtures will be skipped." -ForegroundColor Yellow
}
if (-not $hasImageMagick) {
    Write-Host "WARNING: ImageMagick not found. Some image fixtures will be skipped." -ForegroundColor Yellow
}

Write-Host ""

# ============================================
# Generate Base Image (if not exists)
# ============================================

$basePng = Join-Path $imagesDir "sample.png"
if (-not (Test-Path $basePng)) {
    Write-Host "Creating base sample.png..." -ForegroundColor Green
    if ($hasImageMagick) {
        # Create a colorful test image with gradients and shapes
        & magick -size 800x600 `
            -seed 42 plasma:blue-purple `
            -fill white -font Arial -pointsize 48 -gravity center `
            -annotate 0 "ConvertSave Test" `
            -fill none -stroke white -strokewidth 3 `
            -draw "rectangle 100,100 700,500" `
            -draw "circle 400,300 400,150" `
            $basePng
        Write-Host "  Created: sample.png" -ForegroundColor Gray
    } else {
        Write-Host "  SKIP: Cannot create sample.png without ImageMagick" -ForegroundColor Yellow
    }
}

# ============================================
# Generate Image Fixtures
# ============================================

Write-Host ""
Write-Host "Generating image fixtures..." -ForegroundColor Cyan

if ((Test-Path $basePng) -and $hasImageMagick) {
    $imageFormats = @{
        # Common formats
        "sample.jpg" = @("-quality", "90")
        "sample.jpeg" = @("-quality", "90")
        "sample.gif" = @()
        "sample.bmp" = @()
        "sample.tiff" = @()
        "sample.webp" = @("-quality", "85")
        
        # Professional formats
        "sample.tga" = @()
        "sample.ppm" = @()
        "sample.pgm" = @("-colorspace", "gray")
        "sample.pbm" = @("-colorspace", "gray", "-threshold", "50%")
        
        # Legacy formats
        "sample.pcx" = @()
        "sample.ico" = @("-resize", "256x256")
        
        # Special test images
        "tiny.png" = @("-resize", "16x16")
        "large_4k.png" = @("-resize", "3840x2160")
        "transparent.png" = @("-alpha", "on", "-channel", "A", "-evaluate", "set", "50%")
        "grayscale.png" = @("-colorspace", "gray")
        "small_100x100.png" = @("-resize", "100x100")
        "wide_1920x200.png" = @("-resize", "1920x200!")
        "tall_200x1920.png" = @("-resize", "200x1920!")
    }

    foreach ($format in $imageFormats.Keys) {
        $output = Join-Path $imagesDir $format
        if (-not (Test-Path $output)) {
            $args = @($basePng) + $imageFormats[$format] + @($output)
            & magick @args 2>$null
            if ($LASTEXITCODE -eq 0) {
                Write-Host "  Created: $format" -ForegroundColor Gray
            } else {
                Write-Host "  FAILED: $format" -ForegroundColor Red
            }
        } else {
            Write-Host "  EXISTS: $format" -ForegroundColor DarkGray
        }
    }

    # Create animated GIF
    $animatedGif = Join-Path $imagesDir "animated.gif"
    if (-not (Test-Path $animatedGif)) {
        Write-Host "  Creating animated.gif..." -ForegroundColor Gray
        # Create 5 frames with different colors
        $tempFrames = @()
        for ($i = 0; $i -lt 5; $i++) {
            $tempFrame = Join-Path $env:TEMP "frame_$i.png"
            $hue = $i * 72  # Rotate hue by 72 degrees each frame
            & magick $basePng -modulate 100,100,$hue $tempFrame 2>$null
            $tempFrames += $tempFrame
        }
        & magick -delay 50 -loop 0 @tempFrames $animatedGif 2>$null
        # Cleanup temp frames
        foreach ($frame in $tempFrames) {
            Remove-Item $frame -ErrorAction SilentlyContinue
        }
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  Created: animated.gif" -ForegroundColor Gray
        }
    }
}

# ============================================
# Generate Video Fixtures
# ============================================

Write-Host ""
Write-Host "Generating video fixtures..." -ForegroundColor Cyan

$baseVideo = Join-Path $videoDir "sample.mp4"

if ($hasFFmpeg) {
    # Create base video if not exists (5 second test pattern)
    if (-not (Test-Path $baseVideo)) {
        Write-Host "  Creating base sample.mp4..." -ForegroundColor Green
        & ffmpeg -y -f lavfi -i "testsrc=duration=5:size=640x480:rate=30" `
            -f lavfi -i "sine=frequency=440:duration=5" `
            -c:v libx264 -preset ultrafast -crf 23 `
            -c:a aac -b:a 128k `
            -pix_fmt yuv420p `
            $baseVideo 2>$null
        Write-Host "  Created: sample.mp4" -ForegroundColor Gray
    }

    if (Test-Path $baseVideo) {
        $videoFormats = @{
            "sample.mov" = @("-c:v", "libx264", "-c:a", "aac")
            "sample.avi" = @("-c:v", "mpeg4", "-c:a", "mp3")
            "sample.mkv" = @("-c:v", "copy", "-c:a", "copy")
            "sample.webm" = @("-c:v", "libvpx-vp9", "-crf", "30", "-b:v", "0", "-c:a", "libopus")
            "sample_short.mp4" = @("-t", "2", "-c:v", "copy", "-c:a", "copy")
        }

        foreach ($format in $videoFormats.Keys) {
            $output = Join-Path $videoDir $format
            if (-not (Test-Path $output)) {
                $args = @("-y", "-i", $baseVideo) + $videoFormats[$format] + @($output)
                & ffmpeg @args 2>$null
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "  Created: $format" -ForegroundColor Gray
                } else {
                    Write-Host "  FAILED: $format" -ForegroundColor Red
                }
            } else {
                Write-Host "  EXISTS: $format" -ForegroundColor DarkGray
            }
        }
    }
}

# ============================================
# Generate Audio Fixtures
# ============================================

Write-Host ""
Write-Host "Generating audio fixtures..." -ForegroundColor Cyan

if ($hasFFmpeg) {
    $baseAudio = Join-Path $audioDir "sample.mp3"
    
    # Create base audio (5 second tone)
    if (-not (Test-Path $baseAudio)) {
        Write-Host "  Creating base sample.mp3..." -ForegroundColor Green
        & ffmpeg -y -f lavfi -i "sine=frequency=440:duration=5" `
            -c:a libmp3lame -b:a 192k `
            $baseAudio 2>$null
        Write-Host "  Created: sample.mp3" -ForegroundColor Gray
    }

    if (Test-Path $baseAudio) {
        $audioFormats = @{
            "sample.wav" = @("-c:a", "pcm_s16le")
            "sample.flac" = @("-c:a", "flac")
            "sample.ogg" = @("-c:a", "libvorbis", "-q:a", "5")
            "sample.m4a" = @("-c:a", "aac", "-b:a", "192k")
            "sample.aac" = @("-c:a", "aac", "-b:a", "192k")
            "sample_stereo.mp3" = @("-ac", "2", "-c:a", "libmp3lame")
            "sample_mono.mp3" = @("-ac", "1", "-c:a", "libmp3lame")
        }

        foreach ($format in $audioFormats.Keys) {
            $output = Join-Path $audioDir $format
            if (-not (Test-Path $output)) {
                $args = @("-y", "-i", $baseAudio) + $audioFormats[$format] + @($output)
                & ffmpeg @args 2>$null
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "  Created: $format" -ForegroundColor Gray
                } else {
                    Write-Host "  FAILED: $format" -ForegroundColor Red
                }
            } else {
                Write-Host "  EXISTS: $format" -ForegroundColor DarkGray
            }
        }
    }
}

# ============================================
# Generate Document Fixtures
# ============================================

Write-Host ""
Write-Host "Generating document fixtures..." -ForegroundColor Cyan

# Create sample markdown
$sampleMd = Join-Path $documentsDir "sample.md"
if (-not (Test-Path $sampleMd)) {
    @"
# ConvertSave Test Document

This is a **sample markdown** document for testing file conversions.

## Features

- Bullet point 1
- Bullet point 2
- Bullet point 3

## Code Example

``````rust
fn main() {
    println!("Hello, ConvertSave!");
}
``````

## Table

| Format | Tool | Status |
|--------|------|--------|
| PNG | ImageMagick | ✓ |
| MP4 | FFmpeg | ✓ |
| PDF | LibreOffice | ✓ |

---

*Generated for ConvertSave testing*
"@ | Out-File -FilePath $sampleMd -Encoding utf8
    Write-Host "  Created: sample.md" -ForegroundColor Gray
}

# Create sample HTML
$sampleHtml = Join-Path $documentsDir "sample.html"
if (-not (Test-Path $sampleHtml)) {
    @"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ConvertSave Test Document</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
        h1 { color: #3562e3; }
        code { background: #f4f1ed; padding: 2px 6px; border-radius: 4px; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background: #3562e3; color: white; }
    </style>
</head>
<body>
    <h1>ConvertSave Test Document</h1>
    <p>This is a <strong>sample HTML</strong> document for testing file conversions.</p>
    
    <h2>Features</h2>
    <ul>
        <li>Bullet point 1</li>
        <li>Bullet point 2</li>
        <li>Bullet point 3</li>
    </ul>
    
    <h2>Code Example</h2>
    <pre><code>fn main() {
    println!("Hello, ConvertSave!");
}</code></pre>
    
    <h2>Table</h2>
    <table>
        <tr><th>Format</th><th>Tool</th><th>Status</th></tr>
        <tr><td>PNG</td><td>ImageMagick</td><td>✓</td></tr>
        <tr><td>MP4</td><td>FFmpeg</td><td>✓</td></tr>
        <tr><td>PDF</td><td>LibreOffice</td><td>✓</td></tr>
    </table>
    
    <hr>
    <p><em>Generated for ConvertSave testing</em></p>
</body>
</html>
"@ | Out-File -FilePath $sampleHtml -Encoding utf8
    Write-Host "  Created: sample.html" -ForegroundColor Gray
}

# Create sample plain text
$sampleTxt = Join-Path $documentsDir "sample.txt"
if (-not (Test-Path $sampleTxt)) {
    @"
ConvertSave Test Document
=========================

This is a sample plain text document for testing file conversions.

Features:
- Bullet point 1
- Bullet point 2
- Bullet point 3

Code Example:
-------------
fn main() {
    println!("Hello, ConvertSave!");
}

Table:
------
Format      | Tool        | Status
------------|-------------|--------
PNG         | ImageMagick | OK
MP4         | FFmpeg      | OK
PDF         | LibreOffice | OK

---
Generated for ConvertSave testing
"@ | Out-File -FilePath $sampleTxt -Encoding utf8
    Write-Host "  Created: sample.txt" -ForegroundColor Gray
}

# Create sample RTF
$sampleRtf = Join-Path $documentsDir "sample.rtf"
if (-not (Test-Path $sampleRtf)) {
    @"
{\rtf1\ansi\deff0
{\fonttbl{\f0 Arial;}}
{\colortbl;\red53\green98\blue227;}
\f0\fs24
\cf1\b ConvertSave Test Document\b0\cf0\par
\par
This is a \b sample RTF\b0  document for testing file conversions.\par
\par
\b Features:\b0\par
\bullet  Bullet point 1\par
\bullet  Bullet point 2\par
\bullet  Bullet point 3\par
\par
\i Generated for ConvertSave testing\i0\par
}
"@ | Out-File -FilePath $sampleRtf -Encoding ascii
    Write-Host "  Created: sample.rtf" -ForegroundColor Gray
}

# ============================================
# Summary
# ============================================

Write-Host ""
Write-Host "=== Fixture Generation Complete ===" -ForegroundColor Cyan
Write-Host ""

# Count files
$imageCount = (Get-ChildItem $imagesDir -File -ErrorAction SilentlyContinue | Measure-Object).Count
$videoCount = (Get-ChildItem $videoDir -File -ErrorAction SilentlyContinue | Measure-Object).Count
$audioCount = (Get-ChildItem $audioDir -File -ErrorAction SilentlyContinue | Measure-Object).Count
$docCount = (Get-ChildItem $documentsDir -File -ErrorAction SilentlyContinue | Measure-Object).Count

Write-Host "Fixtures created:" -ForegroundColor Green
Write-Host "  Images:    $imageCount files" -ForegroundColor Gray
Write-Host "  Videos:    $videoCount files" -ForegroundColor Gray
Write-Host "  Audio:     $audioCount files" -ForegroundColor Gray
Write-Host "  Documents: $docCount files" -ForegroundColor Gray
Write-Host ""
Write-Host "Total: $($imageCount + $videoCount + $audioCount + $docCount) fixture files" -ForegroundColor Green
Write-Host ""
Write-Host "Run tests with: npm test" -ForegroundColor Cyan


