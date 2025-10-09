import { useState } from "react";
import { FileInfo, type BatchConversionSettings } from "../types";
import ConversionOptions from "./ConversionOptions";

interface BatchConversionSettingsProps {
  files: FileInfo[];
  batchSettings: BatchConversionSettings;
  onBatchSettingChange: (inputExtension: string, format: string) => void;
}

export default function BatchConversionSettings({
  files,
  batchSettings,
  onBatchSettingChange,
}: BatchConversionSettingsProps) {
  const [expandedSections, setExpandedSections] = useState<Set<string>>(
    new Set()
  );

  // Group files by their extension
  const filesByExtension = files.reduce((acc, file) => {
    const ext = file.extension.toLowerCase();
    if (!acc[ext]) {
      acc[ext] = [];
    }
    acc[ext].push(file);
    return acc;
  }, {} as Record<string, FileInfo[]>);

  const toggleSection = (extension: string) => {
    const newExpanded = new Set(expandedSections);
    if (newExpanded.has(extension)) {
      newExpanded.delete(extension);
    } else {
      newExpanded.add(extension);
    }
    setExpandedSections(newExpanded);
  };

  const handleBatchFormatSelect = (extension: string, format: string) => {
    onBatchSettingChange(extension, format);
    setExpandedSections((prev) => {
      const newSet = new Set(prev);
      newSet.delete(extension);
      return newSet;
    });
  };

  const getDisplayFormat = (extension: string) => {
    const setting = batchSettings[extension];
    if (!setting) return "Choose Format";
    if (setting.isMixed) return "Mixed Formats";
    return setting.format.toUpperCase();
  };

  const getButtonStyle = (extension: string) => {
    const setting = batchSettings[extension];
    if (!setting) return "bg-light-grey text-dark-purple";
    if (setting.isMixed) return "bg-orange text-dark-purple";
    return "bg-aquamarine text-dark-purple";
  };

  if (Object.keys(filesByExtension).length === 0) {
    return null;
  }

  return (
    <div className="space-y-4">
      <h2 className="text-xl font-bold text-primary">
        Batch Conversion Settings
      </h2>
      <p className="text-sm text-secondary">
        Set conversion formats for all files of the same type at once
      </p>

      <div className="space-y-3">
        {Object.entries(filesByExtension).map(([extension, extensionFiles]) => {
          const isExpanded = expandedSections.has(extension);
          const fileCount = extensionFiles.length;

          return (
            <div key={extension} className="space-y-2">
              <div className="flex items-center justify-between p-4 bg-white rounded-xl border-2 border-dark-purple">
                <div className="flex items-center space-x-4">
                  <div className="w-10 h-10 bg-light-grey rounded-lg flex items-center justify-center">
                    <span className="text-xs font-bold text-dark-purple">
                      {extension.toUpperCase()}
                    </span>
                  </div>
                  <div>
                    <p className="font-bold text-dark-purple">
                      {extension.toUpperCase()} Files
                    </p>
                    <p className="text-sm text-light-purple">
                      {fileCount} file{fileCount !== 1 ? "s" : ""}
                    </p>
                  </div>
                </div>

                <div className="flex items-center space-x-2">
                  <button
                    onClick={() => toggleSection(extension)}
                    className={`btn-chunky px-4 py-2 text-sm ${getButtonStyle(
                      extension
                    )}`}
                  >
                    {getDisplayFormat(extension)}
                  </button>
                </div>
              </div>

              {isExpanded && extensionFiles.length > 0 && (
                <div className="ml-14 mr-4">
                  <ConversionOptions
                    inputFile={extensionFiles[0]}
                    selectedFormat={batchSettings[extension]?.format || ""}
                    onFormatSelect={(format) =>
                      handleBatchFormatSelect(extension, format)
                    }
                  />
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
