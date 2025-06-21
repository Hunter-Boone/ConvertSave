import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { ChevronDown, ChevronUp, FolderOpen } from "lucide-react";

interface AdvancedSettingsProps {
  advancedOptions: string;
  onOptionsChange: (options: string) => void;
  outputDirectory: string;
  onDirectoryChange: (directory: string) => void;
}

export default function AdvancedSettings({
  advancedOptions,
  onOptionsChange,
  outputDirectory,
  onDirectoryChange,
}: AdvancedSettingsProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const handleSelectDirectory = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected && typeof selected === "string") {
      onDirectoryChange(selected);
    }
  };

  return (
    <div className="space-y-4">
      {/* Output Directory */}
      <div className="space-y-2">
        <label className="block text-sm font-bold text-primary">
          Output Folder:
        </label>
        <div className="flex gap-2">
          <input
            type="text"
            value={outputDirectory}
            readOnly
            placeholder="Default: Documents/ConvertSave/Converted"
            className="flex-1 px-4 py-2 bg-surface border border-light-grey rounded-lg text-primary font-normal"
          />
          <button
            onClick={handleSelectDirectory}
            className="btn-chunky px-4 py-2 bg-yellow text-dark-purple"
          >
            <FolderOpen className="w-5 h-5" />
          </button>
        </div>
      </div>
      {/* Advanced Options Toggle */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="flex items-center gap-2 text-sm text-secondary hover:text-primary transition-colors font-normal"
      >
        {isExpanded ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
        Advanced Options
      </button>

      {/* Advanced Options Content */}
      {isExpanded && (
        <div className="space-y-2 p-4 bg-tan rounded-lg">
          <label className="block text-sm font-bold text-primary">
            Custom Command Flags:
          </label>
          <textarea
            value={advancedOptions}
            onChange={(e) => onOptionsChange(e.target.value)}
            placeholder="e.g., -b:v 2M -preset fast for ffmpeg"
            className="w-full px-4 py-2 bg-surface border border-light-grey rounded-lg text-primary font-mono text-sm"
            rows={3}
          />
          <p className="text-xs text-secondary font-normal">
            Enter command-line arguments for the conversion tool. These will be passed directly to the underlying converter.
          </p>
        </div>
      )}
    </div>
  );
}