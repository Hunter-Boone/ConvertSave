import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Download, Check, X, Loader } from "lucide-react";

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

interface DownloadProgress {
  status: string;
  message: string;
}

interface ToolDownloaderProps {
  onAllToolsReady: () => void;
}

export default function ToolDownloader({
  onAllToolsReady,
}: ToolDownloaderProps) {
  const [toolStatus, setToolStatus] = useState<ToolStatus | null>(null);
  const [downloadingTools, setDownloadingTools] = useState<Set<string>>(
    new Set()
  );
  const [downloadProgress, setDownloadProgress] =
    useState<DownloadProgress | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [currentPlatform, setCurrentPlatform] = useState<string>("windows");

  useEffect(() => {
    // Detect platform
    const userAgent = navigator.userAgent.toLowerCase();
    if (userAgent.includes("mac")) {
      setCurrentPlatform("macos");
    } else if (userAgent.includes("linux")) {
      setCurrentPlatform("linux");
    } else {
      setCurrentPlatform("windows");
    }

    checkToolsStatus();

    const unlisten = listen<DownloadProgress>("download-progress", (event) => {
      setDownloadProgress(event.payload);
      if (event.payload.status === "complete") {
        // Extract tool name from the message (e.g., "FFmpeg download complete!")
        const toolName = event.payload.message.toLowerCase().split(" ")[0];

        setSuccessMessage(`${toolName} downloaded successfully!`);

        // Remove this tool from downloading set
        setDownloadingTools((prev) => {
          const newSet = new Set(prev);
          newSet.delete(toolName);
          return newSet;
        });

        // Immediately check status to update UI
        // The backend already verified the file exists before emitting "complete"
        checkToolsStatus();

        // Clear progress message after a short delay
        setTimeout(() => {
          setDownloadProgress(null);
        }, 1000);

        // Clear success message after showing it
        setTimeout(() => {
          setSuccessMessage(null);
        }, 2500);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []); // Empty dependency array so listener is only registered once

  // Auto-transition removed - user must manually close Tools Manager
  // This prevents issues with screen refreshing/flashing

  const checkToolsStatus = async () => {
    try {
      const status = await invoke<ToolStatus>("check_tools_status");
      // Force a new object to ensure state update triggers
      setToolStatus({ ...status });
    } catch (err) {
      setError(`Failed to check tool status: ${err}`);
    }
  };

  const downloadTool = async (toolName: string) => {
    // Add to downloading set
    setDownloadingTools((prev) => new Set(prev).add(toolName));
    setDownloadProgress(null);
    setError(null);
    setSuccessMessage(null);

    try {
      if (toolName === "ffmpeg") {
        await invoke("download_ffmpeg");
      } else if (toolName === "pandoc") {
        await invoke("download_pandoc");
      } else if (toolName === "imagemagick") {
        await invoke("download_imagemagick");
      }
      // Note: Success is handled by the download-progress event listener
    } catch (err) {
      setError(`Failed to download ${toolName}: ${err}`);
      // Remove from downloading set on error
      setDownloadingTools((prev) => {
        const newSet = new Set(prev);
        newSet.delete(toolName);
        return newSet;
      });
      setDownloadProgress(null);
    }
  };

  if (!toolStatus) {
    return (
      <div className="flex items-center justify-center h-screen bg-off-white">
        <div className="text-center space-y-4">
          <Loader className="w-12 h-12 text-aquamarine animate-spin mx-auto" />
          <p className="text-dark-purple font-bold">Checking tool status...</p>
        </div>
      </div>
    );
  }

  // Core tools required: FFmpeg and Pandoc. ImageMagick is optional.
  const coreToolsReady =
    toolStatus.ffmpeg.available && toolStatus.pandoc.available;
  const allToolsReady = coreToolsReady && toolStatus.imagemagick.available;

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
              ConvertSave - Tools Manager
            </div>
          )}
        </div>

        {/* Center - Title on macOS */}
        {currentPlatform === "macos" && (
          <div className="text-dark-purple font-bold text-sm">
            ConvertSave - Tools Manager
          </div>
        )}

        {/* Right side - Controls on Windows/Linux */}
        <div className="flex items-center space-x-2">
          {currentPlatform === "windows" && <WindowsControls />}
          {currentPlatform === "linux" && <LinuxControls />}
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto overflow-x-hidden">
        <div className="p-6 flex items-center justify-center min-h-full">
          <div className="max-w-2xl w-full space-y-6">
            {/* Header */}
            <div className="text-center space-y-2">
              <h1 className="text-3xl font-bold text-dark-purple">
                Welcome to ConvertSave
              </h1>
              <p className="text-lg text-light-purple">
                To get started, we need to download some conversion tools
              </p>
            </div>

            {/* Tool Cards */}
            <div className="space-y-4">
              {/* FFmpeg Card */}
              <div className="bg-white border-2 border-light-purple rounded-xl p-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-3">
                      <h3 className="text-xl font-bold text-dark-purple">
                        FFmpeg
                      </h3>
                      {toolStatus.ffmpeg.available ? (
                        <div className="flex items-center space-x-1 bg-aquamarine text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                          <Check className="w-4 h-4" />
                          <span>Ready</span>
                        </div>
                      ) : (
                        <div className="flex items-center space-x-1 bg-yellow text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                          <X className="w-4 h-4" />
                          <span>Not Available</span>
                        </div>
                      )}
                    </div>
                    <p className="text-light-purple mt-2">
                      For converting images, videos, and audio files
                    </p>
                    {toolStatus.ffmpeg.available && toolStatus.ffmpeg.path && (
                      <p className="text-xs text-light-purple mt-1 font-mono">
                        {toolStatus.ffmpeg.path}
                      </p>
                    )}
                  </div>
                  {!toolStatus.ffmpeg.available && (
                    <button
                      onClick={() => downloadTool("ffmpeg")}
                      disabled={downloadingTools.has("ffmpeg")}
                      className="btn-chunky bg-aquamarine text-dark-purple px-6 py-3 flex items-center space-x-2"
                    >
                      {downloadingTools.has("ffmpeg") ? (
                        <>
                          <Loader className="w-5 h-5 animate-spin" />
                          <span>Downloading...</span>
                        </>
                      ) : (
                        <>
                          <Download className="w-5 h-5" />
                          <span>Download</span>
                        </>
                      )}
                    </button>
                  )}
                </div>
              </div>

              {/* Pandoc Card */}
              <div className="bg-white border-2 border-light-purple rounded-xl p-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-3">
                      <h3 className="text-xl font-bold text-dark-purple">
                        Pandoc
                      </h3>
                      {toolStatus.pandoc.available ? (
                        <div className="flex items-center space-x-1 bg-aquamarine text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                          <Check className="w-4 h-4" />
                          <span>Ready</span>
                        </div>
                      ) : (
                        <div className="flex items-center space-x-1 bg-yellow text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                          <X className="w-4 h-4" />
                          <span>Not Available</span>
                        </div>
                      )}
                    </div>
                    <p className="text-light-purple mt-2">
                      For converting document formats (Markdown, PDF, etc.)
                    </p>
                    {toolStatus.pandoc.available && toolStatus.pandoc.path && (
                      <p className="text-xs text-light-purple mt-1 font-mono">
                        {toolStatus.pandoc.path}
                      </p>
                    )}
                  </div>
                  {!toolStatus.pandoc.available && (
                    <button
                      onClick={() => downloadTool("pandoc")}
                      disabled={downloadingTools.has("pandoc")}
                      className="btn-chunky bg-aquamarine text-dark-purple px-6 py-3 flex items-center space-x-2"
                    >
                      {downloadingTools.has("pandoc") ? (
                        <>
                          <Loader className="w-5 h-5 animate-spin" />
                          <span>Downloading...</span>
                        </>
                      ) : (
                        <>
                          <Download className="w-5 h-5" />
                          <span>Download</span>
                        </>
                      )}
                    </button>
                  )}
                </div>
              </div>

              {/* ImageMagick Card */}
              <div className="bg-white border-2 border-light-purple rounded-xl p-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-3">
                      <h3 className="text-xl font-bold text-dark-purple">
                        ImageMagick
                      </h3>
                      {toolStatus.imagemagick.available ? (
                        <div className="flex items-center space-x-1 bg-aquamarine text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                          <Check className="w-4 h-4" />
                          <span>Ready</span>
                        </div>
                      ) : (
                        <div className="flex items-center space-x-1 bg-yellow text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                          <X className="w-4 h-4" />
                          <span>Not Available</span>
                        </div>
                      )}
                    </div>
                    <p className="text-light-purple mt-2">
                      Used for advanced image processing and HEIC/HEIF encoding
                    </p>
                    {toolStatus.imagemagick.available &&
                      toolStatus.imagemagick.path && (
                        <p className="text-xs text-light-purple mt-1 font-mono">
                          {toolStatus.imagemagick.path}
                        </p>
                      )}
                  </div>
                  {!toolStatus.imagemagick.available && (
                    <button
                      onClick={() => downloadTool("imagemagick")}
                      disabled={downloadingTools.has("imagemagick")}
                      className="btn-chunky bg-aquamarine text-dark-purple px-6 py-3 flex items-center space-x-2"
                    >
                      {downloadingTools.has("imagemagick") ? (
                        <>
                          <Loader className="w-5 h-5 animate-spin" />
                          <span>Downloading...</span>
                        </>
                      ) : (
                        <>
                          <Download className="w-5 h-5" />
                          <span>Download</span>
                        </>
                      )}
                    </button>
                  )}
                </div>
              </div>
            </div>

            {/* Download Progress */}
            {downloadProgress && downloadProgress.status !== "complete" && (
              <div className="bg-aquamarine border-2 border-dark-purple rounded-xl p-4">
                <div className="flex items-center space-x-3">
                  <Loader className="w-5 h-5 animate-spin text-dark-purple" />
                  <div>
                    <p className="font-bold text-dark-purple capitalize">
                      {downloadProgress.status}
                    </p>
                    <p className="text-sm text-dark-purple">
                      {downloadProgress.message}
                    </p>
                  </div>
                </div>
              </div>
            )}

            {/* Success Message */}
            {successMessage && (
              <div className="bg-aquamarine border-2 border-dark-purple rounded-xl p-4">
                <div className="flex items-center space-x-3">
                  <Check className="w-5 h-5 text-dark-purple" />
                  <p className="font-bold text-dark-purple">{successMessage}</p>
                </div>
              </div>
            )}

            {/* Error Message */}
            {error && (
              <div className="bg-pink border-2 border-dark-purple rounded-xl p-4">
                <div className="flex items-center space-x-3">
                  <X className="w-5 h-5 text-dark-purple" />
                  <p className="font-bold text-dark-purple">{error}</p>
                </div>
              </div>
            )}

            {/* License Notice */}
            <div className="bg-light-grey rounded-xl p-4 text-sm text-light-purple space-y-2">
              <p className="font-bold text-dark-purple">License Information</p>
              <p>
                <strong>FFmpeg</strong> is licensed under the{" "}
                <a
                  href="https://www.gnu.org/licenses/gpl-3.0.html"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-dark-purple underline hover:text-aquamarine"
                >
                  GNU GPL v3
                </a>
                . We download it from official sources.
              </p>
              <p>
                <strong>Pandoc</strong> is licensed under the{" "}
                <a
                  href="https://www.gnu.org/licenses/gpl-2.0.html"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-dark-purple underline hover:text-aquamarine"
                >
                  GNU GPL v2+
                </a>
                .
              </p>
              <p>
                <strong>ImageMagick</strong> is licensed under the{" "}
                <a
                  href="https://imagemagick.org/script/license.php"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-dark-purple underline hover:text-aquamarine"
                >
                  Apache 2.0 License
                </a>
                .
              </p>
              <p className="text-xs">
                These tools are downloaded on first use to comply with their
                respective licenses and to keep the application size small.
              </p>
            </div>

            {/* Continue Button */}
            {allToolsReady ? (
              <div className="text-center space-y-3">
                <button
                  onClick={onAllToolsReady}
                  className="btn-chunky bg-aquamarine text-dark-purple px-8 py-4 text-lg w-full"
                >
                  Continue to ConvertSave
                </button>
                <p className="text-sm text-light-purple">
                  All tools are ready! You can now convert files.
                </p>
              </div>
            ) : coreToolsReady ? (
              <div className="text-center space-y-3">
                <button
                  onClick={onAllToolsReady}
                  className="btn-chunky bg-aquamarine text-dark-purple px-8 py-4 text-lg w-full"
                >
                  Continue to ConvertSave
                </button>
                <p className="text-sm text-light-purple">
                  Core tools ready! ImageMagick (optional) can be added later.
                </p>
              </div>
            ) : (
              <div className="text-center space-y-3">
                <button
                  onClick={onAllToolsReady}
                  className="btn-chunky bg-light-grey text-dark-purple px-8 py-3 text-base w-full hover:bg-tan"
                >
                  Skip for Now
                </button>
                <p className="text-xs text-light-purple">
                  FFmpeg and Pandoc are required for most conversions.
                </p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
