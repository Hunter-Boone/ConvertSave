# Debug ImageMagick on macOS

## Step 1: Clean Up Old Installation
```bash
# Remove the old broken installation
rm -rf ~/Library/Application\ Support/com.convertsave/imagemagick/
```

## Step 2: Check if the new build is ready
The GitHub Actions build needs to complete first. Check: https://github.com/Hunter-Boone/ConvertSave/actions

Once the build is done:
1. Download the new `.dmg` for macOS
2. Install it
3. Open the app

## Step 3: Download ImageMagick in the App
1. Go to Settings/Tools
2. Click "Download ImageMagick"
3. Watch the console output for any errors

## Step 4: Check What Was Installed

Run these commands in Terminal:

```bash
# Check if ImageMagick was downloaded
ls -la ~/Library/Application\ Support/com.convertsave/imagemagick/

# Check the magick binary's library dependencies
otool -L ~/Library/Application\ Support/com.convertsave/imagemagick/magick

# Look for lines like:
#   @executable_path/libMagickCore-7.Q16HDRI.8.dylib  ✓ GOOD
#   /ImageMagick-7.0.10/lib/libMagickCore-7.Q16HDRI.8.dylib  ✗ BAD

# Check if dylib files are in the same directory as magick
ls -la ~/Library/Application\ Support/com.convertsave/imagemagick/*.dylib
```

## Step 5: Test Conversion

Try converting an image and send me:
1. The error message you get
2. Output from the `otool -L` command above
3. Output from the `ls -la` commands above

## Alternative: Use Homebrew (Fastest Solution)

If you want ImageMagick working immediately:

```bash
# Install Homebrew if you don't have it
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install ImageMagick
brew install imagemagick

# Verify it works
magick --version
```

Then just restart the ConvertSave app - it will automatically detect and use Homebrew's ImageMagick!

