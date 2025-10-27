import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
// import { getCurrentWindow } from "@tauri-apps/api/window"; // Uncomment for custom title bar
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import FileConversionRow from "./components/FileConversionRow";
import BatchConversionSettings from "./components/BatchConversionSettings";
import ToolDownloader from "./components/ToolDownloader";
import {
  FileInfo,
  type BatchConversionSettings as BatchSettings,
} from "./types";
import { ChevronDown } from "lucide-react";

interface ToolStatus {
  ffmpeg: {
    available: boolean;
    path: string | null;
  };
  pandoc: {
    available: boolean;
    path: string | null;
  };
  imagemagick: {
    available: boolean;
    path: string | null;
  };
}

function App() {
  const [selectedFiles, setSelectedFiles] = useState<FileInfo[]>([]);
  const [batchSettings, setBatchSettings] = useState<BatchSettings>({});
  const [advancedOptions] = useState<string>("");
  const [outputDirectory] = useState<string>("");
  const [isConverting, setIsConverting] = useState(false);
  const [conversionProgress, setConversionProgress] = useState(0);
  const [conversionResult, setConversionResult] = useState<{
    success: boolean;
    message: string;
    outputPath?: string;
  } | null>(null);
  // const [currentPlatform, setCurrentPlatform] = useState<string>("windows"); // Uncomment for custom title bar
  const [isIndividualSettingsExpanded, setIsIndividualSettingsExpanded] =
    useState(false);
  const [toolsReady, setToolsReady] = useState<boolean | null>(null);
  const [showToolManager, setShowToolManager] = useState(false);
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const isProcessingDrop = useRef(false);

  useEffect(() => {
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
        setSelectedFiles((prev) => [...prev, ...fileInfos]);

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

  const checkToolsStatus = async () => {
    try {
      const status = await invoke<ToolStatus>("check_tools_status");
      // Core tools required: FFmpeg and Pandoc
      // ImageMagick is optional (only needed for HEIC encoding)
      const allReady = status.ffmpeg.available && status.pandoc.available;
      setToolsReady(allReady);
    } catch (err) {
      console.error("Failed to check tool status:", err);
      setToolsReady(false);
    }
  };

  const handleToolsReady = () => {
    setShowToolManager(false);
    // Re-check status to update the UI properly
    checkToolsStatus();
  };

  // Update batch settings when files change
  useEffect(() => {
    const newBatchSettings: BatchSettings = {};

    // Group files by extension and check for mixed formats
    const filesByExtension = selectedFiles.reduce((acc, file) => {
      const ext = file.extension.toLowerCase();
      if (!acc[ext]) {
        acc[ext] = [];
      }
      acc[ext].push(file);
      return acc;
    }, {} as Record<string, FileInfo[]>);

    Object.entries(filesByExtension).forEach(([extension, extensionFiles]) => {
      const formats = extensionFiles
        .map((f) => f.selectedFormat)
        .filter((f) => f !== undefined);

      if (formats.length === 0) {
        // No formats selected yet
        return;
      }

      const uniqueFormats = Array.from(new Set(formats));

      if (uniqueFormats.length === 1) {
        // All files have the same format
        newBatchSettings[extension] = {
          format: uniqueFormats[0]!,
          isMixed: false,
        };
      } else {
        // Mixed formats
        newBatchSettings[extension] = {
          format: uniqueFormats[0]!, // Use first format as default
          isMixed: true,
        };
      }
    });

    setBatchSettings(newBatchSettings);
  }, [selectedFiles]);

  const removeFile = (index: number) => {
    setSelectedFiles((prev) => prev.filter((_, i) => i !== index));
  };

  const handleFileFormatChange = (index: number, format: string) => {
    setSelectedFiles((prev) =>
      prev.map((file, i) =>
        i === index ? { ...file, selectedFormat: format } : file
      )
    );
    // Batch settings will be updated automatically via useEffect
  };

  const handleBatchSettingChange = (inputExtension: string, format: string) => {
    // Update all files of this extension
    setSelectedFiles((prev) =>
      prev.map((file) =>
        file.extension.toLowerCase() === inputExtension.toLowerCase()
          ? { ...file, selectedFormat: format }
          : file
      )
    );
    // Update batch settings
    setBatchSettings((prev) => ({
      ...prev,
      [inputExtension.toLowerCase()]: {
        format: format,
        isMixed: false,
      },
    }));
  };

  const handleConvert = async () => {
    // Check if all files have selected formats
    const filesWithFormats = selectedFiles.filter(
      (file) => file.selectedFormat
    );
    if (filesWithFormats.length === 0) return;

    setIsConverting(true);
    setConversionProgress(0);
    setConversionResult(null);

    try {
      let lastOutputPath = "";
      // Convert each file with its selected format
      for (let i = 0; i < filesWithFormats.length; i++) {
        const file = filesWithFormats[i];
        const result = await invoke<string>("convert_file", {
          inputPath: file.path,
          outputFormat: file.selectedFormat!,
          outputDirectory: outputDirectory || undefined,
          advancedOptions: advancedOptions || undefined,
        });
        lastOutputPath = result;
        setConversionProgress(((i + 1) / filesWithFormats.length) * 100);
      }

      setConversionResult({
        success: true,
        message: `Successfully converted ${selectedFiles.length} file(s)!`,
        outputPath: lastOutputPath,
      });
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

        setSelectedFiles((prev) => [...prev, ...fileInfos]);
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

          setSelectedFiles((prev) => [...prev, fileInfo]);
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

          setSelectedFiles((prev) => [...prev, fileInfo]);
        }
      }
    } catch (error) {
      console.error("Error opening file dialog:", error);
    }
  };

  const openOutputFolder = async () => {
    if (conversionResult?.outputPath) {
      try {
        await invoke("open_folder", { path: conversionResult.outputPath });
      } catch (error) {
        console.error("Failed to open folder:", error);
      }
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

  // Show tool downloader if tools aren't ready OR if user manually opens tool manager
  if (toolsReady === false || showToolManager) {
    return <ToolDownloader onAllToolsReady={handleToolsReady} />;
  }

  // Show loading state while checking
  if (toolsReady === null) {
    return (
      <div className="h-screen bg-off-white flex items-center justify-center">
        <div className="text-center space-y-4">
          <div className="w-16 h-16 border-4 border-aquamarine border-t-transparent rounded-full animate-spin mx-auto"></div>
          <p className="text-dark-purple font-bold">Loading ConvertSave...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen bg-off-white flex flex-col overflow-hidden relative">
      {/* Drag Overlay - Shows when dragging files over window */}
      {isDraggingOver && (
        <div className="absolute inset-0 z-[100] bg-aquamarine bg-opacity-20 border-4 border-dashed border-aquamarine flex items-center justify-center pointer-events-none">
          <div className="bg-white rounded-2xl p-8 shadow-2xl">
            <div className="flex flex-col items-center space-y-4">
              <div className="w-20 h-20 bg-aquamarine rounded-full flex items-center justify-center animate-bounce">
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
              <p className="text-light-purple">
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

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto overflow-x-hidden">
        <div
          className={`${
            selectedFiles.length === 0
              ? "h-full flex flex-col p-6"
              : "p-6 space-y-6"
          }`}
        >
          {/* Show drag zone when no files are selected, or a smaller version when files are present */}
          {selectedFiles.length === 0 ? (
            /* Main Drop Zone - Full Size */
            <div className="flex-1 border-2 border-dashed border-light-purple rounded-xl p-16 text-center bg-white flex items-center justify-center">
              <div className="space-y-6">
                <div className="w-20 h-20 mx-auto bg-light-grey rounded-lg flex items-center justify-center">
                  <svg
                    width="40"
                    height="40"
                    viewBox="0 0 40 40"
                    fill="none"
                    className="text-light-purple"
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
                    Drop your files here to convert
                  </h2>
                  <p className="text-lg text-light-purple">
                    Support for images, videos, audio, and documents
                  </p>
                  <button
                    onClick={handleBrowseFiles}
                    className="btn-chunky bg-aquamarine text-dark-purple px-8 py-3 text-lg hover:bg-opacity-80"
                  >
                    Choose Files
                  </button>
                </div>
              </div>
            </div>
          ) : (
            /* Compact Drop Zone */
            <div className="border-2 border-dashed border-light-purple rounded-xl p-6 text-center bg-white">
              <div className="flex items-center justify-center space-x-4">
                <div className="w-12 h-12 bg-light-grey rounded-lg flex items-center justify-center">
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
                </div>
                <div>
                  <p className="font-bold text-dark-purple">Add more files</p>
                  <button
                    onClick={handleBrowseFiles}
                    className="text-sm text-light-purple hover:text-dark-purple underline"
                  >
                    Browse files
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Batch Conversion Settings */}
          {selectedFiles.length > 0 && (
            <BatchConversionSettings
              files={selectedFiles}
              batchSettings={batchSettings}
              onBatchSettingChange={handleBatchSettingChange}
            />
          )}

          {/* Individual File Conversion Options */}
          {selectedFiles.length > 0 && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <h2 className="text-xl font-bold text-primary">
                    Individual File Settings
                  </h2>
                  <p className="text-sm text-secondary">
                    Customize conversion settings for each file individually
                  </p>
                </div>
                <button
                  onClick={() =>
                    setIsIndividualSettingsExpanded(
                      !isIndividualSettingsExpanded
                    )
                  }
                  className="btn-chunky bg-light-grey text-dark-purple px-4 py-2 flex items-center space-x-2 hover:bg-tan"
                >
                  <span>
                    {isIndividualSettingsExpanded ? "Hide" : "Show"} Files (
                    {selectedFiles.length})
                  </span>
                  <ChevronDown
                    className={`w-4 h-4 transition-transform duration-200 ${
                      isIndividualSettingsExpanded ? "rotate-180" : ""
                    }`}
                  />
                </button>
              </div>

              {isIndividualSettingsExpanded && (
                <div className="space-y-3">
                  {selectedFiles.map((file, index) => (
                    <FileConversionRow
                      key={index}
                      file={file}
                      index={index}
                      onFormatChange={handleFileFormatChange}
                      onRemove={removeFile}
                      formatFileSize={formatFileSize}
                    />
                  ))}
                </div>
              )}
            </div>
          )}

          {/* Convert Button */}
          {selectedFiles.length > 0 && (
            <div className="text-center space-y-2">
              <button
                onClick={handleConvert}
                disabled={isConverting}
                className="btn-chunky bg-aquamarine text-dark-purple px-8 py-4 text-lg"
              >
                {isConverting
                  ? "Converting..."
                  : `Convert ${selectedFiles.length} file(s)`}
              </button>
            </div>
          )}

          {/* Progress Bar */}
          {isConverting && (
            <div className="w-full bg-light-grey rounded-full h-2">
              <div
                className="bg-aquamarine h-2 rounded-full transition-all duration-300"
                style={{ width: `${conversionProgress}%` }}
              />
            </div>
          )}

          {/* Results */}
          {conversionResult && (
            <div
              className={`
              p-4 rounded-xl font-normal text-center space-y-3
              ${
                conversionResult.success
                  ? "bg-aquamarine text-dark-purple"
                  : "bg-pink text-dark-purple"
              }
            `}
            >
              <p className="font-bold">{conversionResult.message}</p>
              {conversionResult.success && conversionResult.outputPath && (
                <>
                  <p className="text-sm">
                    Output: {conversionResult.outputPath}
                  </p>
                  <button
                    onClick={openOutputFolder}
                    className="btn-chunky bg-dark-purple text-off-white px-4 py-2 text-sm"
                  >
                    Open Output Folder
                  </button>
                </>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
