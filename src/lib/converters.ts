import { ConversionTool } from "../types";

export const conversionTools: ConversionTool[] = [
  {
    name: "FFmpeg",
    command: "ffmpeg",
    supportedInputs: ["mp4", "mov", "avi", "mkv", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "3gp", "mp3", "wav", "flac", "ogg", "m4a", "wma", "aac"],
    supportedOutputs: ["mp4", "mov", "avi", "mkv", "webm", "mp3", "wav", "flac", "ogg", "m4a", "aac", "gif"],
  },
  // DISABLED: Pandoc functionality temporarily disabled
  // {
  //   name: "Pandoc",
  //   command: "pandoc",
  //   supportedInputs: ["md", "markdown", "txt", "html", "htm", "docx", "odt", "rtf", "tex", "latex", "epub", "rst"],
  //   supportedOutputs: ["md", "html", "pdf", "docx", "odt", "rtf", "tex", "epub", "txt"],
  // },
  {
    name: "LibreOffice",
    command: "libreoffice",
    supportedInputs: ["doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf"],
    supportedOutputs: ["pdf", "html", "txt", "docx", "odt", "rtf"],
  },
  {
    name: "ImageMagick",
    command: "imagemagick",
    supportedInputs: [
      // Standard/Common formats
      "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
      // Modern formats
      "heic", "heif", "avif", "jxl",
      // Professional/High-end formats
      "tga", "exr", "hdr", "dpx", "psd", "psb",
      // Legacy/Specialized formats
      "pcx", "ico", "sgi", "sun", "pfm", "ppm", "pgm", "pbm", "pam",
      // X Window System formats
      "xbm", "xpm", "xwd",
      // Gaming formats
      "dds",
      // Vector formats (rasterized on input)
      "svg", "svgz",
      // Animation formats
      "apng",
      // GIMP format
      "xcf",
      // Windows formats
      "cur", "emf", "wmf",
      // Digital camera RAW formats
      "raw", "arw", "cr2", "cr3", "crw", "dng", "nef", "nrw", "orf", "raf", "rw2", "rwl", "srw",
      // JPEG 2000
      "j2k", "jp2",
    ],
    supportedOutputs: [
      // Standard/Common formats
      "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
      // Modern formats
      "heic", "heif", "avif", "jxl",
      // Document format
      "pdf",
      // Professional formats
      "tga", "exr", "hdr", "psd",
      // Animation formats
      "apng",
      // Legacy formats
      "ico", "pcx", "cur",
      // Raw/Uncompressed
      "ppm", "pgm", "pbm", "pam",
      // X Window System
      "xbm", "xpm", "xwd",
      // Gaming
      "dds",
      // JPEG 2000
      "j2k", "jp2",
    ],
  },
];

export function getAvailableOutputFormats(inputExtension: string): string[] {
  const formats = new Set<string>();
  
  for (const tool of conversionTools) {
    if (tool.supportedInputs.includes(inputExtension.toLowerCase())) {
      tool.supportedOutputs.forEach(format => {
        if (format !== inputExtension.toLowerCase()) {
          formats.add(format);
        }
      });
    }
  }
  
  return Array.from(formats);
}

export function getToolForConversion(inputExt: string, outputExt: string): ConversionTool | null {
  for (const tool of conversionTools) {
    if (tool.supportedInputs.includes(inputExt.toLowerCase()) && 
        tool.supportedOutputs.includes(outputExt.toLowerCase())) {
      return tool;
    }
  }
  return null;
}
