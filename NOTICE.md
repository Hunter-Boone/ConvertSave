# ConvertSave - Third-Party Software Notices and Information

ConvertSave uses third-party conversion tools that are downloaded on-demand to comply with their respective licenses.

## FFmpeg

**Website**: https://ffmpeg.org/  
**License**: GNU General Public License (GPL) version 3  
**License URL**: https://www.gnu.org/licenses/gpl-3.0.html

FFmpeg is a complete, cross-platform solution to record, convert and stream audio and video. ConvertSave downloads FFmpeg binaries from official sources when needed:

- **Windows**: Downloaded from [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds/releases)
- **macOS**: Downloaded from [evermeet.cx](https://evermeet.cx/ffmpeg/)
- **Linux**: Downloaded from [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds/releases)

FFmpeg is licensed under the GPL version 3, which requires that any software using it must also be open source under a compatible license. The full source code for FFmpeg can be obtained from the official FFmpeg website.

### Important Notes:

- ConvertSave does NOT bundle FFmpeg; it downloads it separately on first use
- Users can view the exact location of the downloaded FFmpeg binary in the application
- FFmpeg is stored in the user's application data directory

## Pandoc

**Website**: https://pandoc.org/  
**License**: GNU General Public License (GPL) version 2 or later  
**License URL**: https://www.gnu.org/licenses/gpl-2.0.html

Pandoc is a universal document converter that can convert files from one markup format into another. ConvertSave downloads Pandoc binaries from the official GitHub releases:

- **All platforms**: Downloaded from [jgm/pandoc](https://github.com/jgm/pandoc/releases)

Pandoc is licensed under the GPL version 2 or later. The full source code for Pandoc can be obtained from the official Pandoc GitHub repository.

### Important Notes:

- ConvertSave does NOT bundle Pandoc; it downloads it separately on first use
- Users can view the exact location of the downloaded Pandoc binary in the application
- Pandoc is stored in the user's application data directory

## LibreOffice (Future Support)

ConvertSave may add support for LibreOffice in the future for office document conversions. If implemented, users will be directed to install LibreOffice separately from the official sources.

## Compliance

By downloading these tools separately rather than bundling them:

1. **License Compliance**: We ensure full compliance with GPL requirements
2. **User Choice**: Users explicitly choose to download and use these tools
3. **Size Optimization**: The application remains lightweight
4. **Updates**: Users can benefit from the latest versions of these tools
5. **Transparency**: The download sources are clearly documented

## Source Code

The complete source code for ConvertSave is available at: [Your Repository URL]

ConvertSave itself is licensed under [Your License - recommend GPL v3 to be compatible with FFmpeg].

---

**Last Updated**: October 2024

For questions about licensing or to report issues, please visit the ConvertSave repository or contact the maintainers.
