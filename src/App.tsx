import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
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

  return (
    <div className="min-h-screen bg-off-white">
      {/* Custom Title Bar */}
      <div
        className="bg-aquamarine px-4 py-2 flex items-center justify-between select-none"
        data-tauri-drag-region
      >
        <div className="flex items-center space-x-4">
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
          <div className="text-dark-purple font-bold text-sm">ConvertSave</div>
        </div>
        <div className="flex items-center space-x-2">
          <button className="btn-chunky bg-yellow text-dark-purple px-3 py-1 text-sm">
            Update Available
          </button>
          <button className="p-1 text-dark-purple hover:bg-dark-purple hover:bg-opacity-10 rounded">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 4.5a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4.5z" />
            </svg>
          </button>
        </div>
      </div>

      <div className="p-6 space-y-6">
        {/* Main Drop Zone */}
        <div className="border-2 border-dashed border-light-purple rounded-xl p-12 text-center bg-white">
          <div className="space-y-4">
            <div className="w-16 h-16 mx-auto bg-light-grey rounded-lg flex items-center justify-center">
              <svg
                width="32"
                height="32"
                viewBox="0 0 32 32"
                fill="none"
                className="text-light-purple"
              >
                <rect
                  x="4"
                  y="8"
                  width="20"
                  height="16"
                  rx="2"
                  stroke="currentColor"
                  strokeWidth="2"
                />
                <circle
                  cx="10"
                  cy="14"
                  r="2"
                  stroke="currentColor"
                  strokeWidth="2"
                />
                <path
                  d="M20 20l-4-4-6 6"
                  stroke="currentColor"
                  strokeWidth="2"
                />
              </svg>
            </div>
            <div className="space-y-2">
              <p className="text-lg font-bold text-dark-purple">
                Drag-and-drop images here.
              </p>
              <button className="btn-chunky bg-light-grey text-dark-purple px-6 py-2 hover:bg-light-purple hover:bg-opacity-20">
                Browse files
              </button>
            </div>
          </div>
        </div>

        {/* Convert To Section */}
        <div className="text-center space-y-4">
          <div className="flex items-center justify-center space-x-2">
            <span className="text-dark-purple font-normal">Convert to.</span>
            <span className="text-pink">✨ conver...</span>
          </div>
          <select
            value={selectedFormat}
            onChange={(e) => setSelectedFormat(e.target.value)}
            className="btn-chunky bg-yellow text-dark-purple px-4 py-2 border-2 border-dark-purple rounded-xl font-bold"
          >
            <option value="JPG">JPG</option>
            <option value="PNG">PNG</option>
            <option value="WEBP">WEBP</option>
            <option value="PDF">PDF</option>
            <option value="SVG">SVG</option>
          </select>
        </div>

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
                      <p className="font-bold text-dark-purple">{file.name}</p>
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
  );
}

export default App;
