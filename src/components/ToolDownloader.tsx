import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { open as openUrl } from "@tauri-apps/plugin-shell";
import {
  Check,
  X,
  Loader,
  ChevronDown,
  Key,
  Bug,
  FileText,
} from "lucide-react";
import {
  LGPL_V3_LICENSE,
  GPL_V3_LICENSE,
  IMAGEMAGICK_LICENSE,
  LUCIDE_LICENSE,
} from "../lib/licenses";

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
  productKey: string | null;
  onProductKeyChanged: (newKey: string) => void;
}

export default function ToolDownloader({
  onAllToolsReady,
  productKey,
  onProductKeyChanged,
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
  const [supportOpen, setSupportOpen] = useState(false);
  const [showLicensesModal, setShowLicensesModal] = useState(false);
  const [expandedLicense, setExpandedLicense] = useState<string | null>(null);
  const [showConfirmPopup, setShowConfirmPopup] = useState(false);

  // Product Key state
  const [productKeyOpen, setProductKeyOpen] = useState(false);
  const [showProductKeyModal, setShowProductKeyModal] = useState(false);
  const [newProductKey, setNewProductKey] = useState("");
  const [isChangingKey, setIsChangingKey] = useState(false);
  const [productKeyError, setProductKeyError] = useState<string | null>(null);
  const [productKeySuccess, setProductKeySuccess] = useState<string | null>(
    null
  );

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

  // Format product key as user types (XXXXX-XXXXX-XXXXX-XXXXX)
  const handleNewKeyChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let value = e.target.value.toUpperCase().replace(/[^A-Z0-9]/g, "");

    // Add dashes every 5 characters
    const parts = [];
    for (let i = 0; i < value.length && i < 20; i += 5) {
      parts.push(value.slice(i, i + 5));
    }
    setNewProductKey(parts.join("-"));
    setProductKeyError(null);
  };

  const handleChangeProductKey = async () => {
    if (!newProductKey || newProductKey.replace(/-/g, "").length !== 20) {
      setProductKeyError("Please enter a valid product key");
      return;
    }

    setIsChangingKey(true);
    setProductKeyError(null);
    setProductKeySuccess(null);

    try {
      // Get device name
      let deviceName = "Unknown Device";
      try {
        deviceName = await invoke<string>("get_device_id");
        deviceName = deviceName.slice(0, 20);
      } catch {
        // Use default
      }

      const status = await invoke<LicenseStatus>("change_product_key", {
        newProductKey: newProductKey,
        deviceName: deviceName,
      });

      if (status.isValid) {
        onProductKeyChanged(newProductKey);
        setProductKeySuccess("Product key changed successfully!");
        setNewProductKey("");

        // Close modal after a short delay
        setTimeout(() => {
          setShowProductKeyModal(false);
          setProductKeySuccess(null);
        }, 2000);
      } else {
        setProductKeyError(status.error || "Failed to change product key");
      }
    } catch (err: any) {
      console.error("Change product key error:", err);
      setProductKeyError(err.toString() || "Failed to change product key");
    } finally {
      setIsChangingKey(false);
    }
  };

  const openProductKeyModal = () => {
    setNewProductKey("");
    setProductKeyError(null);
    setProductKeySuccess(null);
    setShowProductKeyModal(true);
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

  // At least one tool must be installed to avoid the confirmation popup
  const hasAnyTool =
    toolStatus.ffmpeg.available || toolStatus.imagemagick.available;

  // All tools must be installed for the button to be green
  const allToolsReady =
    toolStatus.ffmpeg.available && toolStatus.imagemagick.available;

  const handleContinueClick = () => {
    if (!hasAnyTool) {
      setShowConfirmPopup(true);
      return;
    }
    onAllToolsReady();
  };

  const handleConfirmContinue = () => {
    setShowConfirmPopup(false);
    onAllToolsReady();
  };

  return (
    <>
      {/* Product Key Modal */}
      {showProductKeyModal && (
        <div
          className="fixed inset-0 z-[200] flex items-center justify-center bg-dark-purple bg-opacity-50"
          onClick={() => setShowProductKeyModal(false)}
        >
          <div
            className="bg-white rounded-xl shadow-2xl max-w-md w-full mx-4 overflow-hidden flex flex-col"
            onClick={(e) => e.stopPropagation()}
          >
            {/* Modal Header */}
            <div className="flex items-center justify-between p-6 border-b border-lighter-bg">
              <h2 className="text-2xl font-bold text-dark-purple">
                Product Key
              </h2>
              <button
                onClick={() => setShowProductKeyModal(false)}
                className="w-8 h-8 bg-light-grey hover:bg-pink rounded-lg flex items-center justify-center transition-colors"
                aria-label="Close"
              >
                <X className="w-5 h-5 text-dark-purple" />
              </button>
            </div>

            {/* Modal Content */}
            <div className="p-6 space-y-6">
              {/* Current Product Key */}
              {productKey && (
                <div className="space-y-2">
                  <label className="block text-sm font-bold text-dark-purple">
                    Current Product Key
                  </label>
                  <div className="w-full px-4 py-3 text-lg font-mono tracking-wider border-2 border-lighter-bg rounded-xl bg-light-bg text-center text-secondary select-all">
                    {productKey}
                  </div>
                </div>
              )}

              {/* New Product Key Input */}
              <div className="space-y-2">
                <label className="block text-sm font-bold text-dark-purple">
                  {productKey ? "New Product Key" : "Enter Product Key"}
                </label>
                <input
                  type="text"
                  value={newProductKey}
                  onChange={handleNewKeyChange}
                  placeholder="XXXXX-XXXXX-XXXXX-XXXXX"
                  className="w-full px-4 py-3 text-lg font-mono tracking-wider border-2 border-dark-purple rounded-xl focus:outline-none focus:ring-2 focus:ring-mint-accent text-center uppercase"
                  disabled={isChangingKey}
                  maxLength={23}
                />
                <p className="text-xs text-secondary text-center">
                  Enter a new product key to switch to a different license
                </p>
              </div>

              {/* Error Message */}
              {productKeyError && (
                <div className="bg-pink-accent text-dark-purple px-4 py-3 rounded-xl text-sm font-medium">
                  {productKeyError}
                </div>
              )}

              {/* Success Message */}
              {productKeySuccess && (
                <div className="bg-mint-accent text-dark-purple px-4 py-3 rounded-xl text-sm font-medium flex items-center space-x-2">
                  <Check className="w-4 h-4" />
                  <span>{productKeySuccess}</span>
                </div>
              )}

              {/* Change Button */}
              <button
                onClick={handleChangeProductKey}
                disabled={
                  isChangingKey || newProductKey.replace(/-/g, "").length !== 20
                }
                className={`w-full py-3 rounded-xl font-bold text-lg border-2 border-dark-purple transition-all ${
                  isChangingKey || newProductKey.replace(/-/g, "").length !== 20
                    ? "bg-lighter-bg text-secondary cursor-not-allowed"
                    : "bg-mint-accent text-dark-purple hover:bg-opacity-80"
                }`}
              >
                {isChangingKey ? (
                  <span className="flex items-center justify-center gap-2">
                    <Loader className="w-5 h-5 animate-spin" />
                    Changing...
                  </span>
                ) : (
                  "Change Product Key"
                )}
              </button>
            </div>
          </div>
        </div>
      )}

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
                onClick={() =>
                  setExpandedLicense(expandedLicense === "lgpl" ? null : "lgpl")
                }
                className="w-full p-4 bg-white border-2 border-dark-purple rounded-xl flex items-center justify-between hover:bg-light-bg transition-colors"
              >
                <span className="font-bold text-dark-purple">GNU LGPLv3</span>
                <ChevronDown
                  className={`w-5 h-5 text-dark-purple transition-transform duration-300 ${
                    expandedLicense === "lgpl" ? "rotate-180" : ""
                  }`}
                />
              </button>
              {expandedLicense === "lgpl" && (
                <div className="p-4 bg-light-bg rounded-xl text-xs text-secondary whitespace-pre-wrap font-mono max-h-96 overflow-y-auto">
                  {LGPL_V3_LICENSE}
                </div>
              )}

              {/* GNU GPLv3 */}
              <button
                onClick={() =>
                  setExpandedLicense(expandedLicense === "gpl" ? null : "gpl")
                }
                className="w-full p-4 bg-white border-2 border-dark-purple rounded-xl flex items-center justify-between hover:bg-light-bg transition-colors"
              >
                <span className="font-bold text-dark-purple">GNU GPLv3</span>
                <ChevronDown
                  className={`w-5 h-5 text-dark-purple transition-transform duration-300 ${
                    expandedLicense === "gpl" ? "rotate-180" : ""
                  }`}
                />
              </button>
              {expandedLicense === "gpl" && (
                <div className="p-4 bg-light-bg rounded-xl text-xs text-secondary whitespace-pre-wrap font-mono max-h-96 overflow-y-auto">
                  {GPL_V3_LICENSE}
                </div>
              )}

              {/* ImageMagick License */}
              <button
                onClick={() =>
                  setExpandedLicense(
                    expandedLicense === "imagemagick" ? null : "imagemagick"
                  )
                }
                className="w-full p-4 bg-white border-2 border-dark-purple rounded-xl flex items-center justify-between hover:bg-light-bg transition-colors"
              >
                <span className="font-bold text-dark-purple">
                  ImageMagick License
                </span>
                <ChevronDown
                  className={`w-5 h-5 text-dark-purple transition-transform duration-300 ${
                    expandedLicense === "imagemagick" ? "rotate-180" : ""
                  }`}
                />
              </button>
              {expandedLicense === "imagemagick" && (
                <div className="p-4 bg-light-bg rounded-xl text-xs text-secondary whitespace-pre-wrap font-mono max-h-96 overflow-y-auto">
                  {IMAGEMAGICK_LICENSE}
                </div>
              )}

              {/* Lucide License */}
              <button
                onClick={() =>
                  setExpandedLicense(
                    expandedLicense === "lucide" ? null : "lucide"
                  )
                }
                className="w-full p-4 bg-white border-2 border-dark-purple rounded-xl flex items-center justify-between hover:bg-light-bg transition-colors"
              >
                <span className="font-bold text-dark-purple">
                  Lucide License (ISC/MIT)
                </span>
                <ChevronDown
                  className={`w-5 h-5 text-dark-purple transition-transform duration-300 ${
                    expandedLicense === "lucide" ? "rotate-180" : ""
                  }`}
                />
              </button>
              {expandedLicense === "lucide" && (
                <div className="p-4 bg-light-bg rounded-xl text-xs text-secondary whitespace-pre-wrap font-mono max-h-96 overflow-y-auto">
                  {LUCIDE_LICENSE}
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Confirmation Popup */}
      {showConfirmPopup && (
        <div
          className="fixed inset-0 z-[200] flex items-center justify-center bg-dark-purple bg-opacity-50"
          onClick={() => setShowConfirmPopup(false)}
        >
          <div
            className="bg-white rounded-xl shadow-2xl max-w-md w-full mx-4 overflow-hidden"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="p-6 space-y-4">
              <h2 className="text-xl font-bold text-dark-purple">
                Are you sure?
              </h2>
              <p className="text-secondary">
                Install at least one tool to support more conversion formats.
              </p>
              <div className="flex space-x-3 pt-2">
                <button
                  onClick={() => setShowConfirmPopup(false)}
                  className="flex-1 py-3 rounded-xl font-bold border-2 border-dark-purple bg-white text-dark-purple hover:bg-light-bg transition-colors"
                >
                  Go Back
                </button>
                <button
                  onClick={handleConfirmContinue}
                  className="flex-1 py-3 rounded-xl font-bold border-2 border-dark-purple bg-mint-accent text-dark-purple hover:bg-opacity-80 transition-colors"
                >
                  Continue Anyway
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="h-screen bg-light-bg flex flex-col overflow-hidden relative">
        {/* Main Content Area */}
        <div className="flex-1 overflow-y-auto overflow-x-hidden">
          <div className="p-6 flex items-center justify-center min-h-full pt-6">
            <div className="max-w-2xl w-full space-y-6">
              {/* Header */}
              <div className="text-center space-y-2">
                <h1 className="text-3xl font-bold text-dark-purple">
                  Tools & Settings
                </h1>
                <p className="text-lg text-secondary">
                  Download these open-source tools to support the most file
                  formats.
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
                        Best for audio and video conversions.
                      </p>
                    </div>
                    {!toolStatus.ffmpeg.available && (
                      <button
                        onClick={() => downloadTool("ffmpeg")}
                        disabled={downloadingTools.has("ffmpeg")}
                        className="btn-chunky bg-mint-accent border-2 border-dark-purple text-dark-purple px-6 py-3 flex items-center space-x-2"
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
                    <ChevronDown
                      className={`w-4 h-4 transition-transform duration-300 ${
                        ffmpegAdvancedOpen ? "rotate-180" : ""
                      }`}
                    />
                  </button>

                  {/* Animated Advanced Content */}
                  <div
                    className={`grid transition-all duration-300 ease-in-out ${
                      ffmpegAdvancedOpen
                        ? "grid-rows-[1fr] opacity-100"
                        : "grid-rows-[0fr] opacity-0"
                    }`}
                  >
                    <div className="overflow-hidden">
                      <div className="pt-4 space-y-3">
                        <p className="text-sm text-secondary">
                          You can also download{" "}
                          <a
                            href="#"
                            onClick={async (e) => {
                              e.preventDefault();
                              try {
                                await openUrl("https://ffmpeg.org");
                              } catch (err) {
                                setError(`Failed to open link: ${err}`);
                              }
                            }}
                            className="text-blue-accent underline hover:text-dark-purple"
                          >
                            FFmpeg
                          </a>{" "}
                          directly and select a custom path.
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
                    </div>
                  </div>
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
                        Best for image, image to PDF, and HEIC conversions.
                      </p>
                    </div>
                    {!toolStatus.imagemagick.available && (
                      <button
                        onClick={() => downloadTool("imagemagick")}
                        disabled={downloadingTools.has("imagemagick")}
                        className="btn-chunky bg-mint-accent border-2 border-dark-purple text-dark-purple px-6 py-3 flex items-center space-x-2"
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
                    onClick={() =>
                      setImagemagickAdvancedOpen(!imagemagickAdvancedOpen)
                    }
                    className="flex items-center space-x-2 text-dark-purple font-bold hover:text-secondary transition-colors"
                  >
                    <span>Advanced</span>
                    <ChevronDown
                      className={`w-4 h-4 transition-transform duration-300 ${
                        imagemagickAdvancedOpen ? "rotate-180" : ""
                      }`}
                    />
                  </button>

                  {/* Animated Advanced Content */}
                  <div
                    className={`grid transition-all duration-300 ease-in-out ${
                      imagemagickAdvancedOpen
                        ? "grid-rows-[1fr] opacity-100"
                        : "grid-rows-[0fr] opacity-0"
                    }`}
                  >
                    <div className="overflow-hidden">
                      <div className="pt-4 space-y-3">
                        <p className="text-sm text-secondary">
                          You can also download{" "}
                          <a
                            href="#"
                            onClick={async (e) => {
                              e.preventDefault();
                              try {
                                await openUrl("https://imagemagick.org");
                              } catch (err) {
                                setError(`Failed to open link: ${err}`);
                              }
                            }}
                            className="text-blue-accent underline hover:text-dark-purple"
                          >
                            ImageMagick
                          </a>{" "}
                          directly and select a custom path.
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
                    </div>
                  </div>
                </div>
              </div>

              {/* Continue Button */}
              <button
                onClick={handleContinueClick}
                className={`btn-chunky border-2 border-dark-purple text-dark-purple px-8 py-4 text-lg w-full transition-colors ${
                  allToolsReady
                    ? "bg-mint-accent"
                    : "bg-transparent hover:bg-mint-accent"
                }`}
              >
                Continue
              </button>

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
                <div className="bg-mint-accent border-2 border-dark-purple rounded-xl p-4 relative">
                  <div className="flex items-center space-x-3 pr-8">
                    <Check className="w-5 h-5 text-dark-purple" />
                    <p className="font-bold text-dark-purple">
                      {successMessage}
                    </p>
                  </div>
                  <button
                    onClick={() => setSuccessMessage(null)}
                    className="absolute top-3 right-3 w-6 h-6 flex items-center justify-center hover:bg-dark-purple hover:bg-opacity-10 rounded transition-colors"
                    aria-label="Dismiss"
                  >
                    <X className="w-4 h-4 text-dark-purple" />
                  </button>
                </div>
              )}

              {/* Error Message */}
              {error && (
                <div className="bg-pink-accent border-2 border-dark-purple rounded-xl p-4 relative">
                  <div className="flex items-center space-x-3 pr-8">
                    <X className="w-5 h-5 text-dark-purple" />
                    <p className="font-bold text-dark-purple">{error}</p>
                  </div>
                  <button
                    onClick={() => setError(null)}
                    className="absolute top-3 right-3 w-6 h-6 flex items-center justify-center hover:bg-dark-purple hover:bg-opacity-10 rounded transition-colors"
                    aria-label="Dismiss"
                  >
                    <X className="w-4 h-4 text-dark-purple" />
                  </button>
                </div>
              )}

              {/* Product Key Section */}
              <div className="bg-lighter-bg rounded-xl overflow-hidden">
                <button
                  onClick={() => setProductKeyOpen(!productKeyOpen)}
                  className="w-full px-6 py-4 flex items-center justify-between hover:bg-muted-bg transition-colors"
                >
                  <span className="font-bold text-dark-purple">
                    Product Key
                  </span>
                  <ChevronDown
                    className={`w-5 h-5 text-dark-purple transition-transform duration-300 ${
                      productKeyOpen ? "rotate-180" : ""
                    }`}
                  />
                </button>

                {/* Animated Content */}
                <div
                  className={`grid transition-all duration-300 ease-in-out ${
                    productKeyOpen
                      ? "grid-rows-[1fr] opacity-100"
                      : "grid-rows-[0fr] opacity-0"
                  }`}
                >
                  <div className="overflow-hidden">
                    <div className="px-6 pb-6 space-y-4 text-sm text-secondary">
                      <p>
                        Manage your product key. You can change it if you have a
                        different license or purchased a new one with a
                        different email.
                      </p>
                      <div className="flex items-center gap-3">
                        {productKey && (
                          <p className="font-mono text-dark-purple bg-white px-3 py-2 rounded-lg border-2 border-dark-purple select-all flex-1">
                            {productKey}
                          </p>
                        )}
                        <button
                          onClick={openProductKeyModal}
                          className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 hover:bg-light-bg flex items-center space-x-2 whitespace-nowrap"
                        >
                          <Key className="w-4 h-4" />
                          <span>
                            {productKey ? "Change" : "Enter Product Key"}
                          </span>
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Support Section */}
              <div className="bg-lighter-bg rounded-xl overflow-hidden">
                <button
                  onClick={() => setSupportOpen(!supportOpen)}
                  className="w-full px-6 py-4 flex items-center justify-between hover:bg-muted-bg transition-colors"
                >
                  <span className="font-bold text-dark-purple">Support</span>
                  <ChevronDown
                    className={`w-5 h-5 text-dark-purple transition-transform duration-300 ${
                      supportOpen ? "rotate-180" : ""
                    }`}
                  />
                </button>

                {/* Animated Content */}
                <div
                  className={`grid transition-all duration-300 ease-in-out ${
                    supportOpen
                      ? "grid-rows-[1fr] opacity-100"
                      : "grid-rows-[0fr] opacity-0"
                  }`}
                >
                  <div className="overflow-hidden">
                    <div className="px-6 pb-6 space-y-4 text-sm text-secondary">
                      <p>
                        Need help? Check our{" "}
                        <a
                          href="#"
                          onClick={async (e) => {
                            e.preventDefault();
                            try {
                              await openUrl("https://convertsave.com/#faq");
                            } catch (err) {
                              setError(`Failed to open link: ${err}`);
                            }
                          }}
                          className="text-blue-accent underline hover:text-dark-purple"
                        >
                          website
                        </a>{" "}
                        FAQ or email us at{" "}
                        <a
                          href="#"
                          onClick={async (e) => {
                            e.preventDefault();
                            try {
                              await openUrl("mailto:team@convertsave.com");
                            } catch (err) {
                              setError(`Failed to open link: ${err}`);
                            }
                          }}
                          className="text-blue-accent underline hover:text-dark-purple"
                        >
                          team@convertsave.com
                        </a>{" "}
                        and we'll do our best to answer your questions.
                      </p>
                      <p>
                        If you find a bug, you can report it with the button
                        below:
                      </p>
                      <div className="flex flex-wrap gap-3">
                        <button
                          onClick={async () => {
                            try {
                              await openUrl(
                                "https://github.com/Hunter-Boone/ConvertSave-Support/issues"
                              );
                            } catch (err) {
                              setError(`Failed to open link: ${err}`);
                            }
                          }}
                          className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 hover:bg-light-bg flex items-center space-x-2"
                        >
                          <Bug className="w-4 h-4" />
                          <span>Report A Bug</span>
                        </button>
                        <button
                          onClick={async () => {
                            try {
                              await invoke("open_log_directory");
                            } catch (err) {
                              setError(`Failed to open logs: ${err}`);
                            }
                          }}
                          className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-4 py-2 hover:bg-light-bg flex items-center space-x-2"
                        >
                          <FileText className="w-4 h-4" />
                          <span>View Logs</span>
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* License & Attribution Section */}
              <div className="bg-lighter-bg rounded-xl overflow-hidden">
                <button
                  onClick={() =>
                    setLicenseAttributionOpen(!licenseAttributionOpen)
                  }
                  className="w-full px-6 py-4 flex items-center justify-between hover:bg-muted-bg transition-colors"
                >
                  <span className="font-bold text-dark-purple">
                    License & Attribution
                  </span>
                  <ChevronDown
                    className={`w-5 h-5 text-dark-purple transition-transform duration-300 ${
                      licenseAttributionOpen ? "rotate-180" : ""
                    }`}
                  />
                </button>

                {/* Animated Content */}
                <div
                  className={`grid transition-all duration-300 ease-in-out ${
                    licenseAttributionOpen
                      ? "grid-rows-[1fr] opacity-100"
                      : "grid-rows-[0fr] opacity-0"
                  }`}
                >
                  <div className="overflow-hidden">
                    <div className="px-6 pb-6 space-y-6 text-sm text-secondary">
                      <p>
                        FFmpeg and ImageMagick are developed and maintained by
                        their respective authors, who retain all associated
                        copyrights. These tools are downloaded or compiled from
                        official sources and installed separately by the user in
                        order to comply with their licenses.
                      </p>

                      <div className="space-y-3">
                        <h3 className="font-bold text-dark-purple">FFmpeg</h3>
                        <p>
                          FFmpeg is the leading multimedia framework, able to
                          decode, encode, transcode, mux, demux, stream, filter
                          and play pretty much anything that humans and machines
                          have created.
                        </p>
                        <p>
                          FFmpeg is licensed under the GNU Lesser General Public
                          License (LGPL) version 2.1 or later. However, FFmpeg
                          incorporates several optional parts and optimizations
                          that are covered by the GNU General Public License
                          (GPL) version 2 or later. For more information, visit{" "}
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
                          FFmpeg is a trademark of Fabrice Bellard, originator
                          of the FFmpeg project.
                        </p>
                      </div>

                      <div className="space-y-3">
                        <h3 className="font-bold text-dark-purple">
                          ImageMagick
                        </h3>
                        <p>
                          ImageMagick is a free, open-source software suite,
                          used for editing and manipulating digital images.
                        </p>
                        <p>
                          ImageMagick is licensed under the ImageMagick License.
                          For more information, visit{" "}
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
                          Copyright © 1999 ImageMagick Studio LLC, a non-profit
                          organization dedicated to making software imaging
                          solutions freely available.
                        </p>
                      </div>

                      <button
                        onClick={() => setShowLicensesModal(true)}
                        className="btn-chunky bg-white border-2 border-dark-purple text-dark-purple px-6 py-3 hover:bg-light-bg"
                      >
                        Licenses
                      </button>
                    </div>
                  </div>
                </div>
              </div>

              {/* Footer */}
              <div className="pt-6 pb-2 text-center space-y-2">
                <p className="text-sm text-secondary">
                  Copyright © 2025 Pixel & Bracket LLC. All rights reserved.
                </p>
                <div className="flex items-center justify-center space-x-2 text-sm">
                  <a
                    href="#"
                    onClick={async (e) => {
                      e.preventDefault();
                      try {
                        await openUrl(
                          "https://convertsave.com/terms-of-service"
                        );
                      } catch (err) {
                        setError(`Failed to open link: ${err}`);
                      }
                    }}
                    className="text-secondary hover:text-dark-purple transition-colors"
                  >
                    Terms of Service
                  </a>
                  <span className="text-secondary">|</span>
                  <a
                    href="#"
                    onClick={async (e) => {
                      e.preventDefault();
                      try {
                        await openUrl("https://convertsave.com/privacy-policy");
                      } catch (err) {
                        setError(`Failed to open link: ${err}`);
                      }
                    }}
                    className="text-secondary hover:text-dark-purple transition-colors"
                  >
                    Privacy Policy
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
