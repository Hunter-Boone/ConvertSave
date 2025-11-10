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
    supportedInputs: ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp", "svg", "ico", "heic", "raw"],
    supportedOutputs: ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "pdf", "ico"],
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
