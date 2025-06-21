import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import DropZone from "./components/DropZone";
import ConversionOptions from "./components/ConversionOptions";
import AdvancedSettings from "./components/AdvancedSettings";
import { FileInfo } from "./types";

function App() {
  const [selectedFiles, setSelectedFiles] = useState<FileInfo[]>([]);
  const [selectedFormat, setSelectedFormat] = useState<string>("JPG");
  const [advancedOptions, setAdvancedOptions] = useState<string>("");
  const [outputDirectory, setOutputDirectory] = useState<string>("");
  const [isConverting, setIsConverting] = useState(false);
  const [conversionProgress, setConversionProgress] = useState(0);
  const [conversionResult, setConversionResult] = useState<{
    success: boolean;
    message: string;
    outputPath?: string;
  } | null>(null);
  const [currentPlatform, setCurrentPlatform] = useState<string>("windows");

  useEffect(() => {
    // Detect platform using user agent as a fallback
    const userAgent = navigator.userAgent.toLowerCase();
    if (userAgent.includes("mac")) {
      setCurrentPlatform("macos");
    } else if (userAgent.includes("linux")) {
      setCurrentPlatform("linux");
    } else {
      setCurrentPlatform("windows");
    }
  }, []);

  const handleFileSelect = (file: FileInfo) => {
    setSelectedFiles((prev) => [...prev, file]);
    setConversionResult(null);
  };

  const handleFilesSelect = (files: FileInfo[]) => {
    setSelectedFiles((prev) => [...prev, ...files]);
    setConversionResult(null);
  };

  const removeFile = (index: number) => {
    setSelectedFiles((prev) => prev.filter((_, i) => i !== index));
  };

  const handleConvert = async () => {
    if (selectedFiles.length === 0 || !selectedFormat) return;

    setIsConverting(true);
    setConversionProgress(0);
    setConversionResult(null);

    try {
      // Convert each file
      for (let i = 0; i < selectedFiles.length; i++) {
        const file = selectedFiles[i];
        await invoke("convert_file", {
          inputPath: file.path,
          outputFormat: selectedFormat,
          outputDirectory: outputDirectory || undefined,
          advancedOptions: advancedOptions || undefined,
        });
        setConversionProgress(((i + 1) / selectedFiles.length) * 100);
      }

      setConversionResult({
        success: true,
        message: `Successfully converted ${selectedFiles.length} file(s)!`,
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

  const handleMinimize = async () => {
    const window = getCurrentWindow();
    await window.minimize();
  };

  const handleMaximize = async () => {
    const window = getCurrentWindow();
    await window.toggleMaximize();
  };

  const handleClose = async () => {
    const window = getCurrentWindow();
    await window.close();
  };

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
              const extension = fileName.split(".").pop() || "";

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
          const extension = fileName.split(".").pop() || "";

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

  // Platform-specific window control components
  const MacOSControls = () => (
    <div className="flex items-center space-x-2">
      <button
        onClick={handleClose}
        className="w-3 h-3 bg-pink rounded-full hover:bg-red-500 transition-colors focus:outline-none"
        aria-label="Close"
      ></button>
      <button
        onClick={handleMinimize}
        className="w-3 h-3 bg-yellow rounded-full hover:bg-yellow-400 transition-colors focus:outline-none"
        aria-label="Minimize"
      ></button>
      <button
        onClick={handleMaximize}
        className="w-3 h-3 bg-aquamarine rounded-full border border-dark-purple hover:bg-green-400 transition-colors focus:outline-none"
        aria-label="Maximize"
      ></button>
    </div>
  );

  const WindowsControls = () => (
    <div className="flex items-center">
      <button
        onClick={handleMinimize}
        className="w-12 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
        aria-label="Minimize"
      >
        <svg
          width="10"
          height="1"
          viewBox="0 0 10 1"
          fill="currentColor"
          className="text-dark-purple"
        >
          <rect width="10" height="1" />
        </svg>
      </button>
      <button
        onClick={handleMaximize}
        className="w-12 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
        aria-label="Maximize"
      >
        <svg
          width="10"
          height="10"
          viewBox="0 0 10 10"
          fill="none"
          className="text-dark-purple"
        >
          <rect
            x="0"
            y="0"
            width="10"
            height="10"
            stroke="currentColor"
            strokeWidth="1"
            fill="none"
          />
        </svg>
      </button>
      <button
        onClick={handleClose}
        className="w-12 h-8 hover:bg-red-500 hover:text-white flex items-center justify-center transition-colors focus:outline-none"
        aria-label="Close"
      >
        <svg
          width="10"
          height="10"
          viewBox="0 0 10 10"
          fill="none"
          className="stroke-current"
        >
          <path d="M1 1L9 9M9 1L1 9" stroke="currentColor" strokeWidth="1" />
        </svg>
      </button>
    </div>
  );

  const LinuxControls = () => (
    <div className="flex items-center">
      <button
        onClick={handleMinimize}
        className="w-8 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
        aria-label="Minimize"
      >
        <svg
          width="12"
          height="2"
          viewBox="0 0 12 2"
          fill="currentColor"
          className="text-dark-purple"
        >
          <rect width="12" height="2" />
        </svg>
      </button>
      <button
        onClick={handleMaximize}
        className="w-8 h-8 hover:bg-dark-purple hover:bg-opacity-10 flex items-center justify-center transition-colors focus:outline-none"
        aria-label="Maximize"
      >
        <svg
          width="12"
          height="12"
          viewBox="0 0 12 12"
          fill="none"
          className="text-dark-purple"
        >
          <rect
            x="1"
            y="1"
            width="10"
            height="10"
            stroke="currentColor"
            strokeWidth="1.5"
            fill="none"
          />
        </svg>
      </button>
      <button
        onClick={handleClose}
        className="w-8 h-8 hover:bg-red-500 hover:text-white flex items-center justify-center transition-colors focus:outline-none"
        aria-label="Close"
      >
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <path
            d="M2 2L10 10M10 2L2 10"
            stroke="currentColor"
            strokeWidth="1.5"
          />
        </svg>
      </button>
    </div>
  );

  return (
    <div className="h-screen bg-off-white flex flex-col overflow-hidden">
      {/* Custom Title Bar */}
      <div
        className="bg-aquamarine px-4 py-2 flex items-center justify-between select-none flex-shrink-0 z-50"
        data-tauri-drag-region
      >
        {/* Left side - Controls on macOS, Title on Windows/Linux */}
        <div className="flex items-center space-x-4">
          {currentPlatform === "macos" && <MacOSControls />}
          {currentPlatform !== "macos" && (
            <div className="text-dark-purple font-bold text-sm">
              ConvertSave
            </div>
          )}
        </div>

        {/* Center - Title on macOS */}
        {currentPlatform === "macos" && (
          <div className="text-dark-purple font-bold text-sm">ConvertSave</div>
        )}

        {/* Right side - Always has Update button, Controls on Windows/Linux */}
        <div className="flex items-center space-x-2">
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

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto overflow-x-hidden">
        <div className="p-6 space-y-6">
          {/* Show drag zone when no files are selected, or a smaller version when files are present */}
          {selectedFiles.length === 0 ? (
            /* Main Drop Zone - Full Size */
            <div className="border-2 border-dashed border-light-purple rounded-xl p-16 text-center bg-white">
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

          {/* Show conversion options only when files are present */}
          {selectedFiles.length > 0 && (
            <>
              {/* Convert To Section */}
              <div className="text-center space-y-4">
                <div className="flex items-center justify-center">
                  <span className="text-dark-purple font-normal text-lg">
                    Convert to
                  </span>
                </div>
                <select
                  value={selectedFormat}
                  onChange={(e) => setSelectedFormat(e.target.value)}
                  className="btn-chunky bg-yellow text-dark-purple px-6 py-3 border-2 border-dark-purple rounded-xl font-bold text-lg appearance-none outline-none focus:outline-none focus:ring-0 focus:border-dark-purple"
                  style={{ outline: "none", boxShadow: "none" }}
                >
                  <option value="JPG">JPG</option>
                  <option value="PNG">PNG</option>
                  <option value="WEBP">WEBP</option>
                  <option value="PDF">PDF</option>
                  <option value="SVG">SVG</option>
                  <option value="BMP">BMP</option>
                  <option value="TIFF">TIFF</option>
                  <option value="MP4">MP4</option>
                  <option value="AVI">AVI</option>
                  <option value="MOV">MOV</option>
                  <option value="MP3">MP3</option>
                  <option value="WAV">WAV</option>
                  <option value="FLAC">FLAC</option>
                </select>
              </div>
            </>
          )}

          {/* File List */}
          {selectedFiles.length > 0 && (
            <div className="space-y-2">
              <h3 className="text-lg font-bold text-dark-purple">Images</h3>
              <div className="space-y-2">
                {selectedFiles.map((file, index) => (
                  <div
                    key={index}
                    className="flex items-center justify-between p-4 bg-white rounded-xl border-2 border-dark-purple"
                  >
                    <div className="flex items-center space-x-4">
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
                      <div className="space-y-1">
                        <p className="font-bold text-dark-purple">
                          {file.name}
                        </p>
                        <p className="text-sm text-light-purple">
                          {formatFileSize(file.size)} •{" "}
                          {file.extension.toUpperCase()}
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center space-x-2">
                      <select className="btn-chunky bg-yellow text-dark-purple px-3 py-1 text-sm border-2 border-dark-purple rounded-lg">
                        <option value="JPG">JPG</option>
                        <option value="PNG">PNG</option>
                        <option value="WEBP">WEBP</option>
                      </select>
                      <button
                        onClick={() => removeFile(index)}
                        className="p-1 text-dark-purple hover:bg-light-grey rounded"
                      >
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 16 16"
                          fill="currentColor"
                        >
                          <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z" />
                        </svg>
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Convert Button */}
          {selectedFiles.length > 0 && (
            <div className="text-center">
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
              p-4 rounded-xl font-normal text-center
              ${
                conversionResult.success
                  ? "bg-aquamarine text-dark-purple"
                  : "bg-pink text-dark-purple"
              }
            `}
            >
              <p className="font-bold">{conversionResult.message}</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
