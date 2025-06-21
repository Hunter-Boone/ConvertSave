import { useState, useCallback } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { FileIcon, Upload } from "lucide-react";
import { FileInfo } from "../types";
import { getFileExtension, formatFileSize } from "../lib/utils";

interface DropZoneProps {
  onFileSelect: (file: FileInfo) => void;
  selectedFile: FileInfo | null;
}

export default function DropZone({ onFileSelect, selectedFile }: DropZoneProps) {
  const [isDragging, setIsDragging] = useState(false);

  const handleDragEnter = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const files = Array.from(e.dataTransfer.files);
    if (files.length > 0) {
      const file = files[0];
      const fileInfo: FileInfo = {
        name: file.name,
        path: file.path || file.name,
        size: file.size,
        extension: getFileExtension(file.name),
      };
      onFileSelect(fileInfo);
    }
  }, [onFileSelect]);

  const handleClick = async () => {
    const selected = await open({
      multiple: false,
      directory: false,
    });

    if (selected && typeof selected === "string") {
      const fileName = selected.split(/[\\/]/).pop() || "Unknown file";
      const fileInfo: FileInfo = {
        name: fileName,
        path: selected,
        size: 0,
        extension: getFileExtension(fileName),
      };
      onFileSelect(fileInfo);
    }
  };

  const clearFile = () => {
    onFileSelect(null as any);
  };

  return (
    <div
      className={`
        drop-zone p-12 text-center cursor-pointer
        ${isDragging ? "drop-zone-active" : "border-light-grey"}
        ${selectedFile ? "bg-surface" : "bg-light-tan"}
      `}
      onDragEnter={handleDragEnter}
      onDragLeave={handleDragLeave}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
      onClick={!selectedFile ? handleClick : undefined}
    >
      {selectedFile ? (
        <div className="space-y-4">
          <FileIcon className="w-16 h-16 mx-auto text-secondary" />
          <div>
            <p className="text-lg font-bold text-primary">
              {selectedFile.name}
            </p>
            {selectedFile.size > 0 && (
              <p className="text-sm text-secondary font-normal">
                {formatFileSize(selectedFile.size)}
              </p>
            )}
          </div>
          <button
            onClick={(e) => {
              e.stopPropagation();
              clearFile();
            }}
            className="text-sm text-secondary hover:text-primary underline font-normal"
          >
            Remove file
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          <Upload className="w-16 h-16 mx-auto text-secondary" />
          <div>
            <p className="text-lg font-bold text-primary">
              Drag & Drop File Here
            </p>
            <p className="text-sm text-secondary font-normal">or click to browse</p>
          </div>
        </div>
      )}
    </div>
  );
}
