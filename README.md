# ConvertSave

A native cross-platform file conversion utility built with Tauri, React, and Tailwind CSS.

## Features

- 🔄 Convert between numerous file formats using powerful conversion tools
- 🎨 Beautiful UI inspired by Gumroad's aesthetic with chunky colorful buttons
- 🚀 Native performance with small bundle size
- 🖥️ Cross-platform support (Windows, macOS, Linux)
- 🛠️ Advanced options for power users
- 📁 Custom output directory selection
- 🔧 Bundled conversion tools (FFmpeg, Pandoc, LibreOffice, ImageMagick)

## Development Setup

### Prerequisites

- Node.js (v18 or later)
- Rust (latest stable)
- Platform-specific build tools:
  - Windows: Microsoft C++ Build Tools
  - macOS: Xcode Command Line Tools
  - Linux: build-essential, libwebkit2gtk-4.0-dev

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/convertsave.git
cd convertsave
```

2. Install dependencies:
```bash
npm install
```

3. Download and place conversion tools in the `tools/` directory (see tools/README.md)

4. Run in development mode:
```bash
npm run tauri dev
```

### Building

To build for production:

```bash
npm run tauri build
```

This will create platform-specific installers in `src-tauri/target/release/bundle/`.

## Supported Conversions

### Video/Audio (FFmpeg)
- Input: MP4, MOV, AVI, MKV, WebM, FLV, MP3, WAV, FLAC, etc.
- Output: MP4, WebM, MP3, WAV, AAC, etc.

### Documents (Pandoc & LibreOffice)
- Input: DOCX, ODT, Markdown, HTML, RTF, etc.
- Output: PDF, EPUB, HTML, TXT, etc.

### Images (ImageMagick)
- Input: JPG, PNG, GIF, BMP, TIFF, WebP, etc.
- Output: JPG, PNG, WebP, PDF, etc.

## License

MIT License - see LICENSE file for details
