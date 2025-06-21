import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { FileInfo, ConversionOption } from "../types";
import { Check } from "lucide-react";

interface ConversionOptionsProps {
  inputFile: FileInfo;
  selectedFormat: string;
  onFormatSelect: (format: string) => void;
}

const colorMap: Record<string, string> = {
  pdf: "bg-pink",
  mp4: "bg-aquamarine",
  mp3: "bg-yellow",
  jpg: "bg-tan",
  png: "bg-light-purple",
  txt: "bg-light-tan",
  docx: "bg-pink",
  epub: "bg-aquamarine",
  webp: "bg-yellow",
  wav: "bg-tan",
  gif: "bg-light-purple",
  html: "bg-light-tan",
};

export default function ConversionOptions({
  inputFile,
  selectedFormat,
  onFormatSelect,}: ConversionOptionsProps) {
  const [availableFormats, setAvailableFormats] = useState<ConversionOption[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadFormats = async () => {
      setLoading(true);
      try {
        const formats = await invoke<ConversionOption[]>("get_available_formats", {
          inputExtension: inputFile.extension,
        });
        setAvailableFormats(formats);
      } catch (error) {
        console.error("Failed to load formats:", error);
        setAvailableFormats([]);
      } finally {
        setLoading(false);
      }
    };

    loadFormats();
  }, [inputFile]);

  if (loading) {
    return (
      <div className="text-center py-8">
        <p className="text-secondary">Loading available formats...</p>
      </div>
    );
  }
  if (availableFormats.length === 0) {
    return (
      <div className="text-center py-8">
        <p className="text-secondary">
          No conversion options available for .{inputFile.extension} files
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <h2 className="text-xl font-bold text-primary">Convert To:</h2>
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
        {availableFormats.map((format) => {
          const isSelected = selectedFormat === format.format;
          const bgColor = colorMap[format.format] || "bg-tan";
          
          return (
            <button
              key={format.format}
              onClick={() => onFormatSelect(format.format)}
              className={`
                btn-chunky relative p-4 text-center
                ${isSelected ? bgColor : "bg-light-grey"}
                ${isSelected ? "text-dark-purple" : "text-secondary"}
              `}
            >
              {isSelected && (
                <Check className="absolute top-2 right-2 w-5 h-5" />
              )}
              <p className="font-bold text-lg uppercase">{format.format}</p>
              <p className="text-xs mt-1 opacity-75">{format.displayName}</p>
            </button>
          );
        })}
      </div>
    </div>
  );
}