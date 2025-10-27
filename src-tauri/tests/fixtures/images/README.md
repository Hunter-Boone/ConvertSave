# Image Test Fixtures

Place your test images in this directory. You need to provide these files yourself.

## Required Files

- `sample.png` - Basic PNG test file (recommended: 800x600px)
- `sample.jpg` - Basic JPEG test file (recommended: 800x600px)
- `sample2.png` - Additional PNG for batch tests
- `sample3.png` - Additional PNG for batch tests
- `transparent.png` - PNG with transparency
- `animated.gif` - Animated GIF test
- `large_4k.png` - Large image (3840x2160 or higher)
- `tiny.png` - Very small image (1x1 pixel)
- `rotated.heic` - HEIC with EXIF rotation (optional)

## Quick Creation with ImageMagick

If you have ImageMagick installed:

```bash
# Basic test images
convert -size 800x600 xc:blue sample.png
convert -size 800x600 xc:red sample.jpg
convert -size 800x600 xc:green sample2.png
convert -size 800x600 xc:yellow sample3.png

# Transparent image
convert -size 100x100 xc:none transparent.png

# Tiny image
convert -size 1x1 xc:white tiny.png

# Large 4K image
convert -size 3840x2160 xc:blue large_4k.png

# Simple animated GIF
convert -delay 20 -loop 0 sample.png sample2.png sample3.png animated.gif
```

## Or Download Free Images

- [Unsplash](https://unsplash.com/)
- [Pexels](https://www.pexels.com/)
- [Pixabay](https://pixabay.com/)
