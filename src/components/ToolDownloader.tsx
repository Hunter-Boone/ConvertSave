import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Download, Check, X, Loader, RefreshCw, AlertCircle } from "lucide-react";

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

interface UpdateInfo {
  installed: boolean;
  currentVersion: string | null;
  updateAvailable: boolean;
  latestVersion: string | null;
}

interface UpdateStatus {
  ffmpeg: UpdateInfo;
  pandoc: UpdateInfo;
  imagemagick: UpdateInfo;
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
  const [updateStatus, setUpdateStatus] = useState<UpdateStatus | null>(null);
  const [checkingUpdates, setCheckingUpdates] = useState(false);
  const [downloadingTools, setDownloadingTools] = useState<Set<string>>(
    new Set()
  );
  const [downloadProgress, setDownloadProgress] =
    useState<DownloadProgress | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  useEffect(() => {
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

        // Re-check for updates to refresh version information
        setTimeout(() => {
          checkForUpdates();
        }, 500);

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

  const checkForUpdates = async () => {
    setCheckingUpdates(true);
    setError(null);
    try {
      const updates = await invoke<UpdateStatus>("check_for_updates");
      setUpdateStatus(updates);
      
      // Show success message if any updates available
      const hasUpdates = Object.values(updates).some(
        (tool) => tool.updateAvailable
      );
      if (hasUpdates) {
        setSuccessMessage("Updates available for some tools!");
        setTimeout(() => setSuccessMessage(null), 3000);
      } else {
        setSuccessMessage("All tools are up to date!");
        setTimeout(() => setSuccessMessage(null), 2500);
      }
    } catch (err) {
      setError(`Failed to check for updates: ${err}`);
    } finally {
      setCheckingUpdates(false);
    }
  };

  const downloadTool = async (toolName: string) => {
    // Add to downloading set
    setDownloadingTools((prev) => new Set(prev).add(toolName));
    setDownloadProgress(null);
    setError(null);
    setSuccessMessage(null);
    // Keep updateStatus visible during download so user can see what's being updated

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

  return (
    <div className="h-screen bg-off-white flex flex-col overflow-hidden relative">
      {/* Close Button */}
      <button
        onClick={onAllToolsReady}
        className="absolute top-6 right-6 z-50 w-10 h-10 bg-white hover:bg-pink rounded-lg flex items-center justify-center transition-colors shadow-lg border-2 border-dark-purple"
        aria-label="Close Tools Manager"
        title="Back to main app"
      >
        <X className="w-6 h-6 text-dark-purple" />
      </button>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto overflow-x-hidden">
        <div className="p-6 flex items-center justify-center min-h-full pt-20">
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

            {/* Check For Updates Button */}
            {toolStatus.ffmpeg.available || toolStatus.pandoc.available || toolStatus.imagemagick.available ? (
              <div className="flex justify-center">
                <button
                  onClick={checkForUpdates}
                  disabled={checkingUpdates}
                  className="btn-chunky bg-yellow text-dark-purple px-6 py-3 flex items-center space-x-2 hover:shadow-lg transition-shadow"
                >
                  {checkingUpdates ? (
                    <>
                      <Loader className="w-5 h-5 animate-spin" />
                      <span>Checking for Updates...</span>
                    </>
                  ) : (
                    <>
                      <RefreshCw className="w-5 h-5" />
                      <span>Check For Updates</span>
                    </>
                  )}
                </button>
              </div>
            ) : null}

            {/* Tool Cards */}
            <div className="space-y-4">
              {/* FFmpeg Card */}
              <div className="bg-white border-2 border-light-purple rounded-xl p-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-3 flex-wrap">
                      <h3 className="text-xl font-bold text-dark-purple">
                        FFmpeg
                      </h3>
                      {toolStatus.ffmpeg.available ? (
                        <>
                          <div className="flex items-center space-x-1 bg-aquamarine text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                            <Check className="w-4 h-4" />
                            <span>Ready</span>
                          </div>
                          {updateStatus?.ffmpeg?.updateAvailable && (
                            <div className="flex items-center space-x-1 bg-yellow text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                              <AlertCircle className="w-4 h-4" />
                              <span>Update Available</span>
                            </div>
                          )}
                        </>
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
                    {updateStatus?.ffmpeg?.currentVersion && (
                      <p className="text-xs text-light-purple mt-1">
                        Version: {updateStatus.ffmpeg.currentVersion}
                        {updateStatus.ffmpeg.latestVersion && 
                         updateStatus.ffmpeg.updateAvailable && (
                          <span className="text-dark-purple font-bold ml-1 bg-yellow px-2 py-0.5 rounded">
                            → {updateStatus.ffmpeg.latestVersion}
                          </span>
                        )}
                      </p>
                    )}
                  </div>
                  {(!toolStatus.ffmpeg.available || updateStatus?.ffmpeg?.updateAvailable) && (
                    <button
                      onClick={() => downloadTool("ffmpeg")}
                      disabled={downloadingTools.has("ffmpeg")}
                      className={`btn-chunky ${updateStatus?.ffmpeg?.updateAvailable ? 'bg-yellow' : 'bg-aquamarine'} text-dark-purple px-6 py-3 flex items-center space-x-2`}
                    >
                      {downloadingTools.has("ffmpeg") ? (
                        <>
                          <Loader className="w-5 h-5 animate-spin" />
                          <span>Downloading...</span>
                        </>
                      ) : (
                        <>
                          <Download className="w-5 h-5" />
                          <span>{updateStatus?.ffmpeg?.updateAvailable ? 'Update' : 'Download'}</span>
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
                    <div className="flex items-center space-x-3 flex-wrap">
                      <h3 className="text-xl font-bold text-dark-purple">
                        Pandoc
                      </h3>
                      {toolStatus.pandoc.available ? (
                        <>
                          <div className="flex items-center space-x-1 bg-aquamarine text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                            <Check className="w-4 h-4" />
                            <span>Ready</span>
                          </div>
                          {updateStatus?.pandoc?.updateAvailable && (
                            <div className="flex items-center space-x-1 bg-yellow text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                              <AlertCircle className="w-4 h-4" />
                              <span>Update Available</span>
                            </div>
                          )}
                        </>
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
                    {updateStatus?.pandoc?.currentVersion && (
                      <p className="text-xs text-light-purple mt-1">
                        Version: {updateStatus.pandoc.currentVersion}
                        {updateStatus.pandoc.latestVersion && 
                         updateStatus.pandoc.updateAvailable && (
                          <span className="text-dark-purple font-bold ml-1 bg-yellow px-2 py-0.5 rounded">
                            → {updateStatus.pandoc.latestVersion}
                          </span>
                        )}
                      </p>
                    )}
                  </div>
                  {(!toolStatus.pandoc.available || updateStatus?.pandoc?.updateAvailable) && (
                    <button
                      onClick={() => downloadTool("pandoc")}
                      disabled={downloadingTools.has("pandoc")}
                      className={`btn-chunky ${updateStatus?.pandoc?.updateAvailable ? 'bg-yellow' : 'bg-aquamarine'} text-dark-purple px-6 py-3 flex items-center space-x-2`}
                    >
                      {downloadingTools.has("pandoc") ? (
                        <>
                          <Loader className="w-5 h-5 animate-spin" />
                          <span>Downloading...</span>
                        </>
                      ) : (
                        <>
                          <Download className="w-5 h-5" />
                          <span>{updateStatus?.pandoc?.updateAvailable ? 'Update' : 'Download'}</span>
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
                    <div className="flex items-center space-x-3 flex-wrap">
                      <h3 className="text-xl font-bold text-dark-purple">
                        ImageMagick
                      </h3>
                      {toolStatus.imagemagick.available ? (
                        <>
                          <div className="flex items-center space-x-1 bg-aquamarine text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                            <Check className="w-4 h-4" />
                            <span>Ready</span>
                          </div>
                          {updateStatus?.imagemagick?.updateAvailable && (
                            <div className="flex items-center space-x-1 bg-yellow text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                              <AlertCircle className="w-4 h-4" />
                              <span>Update Available</span>
                            </div>
                          )}
                        </>
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
                    {updateStatus?.imagemagick?.currentVersion && (
                      <p className="text-xs text-light-purple mt-1">
                        Version: {updateStatus.imagemagick.currentVersion}
                        {updateStatus.imagemagick.latestVersion && 
                         updateStatus.imagemagick.updateAvailable && (
                          <span className="text-dark-purple font-bold ml-1 bg-yellow px-2 py-0.5 rounded">
                            → {updateStatus.imagemagick.latestVersion}
                          </span>
                        )}
                      </p>
                    )}
                  </div>
                  {(!toolStatus.imagemagick.available || updateStatus?.imagemagick?.updateAvailable) && (
                    <button
                      onClick={() => downloadTool("imagemagick")}
                      disabled={downloadingTools.has("imagemagick")}
                      className={`btn-chunky ${updateStatus?.imagemagick?.updateAvailable ? 'bg-yellow' : 'bg-aquamarine'} text-dark-purple px-6 py-3 flex items-center space-x-2`}
                    >
                      {downloadingTools.has("imagemagick") ? (
                        <>
                          <Loader className="w-5 h-5 animate-spin" />
                          <span>Downloading...</span>
                        </>
                      ) : (
                        <>
                          <Download className="w-5 h-5" />
                          <span>{updateStatus?.imagemagick?.updateAvailable ? 'Update' : 'Download'}</span>
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
