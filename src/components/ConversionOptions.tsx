import { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { FileInfo, ConversionOption } from "../types";
import { Check, ChevronDown } from "lucide-react";

interface ConversionOptionsProps {
  inputFile: FileInfo;
  selectedFormat: string;
  onFormatSelect: (format: string) => void;
}

const colorMap: Record<string, string> = {
  // Standard formats
  pdf: "bg-pink",
  mp4: "bg-aquamarine",
  mp3: "bg-yellow",
  jpg: "bg-yellow",
  jpeg: "bg-yellow",
  png: "bg-orange",
  txt: "bg-light-tan",
  docx: "bg-pink",
  epub: "bg-aquamarine",
  webp: "bg-green",
  wav: "bg-tan",
  gif: "bg-blue",
  html: "bg-light-tan",
  bmp: "bg-light-purple",
  tiff: "bg-light-tan",

  // Professional/High-end formats
  tga: "bg-pink",
  exr: "bg-aquamarine",
  hdr: "bg-aquamarine",
  dpx: "bg-pink",
  pfm: "bg-aquamarine",

  // JPEG 2000
  j2k: "bg-yellow",
  jp2: "bg-yellow",

  // Legacy/Specialized formats
  pcx: "bg-light-purple",
  ico: "bg-blue",
  sgi: "bg-green",
  sun: "bg-orange",

  // Raw/Uncompressed formats
  ppm: "bg-light-tan",
  pgm: "bg-light-tan",
  pbm: "bg-light-tan",
  pam: "bg-light-tan",

  // X Window System formats
  xbm: "bg-light-purple",
  xpm: "bg-light-purple",
  xwd: "bg-light-purple",

  // Gaming/3D formats
  dds: "bg-blue",
};

export default function ConversionOptions({
  inputFile,
  selectedFormat,
  onFormatSelect,
}: ConversionOptionsProps) {
  const [availableFormats, setAvailableFormats] = useState<ConversionOption[]>(
    []
  );
  const [loading, setLoading] = useState(true);
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const [dropdownPosition, setDropdownPosition] = useState<"below" | "above">(
    "below"
  );
  const dropdownRef = useRef<HTMLDivElement>(null);
  const buttonRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    const loadFormats = async () => {
      setLoading(true);
      try {
        const formats = await invoke<ConversionOption[]>(
          "get_available_formats",
          {
            inputExtension: inputFile.extension,
          }
        );
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

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsDropdownOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  // Scroll dropdown into view when opened
  useEffect(() => {
    if (isDropdownOpen && dropdownRef.current) {
      const dropdownElement = dropdownRef.current.querySelector(
        '[class*="absolute"]'
      ) as HTMLElement;
      if (dropdownElement) {
        setTimeout(() => {
          dropdownElement.scrollIntoView({
            behavior: "smooth",
            block: "nearest",
            inline: "nearest",
          });
        }, 10);
      }
    }
  }, [isDropdownOpen]);

  const selectedFormatData = availableFormats.find(
    (f) => f.format === selectedFormat
  );
  const selectedBgColor = selectedFormatData
    ? colorMap[selectedFormatData.format] || "bg-tan"
    : "bg-light-grey";

  const handleFormatSelect = (format: string) => {
    onFormatSelect(format);
    setIsDropdownOpen(false);
  };

  const toggleDropdown = () => {
    if (!isDropdownOpen && buttonRef.current) {
      // Calculate if there's enough space below
      const buttonRect = buttonRef.current.getBoundingClientRect();
      const viewportHeight = window.innerHeight;

      // Calculate estimated dropdown height based on number of formats
      const itemsPerRow = Math.min(
        6,
        Math.max(3, Math.floor(window.innerWidth / 120))
      ); // Responsive column count
      const rows = Math.ceil(availableFormats.length / itemsPerRow);
      const estimatedHeight = Math.min(384, rows * 70 + 32); // Each item ~70px + padding

      const spaceBelow = viewportHeight - buttonRect.bottom - 20; // Extra margin
      const spaceAbove = buttonRect.top - 20; // Extra margin

      // Position above if there's not enough space below but enough above
      if (spaceBelow < estimatedHeight && spaceAbove > estimatedHeight) {
        setDropdownPosition("above");
      } else {
        setDropdownPosition("below");
      }
    }
    setIsDropdownOpen(!isDropdownOpen);
  };

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

      <div className="relative" ref={dropdownRef}>
        {/* Dropdown Button */}
        <button
          ref={buttonRef}
          onClick={toggleDropdown}
          className={`
            btn-chunky w-full p-4 flex items-center justify-between
            ${selectedFormatData ? selectedBgColor : "bg-light-grey"}
            ${selectedFormatData ? "text-dark-purple" : "text-secondary"}
          `}
        >
          <div className="flex items-center space-x-3">
            {selectedFormatData && <Check className="w-5 h-5" />}
            <div className="text-left">
              <p className="font-bold text-lg uppercase">
                {selectedFormatData?.format || "Select Format"}
              </p>
              {selectedFormatData && (
                <p className="text-xs opacity-75">
                  {selectedFormatData.display_name}
                </p>
              )}
            </div>
          </div>
          <ChevronDown
            className={`w-5 h-5 transition-transform duration-200 ${
              isDropdownOpen ? "rotate-180" : ""
            }`}
          />
        </button>

        {/* Dropdown Grid */}
        {isDropdownOpen && (
          <div
            className={`absolute left-0 right-0 z-50 bg-white border-2 border-dark-purple rounded-lg shadow-lg max-h-96 overflow-y-auto ${
              dropdownPosition === "above"
                ? "bottom-full mb-2"
                : "top-full mt-2"
            }`}
          >
            <div className="p-4">
              <div className="grid grid-cols-3 sm:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-2">
                {availableFormats.map((format) => {
                  const isSelected = selectedFormat === format.format;
                  const bgColor = colorMap[format.format] || "bg-tan";

                  return (
                    <button
                      key={format.format}
                      onClick={() => handleFormatSelect(format.format)}
                      className={`
                        btn-chunky relative p-2 text-center transition-all duration-200
                        ${isSelected ? bgColor : "bg-light-grey hover:bg-tan"}
                        ${
                          isSelected
                            ? "text-dark-purple"
                            : "text-secondary hover:text-dark-purple"
                        }
                      `}
                    >
                      {isSelected && (
                        <Check className="absolute top-0.5 right-0.5 w-3 h-3" />
                      )}
                      <p className="font-bold text-sm uppercase">
                        {format.format}
                      </p>
                      <p className="text-xs mt-0.5 opacity-75">
                        {format.display_name}
                      </p>
                    </button>
                  );
                })}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
