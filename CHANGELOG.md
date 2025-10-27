# Changelog

All notable changes to ConvertSave will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- On-demand binary downloader for FFmpeg and Pandoc
- Tool downloader UI component with real-time progress
- Support for downloading from official sources:
  - FFmpeg from BtbN/FFmpeg-Builds and evermeet.cx
  - Pandoc from official GitHub releases
- Cross-platform archive extraction (ZIP, TAR.GZ, TAR.XZ)
- Tool status checking on application startup
- GitHub Actions workflow for automated releases
- Comprehensive documentation:
  - `NOTICE.md` for license compliance
  - `.github/RELEASE.md` for release process
  - `.github/SIGNING.md` for code signing setup
  - `.github/README.md` for CI/CD documentation

### Changed

- Tool path resolution now checks app data directory first
- Updated to check for downloaded tools before bundled tools
- Removed bundled FFmpeg and Pandoc binaries (GPL compliance)
- Application size reduced from ~100MB+ to ~5-10MB

### Fixed

- ICO conversion now automatically resizes large images to 256x256 pixels to fit icon format limitations
- Prevented "width or height exceeds limit" errors when converting photos to ICO format

### Security

- Added HTTP permissions for downloading from official sources only
- Implemented secure tool verification
- Binary downloads from verified official repositories

## [0.1.0] - Initial Release

### Added

- File conversion support for:
  - Images (PNG, JPG, GIF, BMP, WebP, TIFF, etc.)
  - Videos (MP4, MOV, AVI, MKV, WebM)
  - Audio (MP3, WAV, FLAC, OGG, M4A)
  - Documents (Markdown, PDF, DOCX, etc.)
- Drag and drop file upload
- Batch file conversion
- Individual file settings
- Advanced conversion options
- Custom output directory selection
- Cross-platform support (Windows, macOS, Linux)
- Modern UI with Tauri + React + TypeScript
- Platform-specific window controls

### Features

- Support for 50+ image formats
- Video/audio conversion with FFmpeg
- Document conversion with Pandoc
- Real-time conversion progress
- Batch settings for multiple files
- Per-file format customization
