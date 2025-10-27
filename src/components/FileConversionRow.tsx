import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { FileInfo } from "../types";
import ConversionOptions from "./ConversionOptions";

interface FileConversionRowProps {
  file: FileInfo;
  index: number;
  onFormatChange: (index: number, format: string) => void;
  onRemove: (index: number) => void;
  formatFileSize: (bytes: number) => string;
}

export default function FileConversionRow({
  file,
  index,
  onFormatChange,
  onRemove,
  formatFileSize,
}: FileConversionRowProps) {
  const [showOptions, setShowOptions] = useState(false);
  const [thumbnailSrc, setThumbnailSrc] = useState<string | null>(null);

  const handleFormatSelect = (format: string) => {
    onFormatChange(index, format);
    setShowOptions(false);
  };

  // Check if file is an image that can be thumbnailed
  const imageExtensions = [
    "jpg",
    "jpeg",
    "png",
    "gif",
    "webp",
    "bmp",
    "svg",
    "ico",
    "tiff",
    "heic",
    "heif",
    "avif",
  ];
  const isImage = imageExtensions.includes(file.extension.toLowerCase());

  useEffect(() => {
    if (!isImage) {
      setThumbnailSrc(null);
      return;
    }

    // Load thumbnail using backend command
    const loadThumbnail = async () => {
      try {
        const dataUrl = await invoke<string>("get_thumbnail", {
          filePath: file.path,
        });
        setThumbnailSrc(dataUrl);
      } catch (error) {
        console.error("Error loading thumbnail:", error);
        setThumbnailSrc(null);
      }
    };

    loadThumbnail();
  }, [file.path, isImage]);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between p-4 bg-white rounded-xl border-2 border-dark-purple">
        <div className="flex items-center space-x-4">
          <div className="w-12 h-12 bg-light-grey rounded-lg flex items-center justify-center overflow-hidden">
            {thumbnailSrc ? (
              <img
                src={thumbnailSrc}
                alt={file.name}
                className="w-full h-full object-cover"
                onError={() => {
                  console.error("Thumbnail failed to load for:", file.path);
                  setThumbnailSrc(null);
                }}
              />
            ) : (
              <svg
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                className="text-light-purple"
              >
                <rect
                  x="3"
                  y="6"
                  width="15"
                  height="12"
                  rx="2"
                  stroke="currentColor"
                  strokeWidth="2"
                />
                <circle
                  cx="7.5"
                  cy="10.5"
                  r="1.5"
                  stroke="currentColor"
                  strokeWidth="2"
                />
                <path
                  d="M15 15l-3-3-4.5 4.5"
                  stroke="currentColor"
                  strokeWidth="2"
                />
              </svg>
            )}
          </div>
          <div className="space-y-1">
            <p className="font-bold text-dark-purple">{file.name}</p>
            <p className="text-sm text-light-purple">
              {formatFileSize(file.size)} • {file.extension.toUpperCase()}
              {file.selectedFormat && (
                <span className="ml-2 px-2 py-0.5 bg-aquamarine text-dark-purple rounded text-xs font-bold">
                  → {file.selectedFormat.toUpperCase()}
                </span>
              )}
            </p>
          </div>
        </div>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => setShowOptions(!showOptions)}
            className="btn-chunky bg-light-grey text-dark-purple px-3 py-1 text-sm hover:bg-tan"
          >
            {file.selectedFormat
              ? `Convert to ${file.selectedFormat.toUpperCase()}`
              : "Choose Format"}
          </button>
          <button
            onClick={() => onRemove(index)}
            className="p-1 text-dark-purple hover:bg-light-grey rounded"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z" />
            </svg>
          </button>
        </div>
      </div>

      {showOptions && (
        <div className="ml-16 mr-16">
          <ConversionOptions
            inputFile={file}
            selectedFormat={file.selectedFormat || ""}
            onFormatSelect={handleFormatSelect}
          />
        </div>
      )}
    </div>
  );
}
