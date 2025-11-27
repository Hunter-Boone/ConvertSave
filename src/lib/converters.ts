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
      "tga", "exr", "hdr", "dpx", "psd",
      // Legacy/Specialized formats
      "pcx", "ico", "sgi",
      // Vector formats (rasterized on input)
      "svg",
      // Digital camera RAW formats
      "raw", "arw", "cr2", "cr3", "dng", "nef", "orf", "raf", "rw2",
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
      // Legacy formats
      "ico", "pcx",
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
