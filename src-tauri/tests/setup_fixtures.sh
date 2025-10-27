#!/bin/bash
# Setup script for generating test fixtures
# Requires: ImageMagick (convert/magick) and FFmpeg

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FIXTURES_DIR="$SCRIPT_DIR/fixtures"

echo "ConvertSave Test Fixtures Setup"
echo "================================"
echo ""

# Check for ImageMagick
if ! command -v convert &> /dev/null && ! command -v magick &> /dev/null; then
    echo "⚠️  ImageMagick not found. Image fixtures will need to be added manually."
    echo "   Install from: https://imagemagick.org/"
    HAVE_IMAGEMAGICK=false
else
    echo "✓ ImageMagick found"
    HAVE_IMAGEMAGICK=true
fi

# Check for FFmpeg
if ! command -v ffmpeg &> /dev/null; then
    echo "⚠️  FFmpeg not found. Video fixtures will need to be added manually."
    echo "   Install from: https://ffmpeg.org/"
    HAVE_FFMPEG=false
else
    echo "✓ FFmpeg found"
    HAVE_FFMPEG=true
fi

echo ""

# Create directories
mkdir -p "$FIXTURES_DIR/images"
mkdir -p "$FIXTURES_DIR/documents"
mkdir -p "$FIXTURES_DIR/video"

# Generate image fixtures
if [ "$HAVE_IMAGEMAGICK" = true ]; then
    echo "Generating image test fixtures..."
    cd "$FIXTURES_DIR/images"
    
    # Use magick command for ImageMagick 7+, convert for older versions
    CMD="convert"
    if command -v magick &> /dev/null; then
        CMD="magick"
    fi
    
    # Basic test images
    echo "  - Creating sample.png (800x600, blue)"
    $CMD -size 800x600 xc:blue sample.png
    
    echo "  - Creating sample.jpg (800x600, red)"
    $CMD -size 800x600 xc:red sample.jpg
    
    echo "  - Creating sample2.png (800x600, green)"
    $CMD -size 800x600 xc:green sample2.png
    
    echo "  - Creating sample3.png (800x600, yellow)"
    $CMD -size 800x600 xc:yellow sample3.png
    
    # Transparent image
    echo "  - Creating transparent.png (100x100, transparent)"
    $CMD -size 100x100 xc:none transparent.png
    
    # Tiny image
    echo "  - Creating tiny.png (1x1, white)"
    $CMD -size 1x1 xc:white tiny.png
    
    # Large 4K image (optional, can be slow)
    echo "  - Creating large_4k.png (3840x2160, blue)"
    $CMD -size 3840x2160 xc:blue large_4k.png
    
    # Animated GIF
    echo "  - Creating animated.gif (3 frames)"
    $CMD -delay 20 -loop 0 sample.png sample2.png sample3.png animated.gif
    
    echo "✓ Image fixtures created"
else
    echo "⊘ Skipping image fixture generation (ImageMagick not available)"
fi

echo ""

# Generate video fixtures
if [ "$HAVE_FFMPEG" = true ]; then
    echo "Generating video test fixtures..."
    cd "$FIXTURES_DIR/video"
    
    # Create a 5-second test video with color bars and audio tone
    echo "  - Creating sample.mp4 (5 seconds, 640x480)"
    ffmpeg -f lavfi -i testsrc=duration=5:size=640x480:rate=30 \
           -f lavfi -i sine=frequency=1000:duration=5 \
           -c:v libx264 -pix_fmt yuv420p -c:a aac -b:a 128k \
           -y sample.mp4 2>/dev/null
    
    # Convert to AVI
    echo "  - Creating sample.avi from sample.mp4"
    ffmpeg -i sample.mp4 -c:v mpeg4 -q:v 5 -c:a mp3 -b:a 128k \
           -y sample.avi 2>/dev/null
    
    # Convert to MOV
    echo "  - Creating sample.mov from sample.mp4"
    ffmpeg -i sample.mp4 -c:v libx264 -pix_fmt yuv420p -c:a aac -b:a 128k \
           -y sample.mov 2>/dev/null
    
    echo "✓ Video fixtures created"
else
    echo "⊘ Skipping video fixture generation (FFmpeg not available)"
fi

echo ""
echo "Test Fixtures Setup Complete!"
echo "=============================="
echo ""
echo "Fixtures location: $FIXTURES_DIR"
echo ""
echo "Document fixtures (MD, HTML, TXT) are already included."
echo ""
echo "Next steps:"
echo "  1. Review the generated fixtures in: $FIXTURES_DIR"
echo "  2. (Optional) Replace generated images with real photos for better testing"
echo "  3. Run tests: cd src-tauri && cargo test --test conversion_tests -- --ignored --nocapture"
echo ""
echo "For more information, see: $SCRIPT_DIR/README.md"

