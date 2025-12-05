import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { X } from "lucide-react";
import ToolDownloader from "./components/ToolDownloader";
import LicenseActivation from "./components/LicenseActivation";
import { CustomSelect } from "./components/CustomSelect";
import { FileInfo } from "./types";

// License status type from Rust
interface LicenseStatus {
  isValid: boolean;
  isActivated: boolean;
  planType: "monthly" | "yearly" | "lifetime" | null;
  daysRemaining: number | null;
  inGracePeriod: boolean;
  error: string | null;
  requiresActivation: boolean;
  productKey: string | null;
}

// FileItem component with thumbnail support
function FileItem({
  file,
  index,
  onRemove,
  formatFileSize,
}: {
  file: FileInfo;
  index: number;
  onRemove: (index: number) => void;
  formatFileSize: (bytes: number) => string;
}) {
  const [thumbnailSrc, setThumbnailSrc] = useState<string | null>(null);

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
    <div className="flex items-center justify-between p-4 bg-white border-2 border-dark-purple rounded-xl">
      <div className="flex items-center space-x-4">
        <div className="w-12 h-12 bg-muted-bg rounded-lg flex items-center justify-center overflow-hidden flex-shrink-0">
          {thumbnailSrc ? (
            <img
              src={thumbnailSrc}
              alt={file.name}
              className="w-full h-full object-cover"
              onError={() => setThumbnailSrc(null)}
            />
          ) : (
            <svg
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              className="text-secondary"
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
          <p className="text-sm text-secondary">
            {formatFileSize(file.size)} • {file.extension.toUpperCase()}
          </p>
        </div>
      </div>
      <button
        onClick={() => onRemove(index)}
        className="w-8 h-8 flex items-center justify-center rounded-lg border-2 border-dark-purple hover:bg-pink-accent transition-colors flex-shrink-0"
        aria-label="Remove file"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path
            d="M2 2L14 14M14 2L2 14"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
          />
        </svg>
      </button>
    </div>
  );
}

interface ToolStatus {
  ffmpeg: {
    available: boolean;
    path: string | null;
  };
  // DISABLED: Pandoc functionality temporarily disabled
  // pandoc: {
  //   available: boolean;
  //   path: string | null;
  // };
  imagemagick: {
    available: boolean;
    path: string | null;
  };
}

function App() {
  // License state
  const [licenseStatus, setLicenseStatus] = useState<LicenseStatus | null>(
    null
  );
  const [licenseChecked, setLicenseChecked] = useState(false);

  const [selectedFiles, setSelectedFiles] = useState<FileInfo[]>([]);
  const [selectedFormat, setSelectedFormat] = useState<string>("");
  const [advancedOptions] = useState<string>("");
  const [outputDirectory] = useState<string>("");
  const [isConverting, setIsConverting] = useState(false);
  const [conversionProgress, setConversionProgress] = useState(0);
  const [conversionResult, setConversionResult] = useState<{
    success: boolean;
    message: string;
  } | null>(null);
  const [toolsReady, setToolsReady] = useState<boolean | null>(null);
  const [showToolManager, setShowToolManager] = useState(false);
  const [toolSetupDismissed, setToolSetupDismissed] = useState(false);
  const [availableFormats, setAvailableFormats] = useState<string[]>([]);
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const isProcessingDrop = useRef(false);

  // Check license on startup
  useEffect(() => {
    const checkLicense = async () => {
      try {
        const status = await invoke<LicenseStatus>("check_license_status");
        setLicenseStatus(status);
      } catch (err) {
        console.error("Failed to check license:", err);
        // Assume needs activation if check fails
        setLicenseStatus({
          isValid: false,
          isActivated: false,
          planType: null,
          daysRemaining: null,
          inGracePeriod: false,
          error: "Failed to check license status",
          requiresActivation: true,
          productKey: null,
        });
      } finally {
        setLicenseChecked(true);
      }
    };

    checkLicense();
  }, []);

  // Handle successful license activation
  const handleLicenseActivated = (status: LicenseStatus) => {
    setLicenseStatus(status);
  };

  useEffect(() => {
    const checkForAppUpdates = async () => {
      try {
        const hasUpdate = await invoke<boolean>("check_app_update");
        if (hasUpdate) {
          setUpdateAvailable(true);
        }
      } catch (error) {
        console.error("Failed to check for app updates:", error);
      }
    };

    checkForAppUpdates();
    // ========== PLATFORM DETECTION (COMMENTED OUT FOR NATIVE DECORATIONS) ==========
    // Uncomment this code if you restore the custom title bar
    // const userAgent = navigator.userAgent.toLowerCase();
    // if (userAgent.includes("mac")) {
    //   setCurrentPlatform("macos");
    // } else if (userAgent.includes("linux")) {
    //   setCurrentPlatform("linux");
    // } else {
    //   setCurrentPlatform("windows");
    // }
    // ========== END PLATFORM DETECTION ==========

    // Check if tools are ready
    checkToolsStatus();

    // Set up Tauri file drop listener using window API
    let unlisten: (() => void) | undefined;

    const setupFileDropListener = async () => {
      const handleFileDrop = async (paths: string[]) => {
        if (isProcessingDrop.current) {
          console.warn("Already processing a drop, ignoring duplicate event");
          return;
        }

        isProcessingDrop.current = true;
        console.log("File drop - processing", paths.length, "file(s)");

        if (paths.length === 0) {
          console.warn("No files in drop event");
          isProcessingDrop.current = false;
          return;
        }

        // Get file info for all dropped files
        const fileInfos = await Promise.all(
          paths.map(async (filePath) => {
            try {
              const stats = (await invoke("get_file_info", {
                path: filePath,
              })) as {
                name: string;
                size: number;
                extension: string;
              };

              console.log("Got file stats:", stats);

              return {
                name: stats.name,
                path: filePath,
                size: stats.size,
                extension: stats.extension,
              };
            } catch (error) {
              console.error("Error getting file info:", error);
              // Fallback if we can't get file info
              const fileName =
                String(filePath).split(/[\\/]/).pop() || "Unknown";
              const extension = (fileName.split(".").pop() || "").toLowerCase();

              return {
                name: fileName,
                path: filePath,
                size: 0,
                extension: extension,
              };
            }
          })
        );

        console.log("Adding files:", fileInfos);
        setSelectedFiles((prev) => {
          // Get existing file paths for deduplication
          const existingPaths = new Set(prev.map((f) => f.path));
          // Only add files that aren't already in the list
          const newFiles = fileInfos.filter(
            (file) => !existingPaths.has(file.path)
          );
          return [...prev, ...newFiles];
        });

        // Reset processing flag after a short delay to prevent rapid duplicates
        setTimeout(() => {
          isProcessingDrop.current = false;
        }, 100);
      };

      // Listen to drag hover event (when dragging over the window)
      const unlistenHover = await listen<{
        paths: string[];
        position: { x: number; y: number };
      }>("tauri://drag-enter", () => {
        console.log("Drag enter - showing overlay");
        setIsDraggingOver(true);
      });

      // Listen to the drag-drop event (when files are dropped)
      const unlistenDrop = await listen<{
        paths: string[];
        position: { x: number; y: number };
      }>("tauri://drag-drop", async (event) => {
        console.log("Drop event - processing files");
        setIsDraggingOver(false);
        // Extract paths from the payload object
        if (event.payload && event.payload.paths) {
          await handleFileDrop(event.payload.paths);
        }
      });

      // Listen to drag leave event (when drag leaves without dropping)
      const unlistenLeave = await listen("tauri://drag-leave", () => {
        console.log("Drag leave - hiding overlay");
        setIsDraggingOver(false);
      });

      console.log("Listening to drag events");

      // Chain all unlisteners
      unlisten = () => {
        unlistenHover();
        unlistenDrop();
        unlistenLeave();
      };
    };

    setupFileDropListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  // Load available formats based on selected files
  useEffect(() => {
    const loadFormats = async () => {
      if (selectedFiles.length === 0) {
        // Clear formats and selected format when no files are selected
        setAvailableFormats([]);
        setSelectedFormat("");
        return;
      }

      try {
        // Get unique extensions from selected files
        const uniqueExtensions = Array.from(
          new Set(selectedFiles.map((file) => file.extension.toLowerCase()))
        );

        // Get available formats for each unique extension
        const formatsByExtension = await Promise.all(
          uniqueExtensions.map(async (ext) => {
            try {
              const formats = await invoke<any[]>("get_available_formats", {
                inputExtension: ext,
              });
              return formats.map((f) => f.format);
            } catch (error) {
              console.error(`Failed to load formats for ${ext}:`, error);
              return [];
            }
          })
        );

        // Use union of all formats (additive approach)
        if (formatsByExtension.length === 0) {
          setAvailableFormats([]);
          return;
        }

        // Combine all formats from all file types
        const allFormats = Array.from(new Set(formatsByExtension.flat()));

        // Sort formats by popularity - most popular formats first
        const formatPopularity: Record<string, number> = {
          // Images - most popular first
          jpg: 1,
          jpeg: 2,
          png: 3,
          webp: 4,
          gif: 5,
          svg: 6,
          bmp: 7,
          tiff: 8,
          tif: 9,
          ico: 10,
          heic: 11,
          raw: 12,

          // Videos
          mp4: 20,
          webm: 21,
          mov: 22,
          avi: 23,
          mkv: 24,
          flv: 25,
          wmv: 26,
          m4v: 27,
          mpg: 28,
          mpeg: 29,
          "3gp": 30,

          // Audio
          mp3: 40,
          wav: 41,
          flac: 42,
          ogg: 43,
          m4a: 44,
          aac: 45,
          wma: 46,

          // Documents
          pdf: 60,
          docx: 61,
          doc: 62,
          txt: 63,
          rtf: 64,
          odt: 65,
          html: 66,
          htm: 67,
          epub: 68,

          // Spreadsheets
          xlsx: 80,
          xls: 81,
          csv: 82,
          ods: 83,

          // Presentations
          pptx: 100,
          ppt: 101,
          odp: 102,
        };

        // Sort formats: known formats by popularity, unknown formats at the end alphabetically
        const sortedFormats = allFormats.sort((a, b) => {
          const aPopularity = formatPopularity[a.toLowerCase()] ?? 9999;
          const bPopularity = formatPopularity[b.toLowerCase()] ?? 9999;

          if (aPopularity !== bPopularity) {
            return aPopularity - bPopularity;
          }

          // If both have same popularity (or both unknown), sort alphabetically
          return a.localeCompare(b);
        });

        // Check if we have multiple image files - if so, add PDF (Multipage) option
        const imageExtensions = [
          "jpg",
          "jpeg",
          "png",
          "gif",
          "bmp",
          "webp",
          "svg",
          "ico",
          "tiff",
          "tif",
          "heic",
          "heif",
          "avif",
          "jxl",
          "tga",
          "exr",
          "hdr",
          "psd",
          "psb",
        ];
        const imageFiles = selectedFiles.filter((f) =>
          imageExtensions.includes(f.extension.toLowerCase())
        );

        // Add pdf-multipage option if there are multiple image files and PDF is available
        if (imageFiles.length > 1 && sortedFormats.includes("pdf")) {
          // Find position after regular PDF and insert pdf-multipage
          const pdfIndex = sortedFormats.indexOf("pdf");
          if (pdfIndex !== -1) {
            sortedFormats.splice(pdfIndex + 1, 0, "pdf-multipage");
          }
        }

        setAvailableFormats(sortedFormats);

        // Update selected format if current one is not available or empty
        if (sortedFormats.length === 0) {
          setSelectedFormat("");
        } else if (!selectedFormat || !sortedFormats.includes(selectedFormat)) {
          setSelectedFormat(sortedFormats[0]);
        }
      } catch (error) {
        console.error("Failed to load formats:", error);
      }
    };

    loadFormats();
  }, [selectedFiles]);

  // Auto-hide success message after 5 seconds
  useEffect(() => {
    if (conversionResult?.success) {
      const timer = setTimeout(() => {
        setConversionResult(null);
      }, 5000);
      return () => clearTimeout(timer);
    }
  }, [conversionResult]);

  // Clear message when files change (add/remove)
  useEffect(() => {
    if (conversionResult) {
      setConversionResult(null);
    }
  }, [selectedFiles]);

  const checkToolsStatus = async () => {
    try {
      const status = await invoke<ToolStatus>("check_tools_status");
      // Core tools required: At least one tool must be installed (FFmpeg or ImageMagick)
      const allReady = status.ffmpeg.available || status.imagemagick.available;
      setToolsReady(allReady);
    } catch (err) {
      console.error("Failed to check tool status:", err);
      setToolsReady(false);
    }
  };

  const handleUpdateApp = async () => {
    try {
      await invoke("install_app_update");
    } catch (error) {
      console.error("Failed to update app:", error);
      alert(`Update failed: ${error}`);
    }
  };

  const handleToolsReady = () => {
    setShowToolManager(false);
    setToolSetupDismissed(true);
    // Re-check status to update the UI properly
    checkToolsStatus();
  };

  const removeFile = (index: number) => {
    setSelectedFiles((prev) => prev.filter((_, i) => i !== index));
  };

  const clearAllFiles = () => {
    setSelectedFiles([]);
    setConversionResult(null);
  };

  const handleConvert = async () => {
    if (selectedFiles.length === 0 || !selectedFormat) return;

    setIsConverting(true);
    setConversionProgress(0);
    setConversionResult(null);

    try {
      // Handle multipage PDF conversion specially
      if (selectedFormat === "pdf-multipage") {
        try {
          const inputPaths = selectedFiles.map((f) => f.path);
          await invoke<string>("convert_images_to_multipage_pdf", {
            inputPaths,
            outputDirectory: outputDirectory || undefined,
          });
          setConversionResult({
            success: true,
            message: `Successfully created multipage PDF from ${selectedFiles.length} images!`,
          });
        } catch (error) {
          console.error("Failed to create multipage PDF:", error);
          setConversionResult({
            success: false,
            message: `Failed to create multipage PDF: ${error}`,
          });
        }
        setConversionProgress(100);
        setIsConverting(false);
        return;
      }

      let successCount = 0;
      let failureCount = 0;
      let firstErrorMessage = "";

      // Convert each file to the selected format
      for (let i = 0; i < selectedFiles.length; i++) {
        const file = selectedFiles[i];

        try {
          await invoke<string>("convert_file", {
            inputPath: file.path,
            outputFormat: selectedFormat,
            outputDirectory: outputDirectory || undefined,
            advancedOptions: advancedOptions || undefined,
          });
          successCount++;
        } catch (error) {
          console.error(`Failed to convert ${file.name}:`, error);
          failureCount++;
          // Store the first error message to show to the user
          if (!firstErrorMessage) {
            const errorString = String(error);
            if (errorString.includes("system cannot find the file")) {
              firstErrorMessage =
                "Failed to convert all files. File not found.";
            } else {
              firstErrorMessage = errorString;
            }
          }
        }

        setConversionProgress(((i + 1) / selectedFiles.length) * 100);
      }

      if (failureCount === 0) {
        setConversionResult({
          success: true,
          message: `Successfully converted ${successCount} file(s) to ${selectedFormat.toUpperCase()}!`,
        });
      } else if (successCount > 0) {
        setConversionResult({
          success: true,
          message: `Converted ${successCount} file(s), ${failureCount} failed. Error: ${firstErrorMessage}`,
        });
      } else {
        // If we have a specific error message (like file not found), use it
        // Otherwise fallback to generic message
        setConversionResult({
          success: false,
          message:
            firstErrorMessage === "Failed to convert all files. File not found."
              ? firstErrorMessage
              : firstErrorMessage ||
                `Failed to convert all files. Some formats may not support conversion to ${selectedFormat.toUpperCase()}.`,
        });
      }
    } catch (error) {
      setConversionResult({
        success: false,
        message: `Conversion failed: ${error}`,
      });
    } finally {
      setIsConverting(false);
      setConversionProgress(100);
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  };

  // ========== CUSTOM TITLE BAR CODE (COMMENTED OUT FOR NATIVE DECORATIONS) ==========
  // Uncomment these functions and the title bar JSX below to restore custom window controls

  // const handleMinimize = async () => {
  //   const window = getCurrentWindow();
  //   await window.minimize();
  // };

  // const handleMaximize = async () => {
  //   const window = getCurrentWindow();
  //   await window.toggleMaximize();
  // };

  // const handleClose = async () => {
  //   const window = getCurrentWindow();
  //   await window.close();
  // };
  // ========== END CUSTOM TITLE BAR CODE ==========

  const handleBrowseFiles = async () => {
    try {
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: "All Files",
            extensions: ["*"],
          },
          {
            name: "Images",
            extensions: [
              "png",
              "jpg",
              "jpeg",
              "gif",
              "bmp",
              "webp",
              "svg",
              "ico",
              "tiff",
              "heic",
              "raw",
            ],
          },
          {
            name: "Videos",
            extensions: [
              "mp4",
              "avi",
              "mov",
              "wmv",
              "flv",
              "webm",
              "mkv",
              "3gp",
              "ogv",
            ],
          },
          {
            name: "Audio",
            extensions: [
              "mp3",
              "wav",
              "flac",
              "aac",
              "ogg",
              "wma",
              "m4a",
              "opus",
            ],
          },
          {
            name: "Documents",
            extensions: ["pdf", "doc", "docx", "txt", "rtf", "odt", "pages"],
          },
        ],
      });

      if (selected && Array.isArray(selected)) {
        // Convert the selected files to FileInfo objects
        const fileInfos: FileInfo[] = await Promise.all(
          selected.map(async (filePath) => {
            try {
              // Get file stats using Tauri's filesystem API
              const stats = (await invoke("get_file_info", {
                path: filePath,
              })) as {
                name: string;
                size: number;
                extension: string;
              };

              return {
                name: stats.name,
                path: filePath,
                size: stats.size,
                extension: stats.extension,
              };
            } catch (error) {
              // Fallback if we can't get file info
              const fileName =
                String(filePath).split(/[\\/]/).pop() || "Unknown";
              const extension = (fileName.split(".").pop() || "").toLowerCase();

              return {
                name: fileName,
                path: filePath,
                size: 0,
                extension: extension,
              };
            }
          })
        );

        setSelectedFiles((prev) => {
          // Get existing file paths for deduplication
          const existingPaths = new Set(prev.map((f) => f.path));
          // Only add files that aren't already in the list
          const newFiles = fileInfos.filter(
            (file) => !existingPaths.has(file.path)
          );
          return [...prev, ...newFiles];
        });
      } else if (selected && typeof selected === "string") {
        // Handle single file selection
        try {
          const stats = (await invoke("get_file_info", { path: selected })) as {
            name: string;
            size: number;
            extension: string;
          };

          const fileInfo: FileInfo = {
            name: stats.name,
            path: selected,
            size: stats.size,
            extension: stats.extension,
          };

          setSelectedFiles((prev) => {
            // Check if file already exists in the list
            if (prev.some((f) => f.path === fileInfo.path)) {
              return prev; // Don't add duplicate
            }
            return [...prev, fileInfo];
          });
        } catch (error) {
          // Fallback for single file
          const fileName = String(selected).split(/[\\/]/).pop() || "Unknown";
          const extension = (fileName.split(".").pop() || "").toLowerCase();

          const fileInfo: FileInfo = {
            name: fileName,
            path: selected,
            size: 0,
            extension: extension,
          };

          setSelectedFiles((prev) => {
            // Check if file already exists in the list
            if (prev.some((f) => f.path === fileInfo.path)) {
              return prev; // Don't add duplicate
            }
            return [...prev, fileInfo];
          });
        }
      }
    } catch (error) {
      console.error("Error opening file dialog:", error);
    }
  };

  // ========== CUSTOM TITLE BAR COMPONENTS (COMMENTED OUT FOR NATIVE DECORATIONS) ==========
  // Uncomment these components to restore platform-specific custom window controls

  // Platform-specific window control components
  // const MacOSControls = () => (
  //   <div className="flex items-center space-x-2">
  //     <button
  //       onClick={handleClose}
  //       className="w-3 h-3 bg-pink rounded-full hover:bg-red-500 transition-colors focus:outline-none"
  //       aria-label="Close"
  //     ></button>
  //     <button
  //       onClick={handleMinimize}
  //       className="w-3 h-3 bg-yellow rounded-full hover:bg-yellow-400 transition-colors focus:outline-none"
  //       aria-label="Minimize"
  //     ></button>
  //     <button
  //       onClick={handleMaximize}
  //       className="w-3 h-3 bg-aquamarine rounded-full border border-dark-purple hover:bg-green-400 transition-colors focus:outline-none"
  //       aria-label="Maximize"
  //     ></button>
  //   </div>
  // );

  // const WindowsControls = () => (
  //   <div className="flex items-center">
  //     <button
  //       onClick={handleMinimize}
  //       className="w-12 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
  //       aria-label="Minimize"
  //     >
  //       <svg
  //         width="10"
  //         height="1"
  //         viewBox="0 0 10 1"
  //         fill="currentColor"
  //         className="text-dark-purple"
  //       >
  //         <rect width="10" height="1" />
  //       </svg>
  //     </button>
  //     <button
  //       onClick={handleMaximize}
  //       className="w-12 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
  //       aria-label="Maximize"
  //     >
  //       <svg
  //         width="10"
  //         height="10"
  //         viewBox="0 0 10 10"
  //         fill="none"
  //         className="text-dark-purple"
  //       >
  //         <rect
  //           x="0"
  //           y="0"
  //           width="10"
  //           height="10"
  //           stroke="currentColor"
  //           strokeWidth="1"
  //           fill="none"
  //         />
  //       </svg>
  //     </button>
  //     <button
  //       onClick={handleClose}
  //       className="w-12 h-8 hover:bg-red-500 hover:text-white flex items-center justify-center transition-colors focus:outline-none"
  //       aria-label="Close"
  //     >
  //       <svg
  //         width="10"
  //         height="10"
  //         viewBox="0 0 10 10"
  //         fill="none"
  //         className="stroke-current"
  //       >
  //         <path d="M1 1L9 9M9 1L1 9" stroke="currentColor" strokeWidth="1" />
  //       </svg>
  //     </button>
  //   </div>
  // );

  // const LinuxControls = () => (
  //   <div className="flex items-center">
  //     <button
  //       onClick={handleMinimize}
  //       className="w-8 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
  //       aria-label="Minimize"
  //     >
  //       <svg
  //         width="12"
  //         height="2"
  //         viewBox="0 0 12 2"
  //         fill="currentColor"
  //         className="text-dark-purple"
  //       >
  //         <rect width="12" height="2" />
  //       </svg>
  //     </button>
  //     <button
  //       onClick={handleMaximize}
  //       className="w-8 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
  //       aria-label="Maximize"
  //     >
  //       <svg
  //         width="12"
  //         height="12"
  //         viewBox="0 0 12 12"
  //         fill="none"
  //         className="text-dark-purple"
  //       >
  //         <rect
  //           x="1"
  //           y="1"
  //           width="10"
  //           height="10"
  //           stroke="currentColor"
  //           strokeWidth="1.5"
  //           fill="none"
  //         />
  //       </svg>
  //     </button>
  //     <button
  //       onClick={handleClose}
  //       className="w-8 h-8 hover:bg-red-500 hover:text-white flex items-center justify-center transition-colors focus:outline-none"
  //       aria-label="Close"
  //     >
  //       <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
  //         <path
  //           d="M2 2L10 10M10 2L2 10"
  //           stroke="currentColor"
  //           strokeWidth="1.5"
  //         />
  //       </svg>
  //     </button>
  //   </div>
  // );
  // ========== END CUSTOM TITLE BAR COMPONENTS ==========

  // Show loading state while checking license
  if (!licenseChecked) {
    return (
      <div className="h-screen bg-light-bg flex items-center justify-center">
        <div className="text-center space-y-4">
          <div className="w-16 h-16 border-4 border-mint-accent border-t-transparent rounded-full animate-spin mx-auto"></div>
          <p className="text-dark-purple font-bold">Checking license...</p>
        </div>
      </div>
    );
  }

  // Show license activation screen if not licensed
  if (!licenseStatus?.isValid || licenseStatus?.requiresActivation) {
    return (
      <LicenseActivation
        onActivated={handleLicenseActivated}
        initialError={licenseStatus?.error || undefined}
      />
    );
  }

  // Show tool downloader if tools aren't ready (and user hasn't dismissed it) OR if user manually opens tool manager
  if ((toolsReady === false && !toolSetupDismissed) || showToolManager) {
    return (
      <ToolDownloader
        onAllToolsReady={handleToolsReady}
        productKey={licenseStatus?.productKey ?? null}
        onProductKeyChanged={(newKey) => {
          if (licenseStatus) {
            setLicenseStatus({ ...licenseStatus, productKey: newKey });
          }
        }}
      />
    );
  }

  // Show loading state while checking tools
  if (toolsReady === null) {
    return (
      <div className="h-screen bg-light-bg flex items-center justify-center">
        <div className="text-center space-y-4">
          <div className="w-16 h-16 border-4 border-mint-accent border-t-transparent rounded-full animate-spin mx-auto"></div>
          <p className="text-dark-purple font-bold">Loading ConvertSave...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen bg-light-bg flex flex-col overflow-hidden relative">
      {/* Drag Overlay - Shows when dragging files over window */}
      {isDraggingOver && (
        <div className="absolute inset-0 z-[100] bg-mint-accent bg-opacity-20 border-4 border-dashed border-mint-accent flex items-center justify-center pointer-events-none">
          <div className="bg-white rounded-2xl p-8 shadow-2xl">
            <div className="flex flex-col items-center space-y-4">
              <div className="w-20 h-20 bg-mint-accent rounded-full flex items-center justify-center animate-bounce">
                <svg
                  width="48"
                  height="48"
                  viewBox="0 0 48 48"
                  fill="none"
                  className="text-dark-purple"
                >
                  <path
                    d="M24 8v24M24 32l-8-8M24 32l8-8"
                    stroke="currentColor"
                    strokeWidth="3"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  />
                  <path
                    d="M10 40h28"
                    stroke="currentColor"
                    strokeWidth="3"
                    strokeLinecap="round"
                  />
                </svg>
              </div>
              <h2 className="text-2xl font-bold text-dark-purple">
                Drop files here
              </h2>
              <p className="text-secondary">
                Release to add files for conversion
              </p>
            </div>
          </div>
        </div>
      )}

      {/* ========== CUSTOM TITLE BAR (COMMENTED OUT FOR NATIVE DECORATIONS) ========== */}
      {/* Uncomment this entire section to restore the custom title bar with colored controls */}
      {/*
      <div
        className="bg-aquamarine px-4 py-2 flex items-center justify-between select-none flex-shrink-0 z-50"
        data-tauri-drag-region
      >
        <div className="flex items-center space-x-4">
          {currentPlatform === "macos" && <MacOSControls />}
          {currentPlatform !== "macos" && (
            <div className="text-dark-purple font-bold text-sm">
              ConvertSave
            </div>
          )}
        </div>

        {currentPlatform === "macos" && (
          <div className="text-dark-purple font-bold text-sm">ConvertSave</div>
        )}

        <div className="flex items-center space-x-2">
          <button
            onClick={() => setShowToolManager(true)}
            className="btn-chunky bg-light-purple text-dark-purple px-3 py-1 text-sm hover:bg-opacity-80"
            title="Manage conversion tools"
          >
            Tools Manager
          </button>
          <button className="btn-chunky bg-yellow text-dark-purple px-3 py-1 text-sm">
            Update Available
          </button>
          <button className="p-1 text-dark-purple hover:bg-dark-purple hover:bg-opacity-10 rounded">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 4.5a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4.5z" />
            </svg>
          </button>
          {currentPlatform === "windows" && <WindowsControls />}
          {currentPlatform === "linux" && <LinuxControls />}
        </div>
      </div>
      */}
      {/* ========== END CUSTOM TITLE BAR ========== */}

      {/* Toolbar */}
      <div className="bg-light-bg p-6 flex items-center justify-between flex-shrink-0">
        {/* Left side - Settings button */}
        <div className="flex items-center space-x-3">
          <button
            onClick={() => setShowToolManager(true)}
            className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-6 py-2 hover:bg-light-bg"
            title="Manage conversion tools"
          >
            Settings
          </button>

          {updateAvailable && (
            <button
              onClick={handleUpdateApp}
              className="btn-chunky bg-pink border-2 border-dark-purple text-dark-purple px-6 py-2 hover:bg-opacity-80"
              title="A new version is available!"
            >
              Update Application
            </button>
          )}
        </div>

        {/* Right side - Format dropdown and Convert button */}
        <div className="flex items-center space-x-3">
          <span className="text-secondary font-normal">Select Output:</span>

          <CustomSelect
            value={selectedFormat}
            onChange={setSelectedFormat}
            options={availableFormats}
            disabled={
              selectedFiles.length === 0 || availableFormats.length === 0
            }
            placeholder="No Formats"
          />

          <button
            onClick={handleConvert}
            disabled={
              selectedFiles.length === 0 || isConverting || !selectedFormat
            }
            className={`btn-chunky border-2 border-dark-purple px-8 py-2 ${
              selectedFiles.length === 0 || isConverting || !selectedFormat
                ? "bg-lighter-bg border-secondary text-secondary cursor-not-allowed"
                : "bg-mint-accent text-dark-purple hover:bg-opacity-80"
            }`}
          >
            {isConverting ? "Converting..." : "Convert"}
          </button>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto overflow-x-hidden">
        <div
          className={`px-6 pb-6 space-y-6 ${
            selectedFiles.length === 0 ? "h-full flex flex-col" : ""
          }`}
        >
          {/* Show drag zone when no files are selected, or a smaller version when files are present */}
          {selectedFiles.length === 0 ? (
            /* Main Drop Zone - Full Size */
            <div className="flex-1 border-2 border-dashed border-secondary rounded-xl p-16 text-center bg-lighter-bg flex items-center justify-center">
              <div className="space-y-6">
                <div className="w-20 h-20 mx-auto bg-muted-bg rounded-lg flex items-center justify-center">
                  <svg
                    width="40"
                    height="40"
                    viewBox="0 0 40 40"
                    fill="none"
                    className="text-secondary"
                  >
                    <rect
                      x="5"
                      y="10"
                      width="25"
                      height="20"
                      rx="2"
                      stroke="currentColor"
                      strokeWidth="2"
                    />
                    <circle
                      cx="12"
                      cy="17"
                      r="2"
                      stroke="currentColor"
                      strokeWidth="2"
                    />
                    <path
                      d="M25 25l-5-5-7.5 7.5"
                      stroke="currentColor"
                      strokeWidth="2"
                    />
                  </svg>
                </div>
                <div className="space-y-3">
                  <h2 className="text-2xl font-bold text-dark-purple">
                    Drop your files here to convert.
                  </h2>
                  <p className="text-lg text-secondary">
                    Supports most common image filetypes.
                  </p>
                  <button
                    onClick={handleBrowseFiles}
                    className="btn-chunky bg-mint-accent border-2 border-dark-purple text-dark-purple px-8 py-3 text-lg hover:bg-opacity-80"
                  >
                    Browse
                  </button>
                </div>
              </div>
            </div>
          ) : (
            /* Compact Drop Zone */
            <div className="border-2 border-dashed border-secondary rounded-xl p-10 text-center bg-lighter-bg">
              <div className="flex items-center justify-center space-x-6">
                <div className="w-20 h-20 bg-muted-bg rounded-xl flex items-center justify-center">
                  <svg
                    width="40"
                    height="40"
                    viewBox="0 0 24 24"
                    fill="none"
                    className="text-secondary"
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
                </div>
                <div>
                  <p className="font-bold text-dark-purple text-lg">
                    Add more files.
                  </p>
                  <button
                    onClick={handleBrowseFiles}
                    className="text-sm text-dark-purple hover:text-secondary font-bold border-2 border-dark-purple rounded-lg px-4 py-1.5 mt-2"
                  >
                    Browse
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* File List */}
          {selectedFiles.length > 0 && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h2 className="text-xl font-bold text-primary">Files</h2>
                <button
                  onClick={clearAllFiles}
                  className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 hover:bg-light-bg"
                >
                  Clear All
                </button>
              </div>

              <div className="space-y-3">
                {selectedFiles.map((file, index) => (
                  <FileItem
                    key={index}
                    file={file}
                    index={index}
                    onRemove={removeFile}
                    formatFileSize={formatFileSize}
                  />
                ))}
              </div>
            </div>
          )}

          {/* Progress Bar */}
          {isConverting && (
            <div className="w-full bg-lighter-bg rounded-full h-2">
              <div
                className="bg-mint-accent h-2 rounded-full transition-all duration-300"
                style={{ width: `${conversionProgress}%` }}
              />
            </div>
          )}

          {/* Results */}
          {conversionResult && (
            <div
              className={`
              p-6 rounded-xl font-normal relative
              ${
                conversionResult.success
                  ? "bg-mint-accent text-dark-purple"
                  : "bg-pink-accent text-dark-purple"
              }
            `}
            >
              <button
                onClick={() => setConversionResult(null)}
                className="absolute top-4 right-4 w-6 h-6 flex items-center justify-center hover:bg-dark-purple hover:bg-opacity-10 rounded transition-colors"
                aria-label="Dismiss"
              >
                <X className="w-5 h-5 text-dark-purple" />
              </button>
              <p className="font-bold text-lg pr-8">
                {conversionResult.message}
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
