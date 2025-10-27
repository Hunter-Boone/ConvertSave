# Video Test Fixtures

Place your test video files in this directory. You need to provide these files yourself.

## Required Files

- `sample.mp4` - MP4 video file (recommended: 5-10 seconds, small size)
- `sample.avi` - AVI video file
- `sample.mov` - QuickTime video file

## Create Test Videos with FFmpeg

If you have FFmpeg installed, you can create test videos:

```bash
# Create a 5-second test video (color bars with audio tone)
ffmpeg -f lavfi -i testsrc=duration=5:size=640x480:rate=30 \
       -f lavfi -i sine=frequency=1000:duration=5 \
       -c:v libx264 -c:a aac sample.mp4

# Convert to AVI
ffmpeg -i sample.mp4 -c:v mpeg4 sample.avi

# Convert to MOV
ffmpeg -i sample.mp4 -c:v libx264 -c:a aac sample.mov
```

## Or Download Sample Videos

- [Sample Videos](https://sample-videos.com/)
- [Pexels Videos](https://www.pexels.com/videos/)

**Note:** Keep test videos small (under 10MB) and short (5-10 seconds) for fast test execution.
