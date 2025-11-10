import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { Check, X, Loader, ChevronDown, ChevronUp } from "lucide-react";
import { LGPL_V3_LICENSE, GPL_V3_LICENSE, IMAGEMAGICK_LICENSE } from "../lib/licenses";

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
  const [ffmpegAdvancedOpen, setFfmpegAdvancedOpen] = useState(false);
  const [imagemagickAdvancedOpen, setImagemagickAdvancedOpen] = useState(false);
  const [licenseAttributionOpen, setLicenseAttributionOpen] = useState(false);
  const [aboutOpen, setAboutOpen] = useState(false);
  const [showLicensesModal, setShowLicensesModal] = useState(false);
  const [expandedLicense, setExpandedLicense] = useState<string | null>(null);

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
    // Update checking functionality temporarily disabled
    // This function is kept for future use
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
      // DISABLED: Pandoc functionality temporarily disabled
      // } else if (toolName === "pandoc") {
      //   await invoke("download_pandoc");
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

  const selectCustomPath = async (toolName: string) => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        title: `Select ${toolName} executable`,
      });

      if (selected && typeof selected === "string") {
        setError(null);
        setSuccessMessage(null);
        
        try {
          await invoke("set_custom_tool_path", {
            toolName,
            path: selected,
          });
          
          setSuccessMessage(`Custom path set for ${toolName}`);
          
          // Refresh tool status to reflect the change
          checkToolsStatus();
          
          // Clear success message after a delay
          setTimeout(() => {
            setSuccessMessage(null);
          }, 3000);
        } catch (err) {
          setError(`Failed to set custom path: ${err}`);
        }
      }
    } catch (err) {
      setError(`Failed to select file: ${err}`);
    }
  };

  const useDefaultPath = async (toolName: string) => {
    try {
      await invoke("clear_custom_tool_path", { toolName });
      setSuccessMessage(`Using default path for ${toolName}`);
      
      // Refresh tool status to reflect the change
      checkToolsStatus();
      
      // Clear success message after a delay
      setTimeout(() => {
        setSuccessMessage(null);
      }, 3000);
    } catch (err) {
      setError(`Failed to clear custom path: ${err}`);
    }
  };

  if (!toolStatus) {
    return (
      <div className="flex items-center justify-center h-screen bg-light-bg">
        <div className="text-center space-y-4">
          <Loader className="w-12 h-12 text-mint-accent animate-spin mx-auto" />
          <p className="text-dark-purple font-bold">Checking tool status...</p>
        </div>
      </div>
    );
  }

  // Core tools required: FFmpeg only (Pandoc disabled). ImageMagick is optional.
  const coreToolsReady =
    toolStatus.ffmpeg.available;

  const handleCloseClick = () => {
    if (!coreToolsReady) {
      setError("At least one conversion tool must be installed before you can use ConvertSave.");
      // Clear the error after a few seconds
      setTimeout(() => {
        setError(null);
      }, 5000);
      return;
    }
    onAllToolsReady();
  };

  return (
    <>
      {/* Licenses Modal */}
      {showLicensesModal && (
        <div 
          className="fixed inset-0 z-[200] flex items-center justify-center bg-dark-purple bg-opacity-50"
          onClick={() => setShowLicensesModal(false)}
        >
          <div 
            className="bg-white rounded-xl shadow-2xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-hidden flex flex-col"
            onClick={(e) => e.stopPropagation()}
          >
            {/* Modal Header */}
            <div className="flex items-center justify-between p-6 border-b border-lighter-bg">
              <h2 className="text-2xl font-bold text-dark-purple">Licenses</h2>
              <button
                onClick={() => setShowLicensesModal(false)}
                className="w-8 h-8 bg-light-grey hover:bg-pink rounded-lg flex items-center justify-center transition-colors"
                aria-label="Close"
              >
                <X className="w-5 h-5 text-dark-purple" />
              </button>
            </div>
            
            {/* Modal Content */}
            <div className="flex-1 overflow-y-auto p-6 space-y-4">
              {/* GNU LGPLv3 */}
              <button
                onClick={() => setExpandedLicense(expandedLicense === "lgpl" ? null : "lgpl")}
                className="w-full p-4 bg-white border-2 border-dark-purple rounded-xl flex items-center justify-between hover:bg-light-bg transition-colors"
              >
                <span className="font-bold text-dark-purple">GNU LGPLv3</span>
                {expandedLicense === "lgpl" ? (
                  <ChevronUp className="w-5 h-5 text-dark-purple" />
                ) : (
                  <ChevronDown className="w-5 h-5 text-dark-purple" />
                )}
              </button>
              {expandedLicense === "lgpl" && (
                <div className="p-4 bg-light-bg rounded-xl text-xs text-secondary whitespace-pre-wrap font-mono max-h-96 overflow-y-auto">
                  {LGPL_V3_LICENSE}
                </div>
              )}

              {/* GNU GPLv3 */}
              <button
                onClick={() => setExpandedLicense(expandedLicense === "gpl" ? null : "gpl")}
                className="w-full p-4 bg-white border-2 border-dark-purple rounded-xl flex items-center justify-between hover:bg-light-bg transition-colors"
              >
                <span className="font-bold text-dark-purple">GNU GPLv3</span>
                {expandedLicense === "gpl" ? (
                  <ChevronUp className="w-5 h-5 text-dark-purple" />
                ) : (
                  <ChevronDown className="w-5 h-5 text-dark-purple" />
                )}
              </button>
              {expandedLicense === "gpl" && (
                <div className="p-4 bg-light-bg rounded-xl text-xs text-secondary whitespace-pre-wrap font-mono max-h-96 overflow-y-auto">
                  {GPL_V3_LICENSE}
                </div>
              )}

              {/* ImageMagick License */}
              <button
                onClick={() => setExpandedLicense(expandedLicense === "imagemagick" ? null : "imagemagick")}
                className="w-full p-4 bg-white border-2 border-dark-purple rounded-xl flex items-center justify-between hover:bg-light-bg transition-colors"
              >
                <span className="font-bold text-dark-purple">ImageMagick License</span>
                {expandedLicense === "imagemagick" ? (
                  <ChevronUp className="w-5 h-5 text-dark-purple" />
                ) : (
                  <ChevronDown className="w-5 h-5 text-dark-purple" />
                )}
              </button>
              {expandedLicense === "imagemagick" && (
                <div className="p-4 bg-light-bg rounded-xl text-xs text-secondary whitespace-pre-wrap font-mono max-h-96 overflow-y-auto">
                  {IMAGEMAGICK_LICENSE}
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      <div className="h-screen bg-light-bg flex flex-col overflow-hidden relative">
        {/* Close Button */}
        <button
          onClick={handleCloseClick}
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
                  Welcome to ConvertSave!
                </h1>
                <p className="text-lg text-secondary">
                  To get started, we need to download a few conversion tools.
                </p>
              </div>

              {/* Tool Cards */}
              <div className="space-y-4">
                {/* FFmpeg Card */}
                <div className="bg-white border-2 border-dark-purple rounded-xl p-6">
                  <div className="flex items-center justify-between mb-4">
                    <div className="flex-1">
                      <div className="flex items-center space-x-3 flex-wrap mb-2">
                        <h3 className="text-xl font-bold text-dark-purple">
                          FFmpeg
                        </h3>
                        {toolStatus.ffmpeg.available ? (
                          <div className="flex items-center space-x-1 bg-mint-accent text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                            <Check className="w-4 h-4" />
                            <span>Ready</span>
                          </div>
                        ) : (
                          <div className="flex items-center space-x-1 bg-pink-accent text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                            <X className="w-4 h-4" />
                            <span>Not Found</span>
                          </div>
                        )}
                      </div>
                      <p className="text-secondary">
                        Used to convert most images formats.
                      </p>
                    </div>
                    {!toolStatus.ffmpeg.available && (
                      <button
                        onClick={() => downloadTool("ffmpeg")}
                        disabled={downloadingTools.has("ffmpeg")}
                        className="btn-chunky bg-mint-accent text-dark-purple px-6 py-3 flex items-center space-x-2"
                      >
                        {downloadingTools.has("ffmpeg") ? (
                          <>
                            <Loader className="w-5 h-5 animate-spin" />
                            <span>Download</span>
                          </>
                        ) : (
                          <span>Download</span>
                        )}
                      </button>
                    )}
                  </div>
                  
                  {/* Advanced Section */}
                  <button
                    onClick={() => setFfmpegAdvancedOpen(!ffmpegAdvancedOpen)}
                    className="flex items-center space-x-2 text-dark-purple font-bold hover:text-secondary transition-colors"
                  >
                    <span>Advanced</span>
                    {ffmpegAdvancedOpen ? (
                      <ChevronUp className="w-4 h-4" />
                    ) : (
                      <ChevronDown className="w-4 h-4" />
                    )}
                  </button>
                  
                  {ffmpegAdvancedOpen && (
                    <div className="mt-4 p-4 bg-light-bg rounded-lg space-y-3">
                      <p className="text-sm text-secondary">
                        You can also download FFmpeg directly and select a custom path.
                      </p>
                      <div className="flex space-x-2">
                        <button 
                          onClick={() => selectCustomPath("ffmpeg")}
                          className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 text-sm hover:bg-light-bg"
                        >
                          Select Custom Path
                        </button>
                        <button 
                          onClick={() => useDefaultPath("ffmpeg")}
                          className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 text-sm hover:bg-light-bg"
                        >
                          Use Default Path
                        </button>
                      </div>
                      {toolStatus.ffmpeg.path && (
                        <p className="text-xs text-secondary font-mono mt-2 break-all">
                          {toolStatus.ffmpeg.path}
                        </p>
                      )}
                    </div>
                  )}
                </div>

              {/* DISABLED: Pandoc functionality temporarily disabled */}
              {/* {/* Pandoc Card */}
              {/* <div className="bg-white border-2 border-light-purple rounded-xl p-6">
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
              </div> */}

                {/* ImageMagick Card */}
                <div className="bg-white border-2 border-dark-purple rounded-xl p-6">
                  <div className="flex items-center justify-between mb-4">
                    <div className="flex-1">
                      <div className="flex items-center space-x-3 flex-wrap mb-2">
                        <h3 className="text-xl font-bold text-dark-purple">
                          ImageMagick
                        </h3>
                        {toolStatus.imagemagick.available ? (
                          <div className="flex items-center space-x-1 bg-mint-accent text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                            <Check className="w-4 h-4" />
                            <span>Ready</span>
                          </div>
                        ) : (
                          <div className="flex items-center space-x-1 bg-pink-accent text-dark-purple px-3 py-1 rounded-full text-sm font-bold">
                            <X className="w-4 h-4" />
                            <span>Not Found</span>
                          </div>
                        )}
                      </div>
                      <p className="text-secondary">
                        Adds additional features, including HEIC encoding.
                      </p>
                    </div>
                    {!toolStatus.imagemagick.available && (
                      <button
                        onClick={() => downloadTool("imagemagick")}
                        disabled={downloadingTools.has("imagemagick")}
                        className="btn-chunky bg-mint-accent text-dark-purple px-6 py-3 flex items-center space-x-2"
                      >
                        {downloadingTools.has("imagemagick") ? (
                          <>
                            <Loader className="w-5 h-5 animate-spin" />
                            <span>Download</span>
                          </>
                        ) : (
                          <span>Download</span>
                        )}
                      </button>
                    )}
                  </div>
                  
                  {/* Advanced Section */}
                  <button
                    onClick={() => setImagemagickAdvancedOpen(!imagemagickAdvancedOpen)}
                    className="flex items-center space-x-2 text-dark-purple font-bold hover:text-secondary transition-colors"
                  >
                    <span>Advanced</span>
                    {imagemagickAdvancedOpen ? (
                      <ChevronUp className="w-4 h-4" />
                    ) : (
                      <ChevronDown className="w-4 h-4" />
                    )}
                  </button>
                  
                  {imagemagickAdvancedOpen && (
                    <div className="mt-4 p-4 bg-light-bg rounded-lg space-y-3">
                      <p className="text-sm text-secondary">
                        You can also download ImageMagick directly and select a custom path.
                      </p>
                      <div className="flex space-x-2">
                        <button 
                          onClick={() => selectCustomPath("imagemagick")}
                          className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 text-sm hover:bg-light-bg"
                        >
                          Select Custom Path
                        </button>
                        <button 
                          onClick={() => useDefaultPath("imagemagick")}
                          className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 text-sm hover:bg-light-bg"
                        >
                          Use Default Path
                        </button>
                      </div>
                      {toolStatus.imagemagick.path && (
                        <p className="text-xs text-secondary font-mono mt-2 break-all">
                          {toolStatus.imagemagick.path}
                        </p>
                      )}
                    </div>
                  )}
                </div>
              </div>

              {/* Continue Button */}
              {coreToolsReady && (
                <button
                  onClick={onAllToolsReady}
                  className="btn-chunky bg-mint-accent text-dark-purple px-8 py-4 text-lg w-full"
                >
                  Continue
                </button>
              )}

              {/* Download Progress */}
              {downloadProgress && downloadProgress.status !== "complete" && (
                <div className="bg-mint-accent border-2 border-dark-purple rounded-xl p-4">
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
                <div className="bg-mint-accent border-2 border-dark-purple rounded-xl p-4">
                  <div className="flex items-center space-x-3">
                    <Check className="w-5 h-5 text-dark-purple" />
                    <p className="font-bold text-dark-purple">{successMessage}</p>
                  </div>
                </div>
              )}

              {/* Error Message */}
              {error && (
                <div className="bg-pink-accent border-2 border-dark-purple rounded-xl p-4">
                  <div className="flex items-center space-x-3">
                    <X className="w-5 h-5 text-dark-purple" />
                    <p className="font-bold text-dark-purple">{error}</p>
                  </div>
                </div>
              )}

              {/* License & Attribution Section */}
              <div className="space-y-2">
                <button
                  onClick={() => setLicenseAttributionOpen(!licenseAttributionOpen)}
                  className="w-full p-4 bg-lighter-bg rounded-xl flex items-center justify-between hover:bg-muted-bg transition-colors"
                >
                  <span className="font-bold text-dark-purple">License & Attribution</span>
                  {licenseAttributionOpen ? (
                    <ChevronUp className="w-5 h-5 text-dark-purple" />
                  ) : (
                    <ChevronDown className="w-5 h-5 text-dark-purple" />
                  )}
                </button>
                
                {licenseAttributionOpen && (
                  <div className="p-6 bg-lighter-bg rounded-xl space-y-6 text-sm text-secondary">
                    <p>
                      The following tools are downloaded separately and from official sources in order to comply with their respective licenses.
                    </p>

                    <div className="space-y-3">
                      <h3 className="font-bold text-dark-purple">FFmpeg</h3>
                      <p>
                        FFmpeg is the leading multimedia framework, able to decode, encode, transcode, mux, demux, stream, filter and play pretty much anything that humans and machines have created.
                      </p>
                      <p>
                        FFmpeg is licensed under the GNU Lesser General Public License (LGPL) version 2.1 or later. However, FFmpeg incorporates several optional parts and optimizations that are covered by the GNU General Public License (GPL) version 2 or later. For more information, visit{" "}
                        <a
                          href="https://www.ffmpeg.org/legal.html"
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-blue-accent underline hover:text-dark-purple"
                        >
                          https://www.ffmpeg.org/legal.html
                        </a>
                        .
                      </p>
                      <p>
                        FFmpeg is a trademark of Fabrice Bellard, originator of the FFmpeg project.
                      </p>
                    </div>

                    <div className="space-y-3">
                      <h3 className="font-bold text-dark-purple">ImageMagick</h3>
                      <p>
                        ImageMagick is a free, open-source software suite, used for editing and manipulating digital images.
                      </p>
                      <p>
                        ImageMagick is licensed under the ImageMagick License. For more information, visit{" "}
                        <a
                          href="https://imagemagick.org/script/license.php"
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-blue-accent underline hover:text-dark-purple"
                        >
                          https://imagemagick.org/script/license.php
                        </a>
                        .
                      </p>
                      <p>
                        Copyright © 1999 ImageMagick Studio LLC, a non-profit organization dedicated to making software imaging solutions freely available.
                      </p>
                    </div>

                    <button
                      onClick={() => setShowLicensesModal(true)}
                      className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-6 py-3 hover:bg-light-bg"
                    >
                      Licenses
                    </button>
                  </div>
                )}
              </div>

              {/* About Section */}
              <div className="space-y-2">
                <button
                  onClick={() => setAboutOpen(!aboutOpen)}
                  className="w-full p-4 bg-lighter-bg rounded-xl flex items-center justify-between hover:bg-muted-bg transition-colors"
                >
                  <span className="font-bold text-dark-purple">About</span>
                  {aboutOpen ? (
                    <ChevronUp className="w-5 h-5 text-dark-purple" />
                  ) : (
                    <ChevronDown className="w-5 h-5 text-dark-purple" />
                  )}
                </button>
                
                {aboutOpen && (
                  <div className="p-6 bg-lighter-bg rounded-xl text-sm text-secondary">
                    <p>
                      ConvertSave helps you quickly convert images locally on your computer.
                    </p>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
