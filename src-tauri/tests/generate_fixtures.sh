#!/bin/bash
# ConvertSave Test Fixture Generator
# This script generates test fixtures in various formats using FFmpeg and ImageMagick
# Run this script to populate the fixtures directory before running integration tests

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FIXTURES_DIR="$SCRIPT_DIR/fixtures"
IMAGES_DIR="$FIXTURES_DIR/images"
VIDEO_DIR="$FIXTURES_DIR/video"
AUDIO_DIR="$FIXTURES_DIR/audio"
DOCUMENTS_DIR="$FIXTURES_DIR/documents"

# Create directories
mkdir -p "$IMAGES_DIR" "$VIDEO_DIR" "$AUDIO_DIR" "$DOCUMENTS_DIR"

echo "=== ConvertSave Test Fixture Generator ==="
echo ""

# Check for required tools
HAS_FFMPEG=false
HAS_IMAGEMAGICK=false

if command -v ffmpeg &> /dev/null; then
    HAS_FFMPEG=true
else
    echo "WARNING: FFmpeg not found. Video/audio fixtures will be skipped."
fi

if command -v magick &> /dev/null; then
    HAS_IMAGEMAGICK=true
else
    echo "WARNING: ImageMagick not found. Some image fixtures will be skipped."
fi

echo ""

# ============================================
# Generate Base Image (if not exists)
# ============================================

BASE_PNG="$IMAGES_DIR/sample.png"
if [ ! -f "$BASE_PNG" ]; then
    echo "Creating base sample.png..."
    if [ "$HAS_IMAGEMAGICK" = true ]; then
        # Create a colorful test image with gradients and shapes
        magick -size 800x600 \
            -seed 42 plasma:blue-purple \
            -fill white -font Arial -pointsize 48 -gravity center \
            -annotate 0 "ConvertSave Test" \
            -fill none -stroke white -strokewidth 3 \
            -draw "rectangle 100,100 700,500" \
            -draw "circle 400,300 400,150" \
            "$BASE_PNG" 2>/dev/null || \
        # Fallback to simpler image if plasma fails
        magick -size 800x600 gradient:blue-purple \
            -fill white -pointsize 48 -gravity center \
            -annotate 0 "ConvertSave Test" \
            "$BASE_PNG"
        echo "  Created: sample.png"
    else
        echo "  SKIP: Cannot create sample.png without ImageMagick"
    fi
fi

# ============================================
# Generate Image Fixtures
# ============================================

echo ""
echo "Generating image fixtures..."

if [ -f "$BASE_PNG" ] && [ "$HAS_IMAGEMAGICK" = true ]; then
    # Common formats
    declare -A IMAGE_FORMATS=(
        ["sample.jpg"]="-quality 90"
        ["sample.jpeg"]="-quality 90"
        ["sample.gif"]=""
        ["sample.bmp"]=""
        ["sample.tiff"]=""
        ["sample.webp"]="-quality 85"
        ["sample.tga"]=""
        ["sample.ppm"]=""
        ["sample.pgm"]="-colorspace gray"
        ["sample.pbm"]="-colorspace gray -threshold 50%"
        ["sample.pcx"]=""
        ["sample.ico"]="-resize 256x256"
        ["tiny.png"]="-resize 16x16"
        ["large_4k.png"]="-resize 3840x2160"
        ["transparent.png"]="-alpha on -channel A -evaluate set 50%"
        ["grayscale.png"]="-colorspace gray"
        ["small_100x100.png"]="-resize 100x100"
        ["wide_1920x200.png"]="-resize 1920x200!"
        ["tall_200x1920.png"]="-resize 200x1920!"
    )

    for format in "${!IMAGE_FORMATS[@]}"; do
        output="$IMAGES_DIR/$format"
        if [ ! -f "$output" ]; then
            args="${IMAGE_FORMATS[$format]}"
            if magick "$BASE_PNG" $args "$output" 2>/dev/null; then
                echo "  Created: $format"
            else
                echo "  FAILED: $format"
            fi
        else
            echo "  EXISTS: $format"
        fi
    done

    # Create animated GIF
    ANIMATED_GIF="$IMAGES_DIR/animated.gif"
    if [ ! -f "$ANIMATED_GIF" ]; then
        echo "  Creating animated.gif..."
        TEMP_DIR=$(mktemp -d)
        for i in {0..4}; do
            hue=$((i * 72))
            magick "$BASE_PNG" -modulate 100,100,$hue "$TEMP_DIR/frame_$i.png" 2>/dev/null
        done
        magick -delay 50 -loop 0 "$TEMP_DIR/frame_"*.png "$ANIMATED_GIF" 2>/dev/null && \
            echo "  Created: animated.gif"
        rm -rf "$TEMP_DIR"
    fi
fi

# ============================================
# Generate Video Fixtures
# ============================================

echo ""
echo "Generating video fixtures..."

BASE_VIDEO="$VIDEO_DIR/sample.mp4"

if [ "$HAS_FFMPEG" = true ]; then
    # Create base video if not exists (5 second test pattern)
    if [ ! -f "$BASE_VIDEO" ]; then
        echo "  Creating base sample.mp4..."
        ffmpeg -y -f lavfi -i "testsrc=duration=5:size=640x480:rate=30" \
            -f lavfi -i "sine=frequency=440:duration=5" \
            -c:v libx264 -preset ultrafast -crf 23 \
            -c:a aac -b:a 128k \
            -pix_fmt yuv420p \
            "$BASE_VIDEO" 2>/dev/null
        echo "  Created: sample.mp4"
    fi

    if [ -f "$BASE_VIDEO" ]; then
        declare -A VIDEO_FORMATS=(
            ["sample.mov"]="-c:v libx264 -c:a aac"
            ["sample.avi"]="-c:v mpeg4 -c:a mp3"
            ["sample.mkv"]="-c:v copy -c:a copy"
            ["sample.webm"]="-c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus"
            ["sample_short.mp4"]="-t 2 -c:v copy -c:a copy"
        )

        for format in "${!VIDEO_FORMATS[@]}"; do
            output="$VIDEO_DIR/$format"
            if [ ! -f "$output" ]; then
                args="${VIDEO_FORMATS[$format]}"
                if ffmpeg -y -i "$BASE_VIDEO" $args "$output" 2>/dev/null; then
                    echo "  Created: $format"
                else
                    echo "  FAILED: $format"
                fi
            else
                echo "  EXISTS: $format"
            fi
        done
    fi
fi

# ============================================
# Generate Audio Fixtures
# ============================================

echo ""
echo "Generating audio fixtures..."

if [ "$HAS_FFMPEG" = true ]; then
    BASE_AUDIO="$AUDIO_DIR/sample.mp3"
    
    # Create base audio (5 second tone)
    if [ ! -f "$BASE_AUDIO" ]; then
        echo "  Creating base sample.mp3..."
        ffmpeg -y -f lavfi -i "sine=frequency=440:duration=5" \
            -c:a libmp3lame -b:a 192k \
            "$BASE_AUDIO" 2>/dev/null
        echo "  Created: sample.mp3"
    fi

    if [ -f "$BASE_AUDIO" ]; then
        declare -A AUDIO_FORMATS=(
            ["sample.wav"]="-c:a pcm_s16le"
            ["sample.flac"]="-c:a flac"
            ["sample.ogg"]="-c:a libvorbis -q:a 5"
            ["sample.m4a"]="-c:a aac -b:a 192k"
            ["sample.aac"]="-c:a aac -b:a 192k"
            ["sample_stereo.mp3"]="-ac 2 -c:a libmp3lame"
            ["sample_mono.mp3"]="-ac 1 -c:a libmp3lame"
        )

        for format in "${!AUDIO_FORMATS[@]}"; do
            output="$AUDIO_DIR/$format"
            if [ ! -f "$output" ]; then
                args="${AUDIO_FORMATS[$format]}"
                if ffmpeg -y -i "$BASE_AUDIO" $args "$output" 2>/dev/null; then
                    echo "  Created: $format"
                else
                    echo "  FAILED: $format"
                fi
            else
                echo "  EXISTS: $format"
            fi
        done
    fi
fi

# ============================================
# Generate Document Fixtures
# ============================================

echo ""
echo "Generating document fixtures..."

# Create sample markdown
SAMPLE_MD="$DOCUMENTS_DIR/sample.md"
if [ ! -f "$SAMPLE_MD" ]; then
    cat > "$SAMPLE_MD" << 'EOF'
# ConvertSave Test Document

This is a **sample markdown** document for testing file conversions.

## Features

- Bullet point 1
- Bullet point 2
- Bullet point 3

## Code Example

```rust
fn main() {
    println!("Hello, ConvertSave!");
}
```

## Table

| Format | Tool | Status |
|--------|------|--------|
| PNG | ImageMagick | ✓ |
| MP4 | FFmpeg | ✓ |
| PDF | LibreOffice | ✓ |

---

*Generated for ConvertSave testing*
EOF
    echo "  Created: sample.md"
fi

# Create sample HTML
SAMPLE_HTML="$DOCUMENTS_DIR/sample.html"
if [ ! -f "$SAMPLE_HTML" ]; then
    cat > "$SAMPLE_HTML" << 'EOF'
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
EOF
    echo "  Created: sample.html"
fi

# Create sample plain text
SAMPLE_TXT="$DOCUMENTS_DIR/sample.txt"
if [ ! -f "$SAMPLE_TXT" ]; then
    cat > "$SAMPLE_TXT" << 'EOF'
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
EOF
    echo "  Created: sample.txt"
fi

# Create sample RTF
SAMPLE_RTF="$DOCUMENTS_DIR/sample.rtf"
if [ ! -f "$SAMPLE_RTF" ]; then
    cat > "$SAMPLE_RTF" << 'EOF'
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
EOF
    echo "  Created: sample.rtf"
fi

# ============================================
# Summary
# ============================================

echo ""
echo "=== Fixture Generation Complete ==="
echo ""

IMAGE_COUNT=$(find "$IMAGES_DIR" -type f 2>/dev/null | wc -l | tr -d ' ')
VIDEO_COUNT=$(find "$VIDEO_DIR" -type f 2>/dev/null | wc -l | tr -d ' ')
AUDIO_COUNT=$(find "$AUDIO_DIR" -type f 2>/dev/null | wc -l | tr -d ' ')
DOC_COUNT=$(find "$DOCUMENTS_DIR" -type f 2>/dev/null | wc -l | tr -d ' ')

echo "Fixtures created:"
echo "  Images:    $IMAGE_COUNT files"
echo "  Videos:    $VIDEO_COUNT files"
echo "  Audio:     $AUDIO_COUNT files"
echo "  Documents: $DOC_COUNT files"
echo ""
TOTAL=$((IMAGE_COUNT + VIDEO_COUNT + AUDIO_COUNT + DOC_COUNT))
echo "Total: $TOTAL fixture files"
echo ""
echo "Run tests with: npm test"


