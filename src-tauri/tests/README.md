# ConvertSave Test Suite

This directory contains integration tests for all conversion features in ConvertSave.

## Test Structure

```
tests/
├── conversion_tests.rs     # Main test file with all conversion tests
├── fixtures/               # Test input files (YOU MUST PROVIDE THESE)
│   ├── images/            # Image test files
│   │   ├── sample.png     # Basic PNG test file
│   │   ├── sample.jpg     # Basic JPEG test file
│   │   ├── sample2.png    # Additional PNG for batch tests
│   │   ├── sample3.png    # Additional PNG for batch tests
│   │   ├── transparent.png # PNG with transparency
│   │   ├── animated.gif   # Animated GIF test
│   │   ├── large_4k.png   # Large image (4K resolution)
│   │   ├── tiny.png       # Very small image (1x1)
│   │   └── rotated.heic   # HEIC with EXIF rotation
│   ├── documents/         # Document test files
│   │   ├── sample.md      # Markdown document
│   │   ├── sample.html    # HTML document
│   │   └── sample.txt     # Plain text document
│   └── video/             # Video/audio test files
│       ├── sample.mp4     # MP4 video file
│       ├── sample.avi     # AVI video file
│       └── sample.mov     # QuickTime video file
└── output/                # Test output directory (auto-created, git-ignored)
```

## Setting Up Test Fixtures

### Required Test Files

You need to provide test files in the `fixtures/` directory. The tests are marked with `#[ignore]` so they won't run automatically until you have the fixtures in place.

### Where to Get Test Files

#### Images (`fixtures/images/`)

1. **sample.png** - Any PNG image (recommended: 800x600px)
2. **sample.jpg** - Any JPEG image (recommended: 800x600px)
3. **sample2.png, sample3.png** - Additional PNGs for batch testing (can be copies)
4. **transparent.png** - PNG with transparency (you can create one in any image editor)
5. **animated.gif** - Any animated GIF
6. **large_4k.png** - A large image (3840x2160 or higher)
7. **tiny.png** - A 1x1 pixel image (create with: `convert -size 1x1 xc:white tiny.png`)
8. **rotated.heic** - iPhone photo with orientation data (optional)

**Quick way to create test images:**

```bash
# Using ImageMagick (if installed)
cd src-tauri/tests/fixtures/images

# Create basic test images
convert -size 800x600 xc:blue sample.png
convert -size 800x600 xc:red sample.jpg
convert -size 800x600 xc:green sample2.png
convert -size 800x600 xc:yellow sample3.png

# Create transparent image
convert -size 100x100 xc:none transparent.png

# Create tiny image
convert -size 1x1 xc:white tiny.png

# Create large image
convert -size 3840x2160 xc:blue large_4k.png
```

Or simply download free test images from:

- [Unsplash](https://unsplash.com/) - Free high-quality images
- [Pexels](https://www.pexels.com/) - Free stock photos
- [Pixabay](https://pixabay.com/) - Free images and videos

#### Documents (`fixtures/documents/`)

1. **sample.md** - Create a simple Markdown file:

```markdown
# Test Document

This is a test document for ConvertSave.

## Features

- Markdown support
- Multiple format conversions
- Easy to use

### Lists

1. First item
2. Second item
3. Third item

**Bold text** and _italic text_ are supported.
```

2. **sample.html** - Create a simple HTML file:

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Test Document</title>
  </head>
  <body>
    <h1>Test Document</h1>
    <p>This is a test document for ConvertSave.</p>
    <ul>
      <li>List item 1</li>
      <li>List item 2</li>
    </ul>
  </body>
</html>
```

3. **sample.txt** - Create a plain text file:

```
Test Document
=============

This is a test document for ConvertSave.

It contains multiple lines of text
for testing conversions.
```

#### Videos (`fixtures/video/`)

For video files, you have several options:

1. **Use your own short video clips** (recommended: 5-10 seconds, small file size)
2. **Download sample videos:**

   - [Sample Videos](https://sample-videos.com/) - Free sample videos
   - [Pexels Videos](https://www.pexels.com/videos/) - Free stock videos

3. **Create test videos with FFmpeg:**

```bash
cd src-tauri/tests/fixtures/video

# Create a 5-second test video (color bars with audio tone)
ffmpeg -f lavfi -i testsrc=duration=5:size=640x480:rate=30 \
       -f lavfi -i sine=frequency=1000:duration=5 \
       -c:v libx264 -c:a aac sample.mp4

# Convert to AVI
ffmpeg -i sample.mp4 -c:v mpeg4 sample.avi

# Convert to MOV
ffmpeg -i sample.mp4 -c:v libx264 -c:a aac sample.mov
```

## Running Tests

### Run All Tests

```bash
cd src-tauri
cargo test --test conversion_tests -- --ignored --nocapture
```

### Run Specific Test Category

```bash
# Image conversion tests only
cargo test --test conversion_tests image_conversions -- --ignored --nocapture

# Document conversion tests only
cargo test --test conversion_tests document_conversions -- --ignored --nocapture

# Video/audio conversion tests only
cargo test --test conversion_tests video_audio_conversions -- --ignored --nocapture

# Edge case tests only
cargo test --test conversion_tests edge_cases -- --ignored --nocapture
```

### Run Individual Test

```bash
# Test PNG to JPG conversion
cargo test --test conversion_tests test_png_to_jpg -- --ignored --nocapture

# Test Markdown to HTML conversion
cargo test --test conversion_tests test_md_to_html -- --ignored --nocapture
```

## Test Coverage

### Image Conversions (28+ formats)

- ✅ PNG ↔ JPG
- ✅ PNG ↔ WebP
- ✅ PNG → GIF
- ✅ PNG → BMP
- ✅ PNG → TIFF
- ✅ PNG → AVIF
- ✅ PNG → ICO
- ✅ JPG → TGA
- ✅ PNG → JPEG 2000 (J2K)
- And many more combinations

### Document Conversions

- ✅ Markdown → HTML
- ✅ Markdown → DOCX
- ✅ Markdown → EPUB
- ✅ Markdown → TXT
- ✅ HTML → Markdown
- ✅ HTML → DOCX
- ✅ TXT → HTML
- ✅ TXT → Markdown

### Video/Audio Conversions

- ✅ MP4 → MP3 (audio extraction)
- ✅ MP4 → WebM
- ✅ AVI → MP4
- ✅ MOV → MP3

### Edge Cases Tested

- ✅ Large images (4K+)
- ✅ Tiny images (1x1)
- ✅ Transparent PNGs
- ✅ Animated GIFs
- ✅ HEIC with rotation
- ✅ Batch conversions

## Tests Are Ready to Run!

The tests are **fully functional** - they directly call conversion tools and perform real conversions. No modifications needed!

- ✅ Calls ffmpeg, pandoc, imagemagick directly
- ✅ Performs actual file conversions
- ✅ Verifies output files
- ✅ Auto-cleanup

## Adding New Tests

To add a new conversion test, use the `conversion_test!` macro:

```rust
#[cfg(test)]
mod my_tests {
    use super::*;

    // Simple one-liner test using the macro
    conversion_test!(test_png_to_svg, "images/sample.png", "output.svg", "svg");
}
```

Or write a custom test:

```rust
#[test]
#[ignore]
fn test_my_custom_conversion() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let input = get_fixtures_dir().join("images").join("input.png");
        let output = get_output_dir().join("output.svg");

        perform_conversion(&input, "svg", &output).await.expect("Conversion failed");
        assert_output_exists(&output);
        fs::remove_file(output).ok();
    });
}
```

3. Run the test: `cargo test test_my_new_conversion -- --ignored --nocapture`

## CI/CD Integration

To run these tests in GitHub Actions, add to your workflow:

```yaml
- name: Setup test fixtures
  run: |
    # Download or create test fixtures
    # Add your fixture setup commands here

- name: Run conversion tests
  run: |
    cd src-tauri
    cargo test --test conversion_tests -- --ignored --nocapture
```

## Troubleshooting

### Tests are not running

- Make sure fixtures are in the correct directories
- Run with `--ignored` flag since tests are marked as ignored
- Check that required tools (ffmpeg, pandoc, etc.) are installed

### Test fails with "Output file does not exist"

- Verify the conversion command is working correctly
- Check that the required tool (ffmpeg/pandoc/imagemagick) is installed
- Look at test output with `--nocapture` flag

### Output directory permission errors

- The output directory is auto-created
- Make sure you have write permissions in the tests directory

## Notes

- Tests automatically clean up output files after completion
- Output directory is git-ignored
- Fixture files should be committed to version control (except very large files)
- Consider using Git LFS for large video fixtures
