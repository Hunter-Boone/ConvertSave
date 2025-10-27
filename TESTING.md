# ConvertSave Testing Guide

This document provides an overview of the test suite for ConvertSave.

## Test Suite Location

All tests are located in: `src-tauri/tests/`

## Quick Start

### 1. Set Up Test Fixtures

You need to provide test files for the conversion tests. You have two options:

#### Option A: Use the Setup Script (Recommended)

**On Windows (PowerShell):**

```powershell
cd src-tauri\tests
.\setup_fixtures.ps1
```

**On Linux/macOS (Bash):**

```bash
cd src-tauri/tests
chmod +x setup_fixtures.sh
./setup_fixtures.sh
```

This will automatically generate test images and videos if you have ImageMagick and FFmpeg installed.

#### Option B: Manual Setup

1. Download or create test files manually
2. Place them in the appropriate directories:
   - Images: `src-tauri/tests/fixtures/images/`
   - Documents: `src-tauri/tests/fixtures/documents/`
   - Videos: `src-tauri/tests/fixtures/video/`

See `src-tauri/tests/README.md` for detailed requirements.

### 2. Run Tests

```bash
# Run all conversion tests
cd src-tauri
cargo test --test conversion_tests -- --ignored --nocapture

# Run specific category
cargo test --test conversion_tests image_conversions -- --ignored --nocapture
cargo test --test conversion_tests document_conversions -- --ignored --nocapture
cargo test --test conversion_tests video_audio_conversions -- --ignored --nocapture

# Run individual test
cargo test --test conversion_tests test_png_to_jpg -- --ignored --nocapture
```

## Test Coverage

The test suite covers:

### Image Conversions (11 tests)

- PNG ↔ JPG
- PNG/JPG → WebP
- PNG → GIF, BMP, TIFF, AVIF, ICO, J2K
- JPG → TGA

### Document Conversions (8 tests)

- Markdown → HTML, DOCX, EPUB, TXT
- HTML → Markdown, DOCX
- TXT → HTML, Markdown

### Video/Audio Conversions (4 tests)

- MP4/MOV → MP3 (audio extraction)
- MP4 → WebM
- AVI → MP4

### Edge Cases (5 tests)

- Large images (4K+)
- Tiny images (1x1 pixel)
- Transparent PNGs → formats without transparency
- Animated GIF conversions
- HEIC with EXIF rotation data

### Batch Operations (1 test)

- Batch PNG → JPG conversion

**Total: 29 comprehensive tests**

## Test File Requirements

### Required Image Files

- `sample.png` - Basic PNG (recommended: 800x600px)
- `sample.jpg` - Basic JPEG (recommended: 800x600px)
- `sample2.png` - Additional PNG
- `sample3.png` - Additional PNG
- `transparent.png` - PNG with alpha channel
- `animated.gif` - Animated GIF
- `large_4k.png` - 4K image (3840x2160+)
- `tiny.png` - 1x1 pixel image
- `rotated.heic` - HEIC with rotation (optional)

### Required Document Files

- `sample.md` - Markdown document ✅ (provided)
- `sample.html` - HTML document ✅ (provided)
- `sample.txt` - Plain text document ✅ (provided)

### Required Video Files

- `sample.mp4` - MP4 video (5-10 seconds recommended)
- `sample.avi` - AVI video
- `sample.mov` - QuickTime MOV video

## Where Test Files Are Stored

```
ConvertSave/
└── src-tauri/
    └── tests/
        ├── conversion_tests.rs    # Main test file
        ├── README.md              # Detailed test documentation
        ├── setup_fixtures.sh      # Setup script (Linux/macOS)
        ├── setup_fixtures.ps1     # Setup script (Windows)
        ├── fixtures/              # Test input files (YOU PROVIDE THESE)
        │   ├── images/           # Image test files
        │   │   ├── README.md
        │   │   ├── sample.png
        │   │   ├── sample.jpg
        │   │   └── ... (other images)
        │   ├── documents/        # Document test files
        │   │   ├── sample.md     ✅ Provided
        │   │   ├── sample.html   ✅ Provided
        │   │   └── sample.txt    ✅ Provided
        │   └── video/            # Video test files
        │       ├── README.md
        │       ├── sample.mp4
        │       ├── sample.avi
        │       └── sample.mov
        └── output/               # Test output (auto-created, git-ignored)
```

## Creating Test Images

If you have ImageMagick installed:

```bash
cd src-tauri/tests/fixtures/images

# Basic test images
magick -size 800x600 xc:blue sample.png
magick -size 800x600 xc:red sample.jpg
magick -size 800x600 xc:green sample2.png
magick -size 800x600 xc:yellow sample3.png

# Special cases
magick -size 100x100 xc:none transparent.png
magick -size 1x1 xc:white tiny.png
magick -size 3840x2160 xc:blue large_4k.png
magick -delay 20 -loop 0 sample.png sample2.png sample3.png animated.gif
```

Or download free images from:

- [Unsplash](https://unsplash.com/)
- [Pexels](https://www.pexels.com/)
- [Pixabay](https://pixabay.com/)

## Creating Test Videos

If you have FFmpeg installed:

```bash
cd src-tauri/tests/fixtures/video

# Create test video with color bars and audio
ffmpeg -f lavfi -i testsrc=duration=5:size=640x480:rate=30 \
       -f lavfi -i sine=frequency=1000:duration=5 \
       -c:v libx264 -c:a aac sample.mp4

# Convert to other formats
ffmpeg -i sample.mp4 -c:v mpeg4 sample.avi
ffmpeg -i sample.mp4 -c:v libx264 -c:a aac sample.mov
```

Or download sample videos from:

- [Sample Videos](https://sample-videos.com/)
- [Pexels Videos](https://www.pexels.com/videos/)

## ✅ Tests Are Fully Functional!

The tests are **completely ready to run** - no uncommenting or modifications needed! They:

- ✅ Call conversion tools directly (ffmpeg, pandoc, imagemagick)
- ✅ Perform real file conversions
- ✅ Verify output files exist and have content
- ✅ Clean up after themselves automatically

## Why Tests Are Marked `#[ignore]`

The tests are marked with `#[ignore]` because they require:

1. Test fixture files to be present (images, videos, documents)
2. External tools (ffmpeg, pandoc, etc.) to be installed
3. Actual file system operations and conversions

This prevents tests from failing in CI/CD or development environments where these prerequisites aren't met. Use the `--ignored` flag to run them when you're ready!

## Running Tests in CI/CD

To run these tests in GitHub Actions or other CI/CD systems:

1. Install required tools (ffmpeg, pandoc, imagemagick, libreoffice)
2. Generate or download test fixtures
3. Run tests with the `--ignored` flag

Example GitHub Actions workflow:

```yaml
- name: Install dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y ffmpeg pandoc imagemagick

- name: Setup test fixtures
  run: |
    cd src-tauri/tests
    chmod +x setup_fixtures.sh
    ./setup_fixtures.sh

- name: Run conversion tests
  run: |
    cd src-tauri
    cargo test --test conversion_tests -- --ignored --nocapture
```

## Test Output

- All test output files are written to `src-tauri/tests/output/`
- This directory is automatically created when tests run
- It's git-ignored to prevent committing test artifacts
- Tests automatically clean up after themselves

## Adding New Tests

To add a new conversion test:

1. Add the test fixture file to `fixtures/`
2. Create a test function in `conversion_tests.rs`:

```rust
#[test]
#[ignore]
fn test_my_conversion() {
    let _input = get_fixtures_dir().join("images").join("input.png");
    let output = get_output_dir().join("output.svg");

    // TODO: Call the conversion function
    // This will be implemented once you integrate with actual conversion logic

    assert_output_exists(&output);
    fs::remove_file(output).ok();
}
```

3. Run: `cargo test test_my_conversion -- --ignored --nocapture`

## Troubleshooting

### "Test failed: Output file does not exist"

- Make sure the conversion tools are installed
- Check that test fixtures exist in the correct location
- Verify the conversion function is being called correctly

### "No such file or directory" errors

- Run the setup script to generate test fixtures
- Manually add required test files

### "Permission denied" errors

- Ensure you have write permissions in the tests directory
- Try running with appropriate permissions

## For More Information

See the detailed test documentation in: `src-tauri/tests/README.md`
