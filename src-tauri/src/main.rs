// Prevents additional console window on Windows in release builds
// Dev builds (with dev-build feature) will show console for debugging
#![cfg_attr(
    all(not(debug_assertions), not(feature = "dev-build"), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use dirs;
use serde_json;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_updater::UpdaterExt;
use log::{info, error, warn, debug};

// License management module
mod license;

// ═══════════════════════════════════════════════════════════════════════════
// FEATURE TOGGLES - Set to `true` to enable, `false` to disable
// ═══════════════════════════════════════════════════════════════════════════
/// Enable Pandoc document conversion support (Markdown, HTML, TXT conversions)
/// Change this to `true` when you want to re-enable Pandoc functionality
const ENABLE_PANDOC: bool = false;
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// APP IDENTIFIER - Different for dev and production builds
// ═══════════════════════════════════════════════════════════════════════════
/// Returns the app identifier based on build type
/// Dev builds use "com.convertsave.dev", production uses "com.convertsave"
#[cfg(feature = "dev-build")]
const APP_IDENTIFIER: &str = "com.convertsave.dev";

#[cfg(not(feature = "dev-build"))]
const APP_IDENTIFIER: &str = "com.convertsave";
// ═══════════════════════════════════════════════════════════════════════════

// Windows-specific imports to hide console windows
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Serialize, Deserialize)]
struct ConversionOption {
    format: String,
    tool: String,
    display_name: String,
    color: String,
}

#[derive(Serialize, Clone)]
struct DownloadProgress {
    status: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct ToolConfig {
    ffmpeg_path: Option<String>,
    pandoc_path: Option<String>,
    imagemagick_path: Option<String>,
}

/// Get the path to the config file
fn get_config_path() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir().ok_or("Could not find data directory")?;
    let config_dir = data_dir.join(APP_IDENTIFIER);
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    Ok(config_dir.join("config.json"))
}

/// Load the tool configuration from disk
fn load_config() -> Result<ToolConfig, String> {
    let config_path = get_config_path()?;
    if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        let config: ToolConfig = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
        debug!("Loaded config from {}: {:?}", config_path.display(), config);
        Ok(config)
    } else {
        debug!("No config file found at {}, using defaults", config_path.display());
        Ok(ToolConfig::default())
    }
}

/// Save the tool configuration to disk
fn save_config(config: &ToolConfig) -> Result<(), String> {
    let config_path = get_config_path()?;
    let contents = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, &contents).map_err(|e| e.to_string())?;
    info!("Config saved to {}: {}", config_path.display(), contents);
    Ok(())
}

/// Helper function to create a Command that doesn't show a console window on Windows
fn create_command<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
    #[cfg(target_os = "windows")]
    let mut command = Command::new(program);
    #[cfg(not(target_os = "windows"))]
    let command = Command::new(program);
    
    #[cfg(target_os = "windows")]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }
    
    command
}

/// Get the log directory path
#[tauri::command]
async fn check_app_update(app: AppHandle) -> Result<bool, String> {
    match app.updater() {
        Ok(updater) => {
            match updater.check().await {
                Ok(Some(_update)) => Ok(true),
                Ok(None) => Ok(false),
                Err(e) => {
                    error!("Failed to check for updates: {}", e);
                    Ok(false)
                }
            }
        }
        Err(e) => {
            error!("Failed to initialize updater: {}", e);
            Ok(false)
        }
    }
}

#[tauri::command]
async fn install_app_update(app: AppHandle) -> Result<(), String> {
    match app.updater() {
        Ok(updater) => {
            match updater.check().await {
                Ok(Some(update)) => {
                    let mut downloaded = 0;
                    
                    // Download and install the update with progress tracking
                    update.download_and_install(
                        |chunk_length, _content_len| {
                            downloaded += chunk_length;
                            info!("Downloaded {} bytes", downloaded);
                        },
                        || {
                            info!("Download finished");
                        },
                    ).await.map_err(|e| format!("Failed to install update: {}", e))?;
                    
                    info!("Update installed successfully, restarting...");
                    app.restart();
                }
                Ok(None) => Err("No update available".to_string()),
                Err(e) => Err(format!("Failed to check for updates: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to initialize updater: {}", e))
    }
}

#[tauri::command]
fn get_log_directory(app: AppHandle) -> Result<String, String> {
    let log_dir = app.path().app_log_dir().map_err(|e| e.to_string())?;
    Ok(log_dir.to_string_lossy().to_string())
}

/// Open the log directory in the system file explorer
#[tauri::command]
async fn open_log_directory(app: AppHandle) -> Result<(), String> {
    let log_dir = app.path().app_log_dir().map_err(|e| e.to_string())?;
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&log_dir).map_err(|e| e.to_string())?;
    
    info!("Opening log directory: {}", log_dir.display());
    
    // Open the directory - note: we ignore the exit code as it varies by platform
    #[cfg(target_os = "windows")]
    {
        let _ = create_command("explorer")
            .arg(&log_dir)
            .spawn();
    }
    
    #[cfg(target_os = "macos")]
    {
        let _ = create_command("open")
            .arg(&log_dir)
            .spawn();
    }
    
    #[cfg(target_os = "linux")]
    {
        let _ = create_command("xdg-open")
            .arg(&log_dir)
            .spawn();
    }
    
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_available_formats(input_extension: String) -> Vec<ConversionOption> {
    // Debug logging
    info!("Getting available formats for extension: '{}'", input_extension);
    
    // This is a simplified version - in production, you'd have more sophisticated mapping
    let mut options = Vec::new();
    
    // Convert to lowercase for case-insensitive matching
    let input_extension = input_extension.to_lowercase();
    
    match input_extension.as_str() {
        // Video formats
        "mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" | "wmv" | "m4v" | "mpg" | "mpeg" | "3gp" => {
            // Video output formats
            if input_extension != "mp4" {
                options.push(ConversionOption {
                    format: "mp4".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "MP4 Video".to_string(),
                    color: "blue".to_string(),
                });
            }
            if input_extension != "mov" {
                options.push(ConversionOption {
                    format: "mov".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "QuickTime Video".to_string(),
                    color: "blue".to_string(),
                });
            }
            if input_extension != "avi" {
                options.push(ConversionOption {
                    format: "avi".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "AVI Video".to_string(),
                    color: "blue".to_string(),
                });
            }
            if input_extension != "mkv" {
                options.push(ConversionOption {
                    format: "mkv".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Matroska Video".to_string(),
                    color: "blue".to_string(),
                });
            }
            if input_extension != "webm" {
                options.push(ConversionOption {
                    format: "webm".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "WebM Video".to_string(),
                    color: "green".to_string(),
                });
            }
            // GIF from video
            options.push(ConversionOption {
                format: "gif".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "Animated GIF".to_string(),
                color: "pink".to_string(),
            });
            // Audio extraction
            options.push(ConversionOption {
                format: "mp3".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "MP3 Audio".to_string(),
                color: "green".to_string(),
            });
            options.push(ConversionOption {
                format: "wav".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "WAV Audio".to_string(),
                color: "light-tan".to_string(),
            });
            options.push(ConversionOption {
                format: "flac".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "FLAC Audio (Lossless)".to_string(),
                color: "aquamarine".to_string(),
            });
            options.push(ConversionOption {
                format: "ogg".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "OGG Audio".to_string(),
                color: "orange".to_string(),
            });
            options.push(ConversionOption {
                format: "m4a".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "M4A Audio".to_string(),
                color: "light-purple".to_string(),
            });
            options.push(ConversionOption {
                format: "aac".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "AAC Audio".to_string(),
                color: "yellow".to_string(),
            });
        }
        // Audio formats
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "wma" | "aac" => {
            if input_extension != "mp3" {
                options.push(ConversionOption {
                    format: "mp3".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "MP3 Audio".to_string(),
                    color: "green".to_string(),
                });
            }
            if input_extension != "wav" {
                options.push(ConversionOption {
                    format: "wav".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "WAV Audio".to_string(),
                    color: "light-tan".to_string(),
                });
            }
            if input_extension != "flac" {
                options.push(ConversionOption {
                    format: "flac".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "FLAC Audio (Lossless)".to_string(),
                    color: "aquamarine".to_string(),
                });
            }
            if input_extension != "ogg" {
                options.push(ConversionOption {
                    format: "ogg".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "OGG Audio".to_string(),
                    color: "orange".to_string(),
                });
            }
            if input_extension != "m4a" {
                options.push(ConversionOption {
                    format: "m4a".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "M4A Audio".to_string(),
                    color: "light-purple".to_string(),
                });
            }
            if input_extension != "aac" {
                options.push(ConversionOption {
                    format: "aac".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "AAC Audio".to_string(),
                    color: "yellow".to_string(),
                });
            }
        }
        "docx" | "doc" | "odt" => {
            options.push(ConversionOption {
                format: "pdf".to_string(),
                tool: "libreoffice".to_string(),
                display_name: "PDF Document".to_string(),
                color: "pink".to_string(),
            });
            if ENABLE_PANDOC {
                options.push(ConversionOption {
                    format: "epub".to_string(),
                    tool: "pandoc".to_string(),
                    display_name: "E-Book".to_string(),
                    color: "blue".to_string(),
                });
                options.push(ConversionOption {
                    format: "txt".to_string(),
                    tool: "pandoc".to_string(),
                    display_name: "Plain Text".to_string(),
                    color: "lavender".to_string(),
                });
            }
        }
        "md" | "markdown" if ENABLE_PANDOC => {
            // Markdown can convert to many formats via Pandoc
            options.push(ConversionOption {
                format: "html".to_string(),
                tool: "pandoc".to_string(),
                display_name: "HTML Document".to_string(),
                color: "orange".to_string(),
            });
            options.push(ConversionOption {
                format: "docx".to_string(),
                tool: "pandoc".to_string(),
                display_name: "Word Document".to_string(),
                color: "blue".to_string(),
            });
            options.push(ConversionOption {
                format: "epub".to_string(),
                tool: "pandoc".to_string(),
                display_name: "E-Book".to_string(),
                color: "pink".to_string(),
            });
            options.push(ConversionOption {
                format: "txt".to_string(),
                tool: "pandoc".to_string(),
                display_name: "Plain Text".to_string(),
                color: "lavender".to_string(),
            });
        }
        "html" | "htm" if ENABLE_PANDOC => {
            // HTML can convert via Pandoc
            options.push(ConversionOption {
                format: "md".to_string(),
                tool: "pandoc".to_string(),
                display_name: "Markdown".to_string(),
                color: "blue".to_string(),
            });
            options.push(ConversionOption {
                format: "docx".to_string(),
                tool: "pandoc".to_string(),
                display_name: "Word Document".to_string(),
                color: "blue".to_string(),
            });
            options.push(ConversionOption {
                format: "epub".to_string(),
                tool: "pandoc".to_string(),
                display_name: "E-Book".to_string(),
                color: "pink".to_string(),
            });
            options.push(ConversionOption {
                format: "txt".to_string(),
                tool: "pandoc".to_string(),
                display_name: "Plain Text".to_string(),
                color: "lavender".to_string(),
            });
        }
        "txt" if ENABLE_PANDOC => {
            // Plain text can convert via Pandoc
            options.push(ConversionOption {
                format: "md".to_string(),
                tool: "pandoc".to_string(),
                display_name: "Markdown".to_string(),
                color: "blue".to_string(),
            });
            options.push(ConversionOption {
                format: "html".to_string(),
                tool: "pandoc".to_string(),
                display_name: "HTML Document".to_string(),
                color: "orange".to_string(),
            });
            options.push(ConversionOption {
                format: "docx".to_string(),
                tool: "pandoc".to_string(),
                display_name: "Word Document".to_string(),
                color: "blue".to_string(),
            });
            options.push(ConversionOption {
                format: "epub".to_string(),
                tool: "pandoc".to_string(),
                display_name: "E-Book".to_string(),
                color: "pink".to_string(),
            });
        }
        // Standard image formats + Modern + Professional + Legacy + RAW + Animation
        "png" | "jpg" | "jpeg" | "bmp" | "tiff" | "tif" | "webp" | "gif" | 
        // Modern formats
        "heic" | "heif" | "avif" | "jxl" |
        // Professional/High-end
        "tga" | "ppm" | "pgm" | "pbm" | "pam" | "xbm" | "xpm" | "dds" | "dpx" | "exr" | "hdr" | "ico" | "j2k" | "jp2" | "pcx" | "pfm" | "sgi" | "sun" | "xwd" |
        // Adobe/Photoshop
        "psd" | "psb" |
        // Vector (rasterized)
        "svg" | "svgz" |
        // Animation
        "apng" |
        // GIMP
        "xcf" |
        // Windows
        "cur" | "emf" | "wmf" |
        // Camera RAW formats
        "arw" | "cr2" | "cr3" | "crw" | "dng" | "nef" | "nrw" | "orf" | "raf" | "raw" | "rw2" | "rwl" | "srw" => {
            // Standard formats
            // JPG <-> JPEG simple rename conversions
            if input_extension == "jpg" {
                options.push(ConversionOption {
                    format: "jpeg".to_string(),
                    tool: "rename".to_string(),
                    display_name: "JPEG (rename extension)".to_string(),
                    color: "yellow".to_string(),
                });
            }
            if input_extension == "jpeg" {
                options.push(ConversionOption {
                    format: "jpg".to_string(),
                    tool: "rename".to_string(),
                    display_name: "JPG (rename extension)".to_string(),
                    color: "yellow".to_string(),
                });
            }
            
            if input_extension != "jpg" && input_extension != "jpeg" {
                options.push(ConversionOption {
                    format: "jpg".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "JPEG Image (.jpg)".to_string(),
                    color: "yellow".to_string(),
                });
                options.push(ConversionOption {
                    format: "jpeg".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "JPEG Image (.jpeg)".to_string(),
                    color: "yellow".to_string(),
                });
            }
            if input_extension != "png" {
                options.push(ConversionOption {
                    format: "png".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "PNG Image".to_string(),
                    color: "orange".to_string(),
                });
            }
            if input_extension != "gif" {
                options.push(ConversionOption {
                    format: "gif".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "GIF Image".to_string(),
                    color: "blue".to_string(),
                });
            }
            if input_extension != "bmp" {
                options.push(ConversionOption {
                    format: "bmp".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Bitmap Image".to_string(),
                    color: "light-purple".to_string(),
                });
            }
            if input_extension != "webp" {
                options.push(ConversionOption {
                    format: "webp".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "WebP Image".to_string(),
                    color: "green".to_string(),
                });
            }
            if input_extension != "tiff" {
                options.push(ConversionOption {
                    format: "tiff".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "TIFF Image".to_string(),
                    color: "light-tan".to_string(),
                });
            }
            
            // Modern formats
            // HEIC/HEIF encoding now supported via ImageMagick
            if input_extension != "heic" && input_extension != "heif" {
                options.push(ConversionOption {
                    format: "heic".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "HEIC (High Efficiency)".to_string(),
                    color: "pink".to_string(),
                });
                options.push(ConversionOption {
                    format: "heif".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "HEIF (High Efficiency)".to_string(),
                    color: "pink".to_string(),
                });
            }
            if input_extension != "avif" {
                options.push(ConversionOption {
                    format: "avif".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "AVIF (AV1 Image)".to_string(),
                    color: "aquamarine".to_string(),
                });
            }
            if input_extension != "jxl" {
                options.push(ConversionOption {
                    format: "jxl".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "JPEG XL".to_string(),
                    color: "aquamarine".to_string(),
                });
            }
            
            // Professional/High-end formats
            if input_extension != "tga" {
                options.push(ConversionOption {
                    format: "tga".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Targa Image".to_string(),
                    color: "pink".to_string(),
                });
            }
            if input_extension != "exr" {
                options.push(ConversionOption {
                    format: "exr".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "OpenEXR (HDR)".to_string(),
                    color: "aquamarine".to_string(),
                });
            }
            if input_extension != "hdr" {
                options.push(ConversionOption {
                    format: "hdr".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Radiance HDR".to_string(),
                    color: "aquamarine".to_string(),
                });
            }
            if input_extension != "dpx" {
                options.push(ConversionOption {
                    format: "dpx".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Digital Picture Exchange".to_string(),
                    color: "pink".to_string(),
                });
            }
            
            // JPEG 2000 formats
            if input_extension != "j2k" && input_extension != "jp2" {
                options.push(ConversionOption {
                    format: "j2k".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "JPEG 2000".to_string(),
                    color: "yellow".to_string(),
                });
            }
            
            // Legacy/Specialized formats
            if input_extension != "pcx" {
                options.push(ConversionOption {
                    format: "pcx".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "PCX Image".to_string(),
                    color: "light-purple".to_string(),
                });
            }
            if input_extension != "ico" {
                options.push(ConversionOption {
                    format: "ico".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Windows Icon".to_string(),
                    color: "blue".to_string(),
                });
            }
            if input_extension != "sgi" {
                options.push(ConversionOption {
                    format: "sgi".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Silicon Graphics Image".to_string(),
                    color: "green".to_string(),
                });
            }
            
            // Raw/Uncompressed formats
            if input_extension != "ppm" {
                options.push(ConversionOption {
                    format: "ppm".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Portable Pixmap".to_string(),
                    color: "light-tan".to_string(),
                });
            }
            if input_extension != "pgm" {
                options.push(ConversionOption {
                    format: "pgm".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Portable Graymap".to_string(),
                    color: "light-tan".to_string(),
                });
            }
            if input_extension != "pbm" {
                options.push(ConversionOption {
                    format: "pbm".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Portable Bitmap".to_string(),
                    color: "light-tan".to_string(),
                });
            }
            if input_extension != "pam" {
                options.push(ConversionOption {
                    format: "pam".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "Portable Arbitrary Map".to_string(),
                    color: "light-tan".to_string(),
                });
            }
            
            // X Window System formats
            if input_extension != "xbm" {
                options.push(ConversionOption {
                    format: "xbm".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "X11 Bitmap".to_string(),
                    color: "light-purple".to_string(),
                });
            }
            if input_extension != "xpm" {
                options.push(ConversionOption {
                    format: "xpm".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "X11 Pixmap".to_string(),
                    color: "light-purple".to_string(),
                });
            }
            if input_extension != "xwd" {
                options.push(ConversionOption {
                    format: "xwd".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "X Window Dump".to_string(),
                    color: "light-purple".to_string(),
                });
            }
            
            // Gaming/3D formats
            if input_extension != "dds" {
                options.push(ConversionOption {
                    format: "dds".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "DirectDraw Surface".to_string(),
                    color: "blue".to_string(),
                });
            }
            
            // Adobe/Photoshop format
            if input_extension != "psd" && input_extension != "psb" {
                options.push(ConversionOption {
                    format: "psd".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "Photoshop Document".to_string(),
                    color: "blue".to_string(),
                });
            }
            
            // Animation format
            if input_extension != "apng" {
                options.push(ConversionOption {
                    format: "apng".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "Animated PNG".to_string(),
                    color: "orange".to_string(),
                });
            }
            
            // Windows cursor format
            if input_extension != "cur" {
                options.push(ConversionOption {
                    format: "cur".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "Windows Cursor".to_string(),
                    color: "blue".to_string(),
                });
            }
            
            // Document format - PDF output from images
            options.push(ConversionOption {
                format: "pdf".to_string(),
                tool: "imagemagick".to_string(),
                display_name: "PDF Document".to_string(),
                color: "pink".to_string(),
            });
        }
        _ => {
            info!("No conversion options found for extension: '{}'", input_extension);
        }
    }
    
    info!("Found {} format options for '{}'", options.len(), input_extension);
    options
}

/// Generate a unique file path by adding a numbered suffix if the file already exists
/// Example: "file.png" -> "file (1).png" -> "file (2).png" etc.
fn get_unique_output_path(base_dir: &PathBuf, file_stem: &str, extension: &str) -> PathBuf {
    let initial_path = base_dir.join(format!("{}.{}", file_stem, extension));
    
    // If the file doesn't exist, use the original name
    if !initial_path.exists() {
        return initial_path;
    }
    
    // File exists, so find the next available number
    let mut counter = 1;
    loop {
        let numbered_path = base_dir.join(format!("{} ({}).{}", file_stem, counter, extension));
        if !numbered_path.exists() {
            return numbered_path;
        }
        counter += 1;
        
        // Safety check to prevent infinite loop (though unlikely to reach this)
        if counter > 10000 {
            // Fall back to timestamp-based naming
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return base_dir.join(format!("{} ({}).{}", file_stem, timestamp, extension));
        }
    }
}

#[tauri::command]
async fn convert_file(
    input_path: String,
    output_format: String,
    output_directory: Option<String>,
    advanced_options: Option<String>,
) -> Result<String, String> {
    // Log conversion details
    info!("Starting conversion: {} -> {}", input_path, output_format);
    info!("Output directory: {:?}", output_directory);
    if let Some(ref opts) = advanced_options {
        info!("Advanced options: {}", opts);
    }
    
    let input_path = PathBuf::from(&input_path);
    let file_stem = input_path.file_stem()
        .ok_or("Invalid input file")?
        .to_str()
        .ok_or("Invalid file name")?;
    
    let input_extension = input_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let output_dir = if let Some(dir) = output_directory {
        PathBuf::from(dir)
    } else {
        // Default to the same directory as the input file
        input_path.parent()
            .ok_or("Could not determine input file directory")?
            .to_path_buf()
    };
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;
    
    // Get a unique output path that won't overwrite existing files
    let output_path = get_unique_output_path(&output_dir, file_stem, &output_format);
    
    // Determine which tool to use and perform the actual conversion
    let output_format_lower = output_format.to_lowercase();
    let conversion_result = match determine_conversion_tool(&input_extension, &output_format_lower) {
        Some(tool) => {
            execute_conversion(tool, &input_path, &output_path, advanced_options).await
        }
        None => {
            let error_msg = format!("No conversion tool available for {} to {}", input_extension, output_format);
            error!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    match conversion_result {
        Ok(_) => {
            info!("Conversion completed successfully: {}", output_path.display());
            // Return the actual output path so the frontend can use it
            Ok(output_path.to_string_lossy().to_string())
        }
        Err(e) => {
            error!("Conversion failed: {}", e);
            Err(e)
        }
    }
}

/// Convert multiple images into a single multipage PDF
#[tauri::command]
async fn convert_images_to_multipage_pdf(
    input_paths: Vec<String>,
    output_directory: Option<String>,
) -> Result<String, String> {
    info!("Starting multipage PDF conversion with {} images", input_paths.len());
    
    if input_paths.is_empty() {
        return Err("No input files provided".to_string());
    }
    
    // Convert string paths to PathBuf
    let input_paths: Vec<PathBuf> = input_paths.iter().map(PathBuf::from).collect();
    
    // Verify all input files exist
    for path in &input_paths {
        if !path.exists() {
            return Err(format!("Input file not found: {}", path.display()));
        }
    }
    
    // Determine output directory - use the directory of the first file if not specified
    let output_dir = if let Some(dir) = output_directory {
        PathBuf::from(dir)
    } else {
        input_paths[0]
            .parent()
            .ok_or("Could not determine output directory")?
            .to_path_buf()
    };
    
    // Generate output filename based on first file name
    let first_file_stem = input_paths[0]
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("combined");
    
    // Get a unique output path
    let output_path = get_unique_output_path(&output_dir, &format!("{}_multipage", first_file_stem), "pdf");
    
    // Get ImageMagick path
    let tool_path = get_tool_path("imagemagick")
        .map_err(|e| format!("ImageMagick is required for multipage PDF creation: {}", e))?;
    
    // Build ImageMagick command: magick input1.jpg input2.png ... output.pdf
    let mut command = create_command(&tool_path);
    
    // On macOS, set environment variables for ImageMagick to find its bundled libraries
    #[cfg(target_os = "macos")]
    {
        // tool_path is: ~/Library/Application Support/com.convertsave/imagemagick/bin/magick
        if let Some(bin_dir) = tool_path.parent() {
            if let Some(imagemagick_dir) = bin_dir.parent() {
                let lib_dir = imagemagick_dir.join("lib");
                let etc_dir = imagemagick_dir.join("etc").join("ImageMagick-7");
                
                info!("Setting DYLD_LIBRARY_PATH: {}", lib_dir.display());
                info!("Setting MAGICK_HOME: {}", imagemagick_dir.display());
                
                command.env("DYLD_LIBRARY_PATH", &lib_dir);
                command.env("MAGICK_HOME", &imagemagick_dir);
                
                // Set configuration path if it exists
                if etc_dir.exists() {
                    info!("Setting MAGICK_CONFIGURE_PATH: {}", etc_dir.display());
                    command.env("MAGICK_CONFIGURE_PATH", &etc_dir);
                } else {
                    warn!("Configuration directory not found: {}", etc_dir.display());
                }
                
                // Set module path for builds with --with-modules enabled
                if let Ok(entries) = std::fs::read_dir(&lib_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() && path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.starts_with("ImageMagick-"))
                            .unwrap_or(false) {
                            let modules_coders = path.join("modules-Q16HDRI").join("coders");
                            if modules_coders.exists() {
                                info!("Setting MAGICK_CODER_MODULE_PATH: {}", modules_coders.display());
                                command.env("MAGICK_CODER_MODULE_PATH", &modules_coders);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Add all input files
    for input_path in &input_paths {
        command.arg(input_path);
    }
    
    // Add PDF-specific options for good quality output
    command.arg("-compress").arg("jpeg");  // Use JPEG compression for images
    command.arg("-quality").arg("85");     // Good quality/size balance
    command.arg("-density").arg("300");    // 300 DPI for print quality
    
    // Add output path
    command.arg(&output_path);
    
    info!("Executing ImageMagick multipage PDF command...");
    
    let output = command.output()
        .map_err(|e| format!("Failed to execute ImageMagick: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        error!("ImageMagick multipage PDF failed - stderr: {}", stderr);
        error!("ImageMagick multipage PDF failed - stdout: {}", stdout);
        return Err(format!("Failed to create multipage PDF: {}", stderr));
    }
    
    info!("Multipage PDF created successfully: {}", output_path.display());
    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn get_file_info(path: String) -> Result<serde_json::Value, String> {
    let path = PathBuf::from(&path);
    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    let file_name = path.file_name()
        .ok_or("Could not get file name")?
        .to_str()
        .ok_or("Invalid file name")?;
    
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let file_info = serde_json::json!({
        "name": file_name,
        "size": metadata.len(),
        "extension": extension
    });
    
    Ok(file_info)
}

#[tauri::command]
async fn test_directories() -> Result<serde_json::Value, String> {
    let mut info = serde_json::Map::new();
    
    if let Some(docs) = dirs::document_dir() {
        info.insert("documents".to_string(), serde_json::Value::String(docs.to_string_lossy().to_string()));
    } else {
        info.insert("documents".to_string(), serde_json::Value::Null);
    }
    
    if let Some(home) = dirs::home_dir() {
        info.insert("home".to_string(), serde_json::Value::String(home.to_string_lossy().to_string()));
    } else {
        info.insert("home".to_string(), serde_json::Value::Null);
    }
    
    if let Ok(current) = std::env::current_dir() {
        info.insert("current".to_string(), serde_json::Value::String(current.to_string_lossy().to_string()));
    } else {
        info.insert("current".to_string(), serde_json::Value::Null);
    }
    
    Ok(serde_json::Value::Object(info))
}

#[tauri::command]
async fn open_folder(path: String) -> Result<(), String> {
    let path = PathBuf::from(path);
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, if it's a file, use /select, to open Explorer and highlight the file
        // If it's a folder, just open the folder normally
        if path.is_file() {
            create_command("explorer")
                .arg("/select,")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        } else {
            create_command("explorer")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // On macOS, use -R flag to reveal the file in Finder
        if path.is_file() {
            create_command("open")
                .arg("-R")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        } else {
            create_command("open")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // On Linux, most file managers don't have a standard "reveal" option
        // So we open the parent folder
        let folder = if path.is_file() {
            path.parent().ok_or("Could not find parent folder")?
        } else {
            &path
        };
        
        create_command("xdg-open")
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    Ok(())
}

fn determine_conversion_tool(input_ext: &str, output_ext: &str) -> Option<&'static str> {
    // JPG <-> JPEG simple rename (no conversion needed, same format)
    if (input_ext == "jpg" && output_ext == "jpeg") || (input_ext == "jpeg" && output_ext == "jpg") {
        return Some("rename");
    }
    
    // Image conversions - ImageMagick supports the widest range of formats
    // FFmpeg is used as fallback for some formats
    let image_inputs = [
        // Standard/Common formats
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
        // Modern formats
        "heic", "heif", "avif", "jxl",
        // Professional/High-end formats
        "tga", "exr", "hdr", "dpx", "pfm", "psd", "psb",
        // JPEG variants
        "j2k", "jp2", "jpc", "jpf", "jpx", "jpm",
        // Legacy/Specialized formats
        "pcx", "ico", "sgi", "sun", "ras", "pict", "pct",
        // Raw/Uncompressed formats
        "ppm", "pgm", "pbm", "pam", "pnm", "pfm",
        // X Window System formats
        "xbm", "xpm", "xwd",
        // Gaming/3D formats
        "dds", "tga", "vtf",
        // Vector/Document formats (rasterized)
        "svg", "svgz", "ai", "eps", "ps", "pdf",
        // Digital camera RAW formats
        "arw", "cr2", "cr3", "crw", "dng", "nef", "nrw", "orf", "raf", "raw", "rw2", "rwl", "srw",
        // Animation formats
        "mng", "apng",
        // Windows formats
        "cur", "dib", "emf", "wmf",
        // Adobe formats
        "psd", "psb", "ai",
        // Other formats
        "fits", "flif", "jbig", "jng", "miff", "otb", "pal", "palm", "pam", "pcd", "pict", 
        "pix", "plasma", "pnm", "pwp", "rgf", "sfw", "sgi", "sun", "tga", "uyvy", "vicar", 
        "viff", "wbmp", "xbm", "xcf", "xpm", "xv", "yuv"
    ];
    let image_outputs_ffmpeg = [
        // Standard formats
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
        // Modern formats (AVIF only, HEIC/HEIF use ImageMagick)
        "avif",
        // Professional/High-end formats
        "tga", "exr", "hdr", "dpx", "pfm",
        // JPEG 2000
        "j2k", "jp2",
        // Legacy/Specialized formats
        "pcx", "ico", "sgi", "sun",
        // Raw/Uncompressed formats
        "ppm", "pgm", "pbm", "pam",
        // Gaming/3D formats
        "dds"
        // Note: X Window System formats (xbm, xpm, xwd) require ImageMagick
    ];
    let image_outputs_imagemagick = [
        // Standard/Common formats
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
        // Modern formats - ImageMagick supports HEIC/HEIF encoding
        "heic", "heif", "avif", "jxl",
        // Professional/High-end formats
        "tga", "exr", "hdr", "dpx", "pfm", "psd", "psb",
        // JPEG variants
        "j2k", "jp2", "jpc", "jpf", "jpx", "jpm",
        // Legacy/Specialized formats
        "pcx", "ico", "sgi", "sun", "ras", "pict", "pct",
        // Raw/Uncompressed formats
        "ppm", "pgm", "pbm", "pam", "pnm",
        // X Window System formats
        "xbm", "xpm", "xwd",
        // Gaming/3D formats
        "dds", "tga", "vtf",
        // Vector/Document formats (rasterized)
        "svg", "svgz", "pdf",
        // Animation formats
        "mng", "apng",
        // Windows formats
        "cur", "dib", "emf", "wmf",
        // Adobe formats
        "psd", "psb",
        // Other formats
        "fits", "jbig", "jng", "miff", "otb", "pal", "palm", "pcd", "pict", 
        "pix", "plasma", "sfw", "wbmp", "xcf", "xv", "yuv"
    ];
    
    // Video/Audio conversions
    let video_inputs = ["mp4", "mov", "avi", "mkv", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "3gp"];
    let audio_inputs = ["mp3", "wav", "flac", "ogg", "m4a", "wma", "aac"];
    let av_outputs = ["mp4", "mov", "avi", "mkv", "webm", "mp3", "wav", "flac", "ogg", "m4a", "aac", "gif"];
    
    // Document conversions
    // Note: PDF output requires LaTeX (not included), so it's removed from Pandoc outputs
    let doc_inputs = ["md", "markdown", "txt", "html", "htm", "docx", "odt", "rtf", "tex", "latex", "epub", "rst"];
    let doc_outputs = ["md", "html", "docx", "odt", "rtf", "tex", "epub", "txt"];
    
    let office_inputs = ["doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf"];
    let office_outputs = ["pdf", "html", "txt", "docx", "odt", "rtf"];
    
    // Use ffmpeg for media and image conversions
    if (video_inputs.contains(&input_ext) || audio_inputs.contains(&input_ext)) && av_outputs.contains(&output_ext) {
        Some("ffmpeg")
    } else if image_inputs.contains(&input_ext) && (output_ext == "heic" || output_ext == "heif") {
        // HEIC/HEIF encoding requires ImageMagick
        Some("imagemagick")
    } else if image_inputs.contains(&input_ext) && (output_ext == "xbm" || output_ext == "xpm" || output_ext == "xwd") {
        // X Window System formats require ImageMagick (FFmpeg doesn't support them properly)
        Some("imagemagick")
    } else if image_inputs.contains(&input_ext) && image_outputs_imagemagick.contains(&output_ext) {
        // Try ImageMagick first for image conversions, but will fallback to FFmpeg if not available
        Some("imagemagick")
    } else if image_inputs.contains(&input_ext) && image_outputs_ffmpeg.contains(&output_ext) {
        // Fallback to ffmpeg for formats ImageMagick doesn't support well
        Some("ffmpeg")
    } else if ENABLE_PANDOC && doc_inputs.contains(&input_ext) && doc_outputs.contains(&output_ext) {
        Some("pandoc")
    } else if office_inputs.contains(&input_ext) && office_outputs.contains(&output_ext) {
        Some("libreoffice")
    } else {
        None
    }
}

fn get_tool_path(tool_name: &str) -> Result<PathBuf, String> {
    // Check for custom path first
    if let Ok(mut config) = load_config() {
        let custom_path = match tool_name {
            "ffmpeg" => &config.ffmpeg_path,
            "pandoc" => &config.pandoc_path,
            "imagemagick" => &config.imagemagick_path,
            _ => &None,
        };
        
        if let Some(path_str) = custom_path {
            let path = PathBuf::from(&path_str);
            info!("Checking custom path for {}: {}", tool_name, path.display());
            if path.exists() {
                info!("Using custom path for {}: {}", tool_name, path.display());
                return Ok(path);
            } else {
                warn!("Custom path for {} no longer exists: {}. Clearing from config.", tool_name, path.display());
                // Clear the invalid custom path from config
                match tool_name {
                    "ffmpeg" => config.ffmpeg_path = None,
                    "pandoc" => config.pandoc_path = None,
                    "imagemagick" => config.imagemagick_path = None,
                    _ => {}
                }
                // Save the updated config (ignore errors as this is cleanup)
                let _ = save_config(&config);
            }
        }
    }
    
    let platform_name = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    
    let exe_name = match tool_name {
        "ffmpeg" => {
            if cfg!(target_os = "windows") {
                "ffmpeg.exe"
            } else {
                "ffmpeg"
            }
        }
        "pandoc" if ENABLE_PANDOC => {
            if cfg!(target_os = "windows") {
                "pandoc.exe"
            } else {
                "pandoc"
            }
        }
        "imagemagick" => {
            if cfg!(target_os = "windows") {
                "magick.exe"
            } else {
                "magick"
            }
        }
        _ => return Err(format!("Unknown tool: {}", tool_name)),
    };
    
    // Try multiple possible locations
    let mut possible_paths = vec![];
    
    // 1. App data directory (downloaded binaries) - CHECK THIS FIRST
    // NOTE: This must match the path used in download_ffmpeg/download_pandoc
    // We can't use app.path().app_data_dir() here since we don't have AppHandle,
    // so we manually construct the same path that Tauri uses
    if let Some(data_dir) = dirs::data_dir() {
        // Tauri's app_data_dir() uses: {data_dir}/{identifier}
        // APP_IDENTIFIER is "com.convertsave" for prod, "com.convertsave.dev" for dev
        
        // For ImageMagick on macOS, the binary is in bin/ subdirectory as per official structure
        #[cfg(target_os = "macos")]
        if tool_name == "imagemagick" {
            let app_data_path = data_dir
                .join(APP_IDENTIFIER)
                .join(tool_name)
                .join("bin")
                .join(exe_name);
            possible_paths.push(app_data_path);
        } else {
            let app_data_path = data_dir
                .join(APP_IDENTIFIER)
                .join(tool_name)
                .join(exe_name);
            possible_paths.push(app_data_path);
        }
        
        // For other platforms, use flat structure
        #[cfg(not(target_os = "macos"))]
        {
            let app_data_path = data_dir
                .join(APP_IDENTIFIER)
                .join(tool_name)
                .join(exe_name);
            possible_paths.push(app_data_path);
        }
    }
    
    // 2. Project root tools directory (development only)
    if let Ok(current) = std::env::current_dir() {
        possible_paths.push(current.join("tools").join(platform_name).join(exe_name));
    }
    
    // 3. Check if we're in src-tauri directory during development
    if let Ok(current) = std::env::current_dir() {
        if let Some(parent) = current.parent() {
            possible_paths.push(parent.join("tools").join(platform_name).join(exe_name));
        }
    }
    
    // 4. On macOS, check Homebrew locations as fallback (Apple Silicon and Intel)
    #[cfg(target_os = "macos")]
    {
        // Apple Silicon Homebrew
        possible_paths.push(PathBuf::from("/opt/homebrew/bin").join(exe_name));
        // Intel Homebrew
        possible_paths.push(PathBuf::from("/usr/local/bin").join(exe_name));
    }
    
    // On macOS, NEVER check inside the .app bundle - it's read-only and code-signed
    // On Windows/Linux, we can check relative to executable for bundled binaries
    #[cfg(not(target_os = "macos"))]
    {
        // Relative to executable (production)
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                possible_paths.push(parent.join("tools").join(platform_name).join(exe_name));
            }
        }
        
        // Parent directory of executable + tools (alternative production layout)
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent().and_then(|p| p.parent()) {
                possible_paths.push(parent.join("tools").join(platform_name).join(exe_name));
            }
        }
    }
    
    for path in &possible_paths {
        if path.exists() {
            return Ok(path.clone());
        }
    }
    
    // If none found, list all the paths we checked
    let checked_paths: Vec<String> = possible_paths.iter()
        .map(|p| p.display().to_string())
        .collect();
    
    let error_msg = format!("Tool not found: {} (checked: {})", tool_name, checked_paths.join(", "));
    warn!("{}", error_msg);
    Err(error_msg)
}

// Helper function to handle HEIC tile grid reassembly
fn convert_heic_with_tiles(
    tool_path: &PathBuf,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), String> {
    // Step 1: Get metadata to find tile grid dimensions and rotation
    let metadata_output = create_command(tool_path)
        .arg("-i")
        .arg(input_path)
        .arg("-f")
        .arg("null")
        .arg("-")
        .output()
        .map_err(|e| format!("Failed to get HEIC metadata: {}", e))?;
    
    let stderr = String::from_utf8_lossy(&metadata_output.stderr);
    
    // Parse resolution from tile grid
    let mut width = 0u32;
    let mut height = 0u32;
    let mut has_rotation = false;
    let mut rotation_degrees = 0i32;
    
    if let Some(tile_grid_line) = stderr.lines().find(|line| line.contains("Tile Grid:") && line.contains("hevc") && line.contains("default")) {
        debug!("HEIC tile grid: {}", tile_grid_line);
        
        use std::str::FromStr;
        for word in tile_grid_line.split_whitespace() {
            if word.contains('x') && !word.starts_with("0x") {
                if let Some((w, h)) = word.split_once('x') {
                    if let (Ok(w_val), Ok(h_val)) = (u32::from_str(w), u32::from_str(h)) {
                        if w_val >= 100 && w_val < 100000 && h_val >= 100 && h_val < 100000 {
                            width = w_val;
                            height = h_val;
                            info!("HEIC resolution: {}x{}", width, height);
                            break;
                        }
                    }
                }
            }
        }
    }
    
    // Check for rotation
    if stderr.contains("rotation of -90") {
        has_rotation = true;
        rotation_degrees = -90;
        info!("HEIC rotation: -90 degrees");
    } else if stderr.contains("rotation of 90") {
        has_rotation = true;
        rotation_degrees = 90;
        info!("HEIC rotation: 90 degrees");
    } else if stderr.contains("rotation of 180") || stderr.contains("rotation of -180") {
        has_rotation = true;
        rotation_degrees = 180;
        info!("HEIC rotation: 180 degrees");
    }
    
    if width == 0 || height == 0 {
        return Err("Could not determine HEIC tile grid dimensions".to_string());
    }
    
    // Step 2: Create temp directory for tiles
    let temp_dir = std::env::temp_dir().join(format!("heic_tiles_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // Step 3: Extract tiles
    info!("Extracting HEIC tiles to: {}", temp_dir.display());
    let tile_pattern = temp_dir.join("tile_%02d.png");
    
    let extract_output = create_command(tool_path)
        .arg("-i")
        .arg(input_path)
        .arg("-map")
        .arg("0:g:0")
        .arg(&tile_pattern)
        .arg("-y")
        .output()
        .map_err(|e| format!("Failed to extract tiles: {}", e))?;
    
    if !extract_output.status.success() {
        let _ = std::fs::remove_dir_all(&temp_dir);
        return Err("Failed to extract HEIC tiles".to_string());
    }
    
    // Step 4: Calculate grid dimensions (tiles are 512x512)
    let tile_size = 512u32;
    let cols = (width + tile_size - 1) / tile_size;  // Round up
    let rows = (height + tile_size - 1) / tile_size;  // Round up
    info!("Tile grid: {}x{} ({}x{} tiles)", cols, rows, cols * tile_size, rows * tile_size);
    
    // Step 5: Stitch tiles together
    let stitched_path = temp_dir.join("stitched.png");
    let tile_input = temp_dir.join("tile_%02d.png");
    
    let stitch_output = create_command(tool_path)
        .arg("-i")
        .arg(&tile_input)
        .arg("-filter_complex")
        .arg(format!("tile={}x{}", cols, rows))
        .arg("-frames:v")
        .arg("1")
        .arg("-y")
        .arg(&stitched_path)
        .output()
        .map_err(|e| format!("Failed to stitch tiles: {}", e))?;
    
    if !stitch_output.status.success() {
        let _ = std::fs::remove_dir_all(&temp_dir);
        return Err("Failed to stitch HEIC tiles".to_string());
    }
    
    // Step 6: Crop to exact dimensions and apply rotation
    let mut filter_parts = vec![];
    
    // Crop if tiles are larger than actual image
    let stitched_width = cols * tile_size;
    let stitched_height = rows * tile_size;
    if stitched_width != width || stitched_height != height {
        filter_parts.push(format!("crop={}:{}:0:0", width, height));
    }
    
    // Add rotation filter
    if has_rotation {
        match rotation_degrees {
            -90 => filter_parts.push("transpose=1".to_string()),  // 90° CCW
            90 => filter_parts.push("transpose=2".to_string()),    // 90° CW
            180 => filter_parts.push("hflip,vflip".to_string()),   // 180°
            _ => {}
        }
    }
    
    // Step 7: Convert to final format
    let mut final_command = create_command(tool_path);
    final_command
        .arg("-i")
        .arg(&stitched_path);
    
    if !filter_parts.is_empty() {
        final_command
            .arg("-vf")
            .arg(filter_parts.join(","));
    }
    
    final_command
        .arg("-frames:v")
        .arg("1")
        .arg("-y")
        .arg(output_path);
    
    let final_output = final_command.output()
        .map_err(|e| format!("Failed to convert final image: {}", e))?;
    
    // Clean up temp directory
    let _ = std::fs::remove_dir_all(&temp_dir);
    
    if final_output.status.success() {
        Ok(())
    } else {
        Err("Failed to convert HEIC to final format".to_string())
    }
}

/// Check if an image has transparency (alpha channel) using ImageMagick or FFmpeg
fn has_transparency(image_path: &PathBuf) -> bool {
    info!("Checking transparency for: {}", image_path.display());
    
    // Try ImageMagick first (if available)
    if let Ok(tool_path) = get_tool_path("imagemagick") {
        info!("Using ImageMagick to check transparency");
        // ImageMagick 7 syntax: magick identify -format "%[channels]" image.png
        // Returns something like "srgba" (with alpha) or "srgb" (no alpha)
        let output = create_command(&tool_path)
            .arg("identify")
            .arg("-format")
            .arg("%[channels]")
            .arg(image_path)
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                let channels = String::from_utf8_lossy(&output.stdout).to_lowercase();
                info!("Image channels detected: '{}'", channels);
                let has_alpha = channels.contains("a");
                info!("Has transparency: {}", has_alpha);
                return has_alpha;
            }
        }
    }
    
    // Fallback to FFmpeg if ImageMagick isn't available
    if let Ok(ffmpeg_path) = get_tool_path("ffmpeg") {
        info!("Using FFmpeg to check transparency");
        // Use FFmpeg itself to get stream info (works without ffprobe)
        // ffmpeg -i input.png will output stream info to stderr
        let output = create_command(&ffmpeg_path)
            .arg("-i")
            .arg(image_path)
            .output();
        
        if let Ok(output) = output {
            // FFmpeg outputs stream info to stderr
            let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
            info!("FFmpeg output (first 500 chars): {}", &stderr.chars().take(500).collect::<String>());
            
            // Look for pixel format information in the stderr output
            // Example: "Stream #0:0: Video: png, rgba, 1920x1080"
            // Look for rgba, yuva420p, gbrap, etc. (formats with alpha)
            let has_alpha = stderr.contains("rgba") || 
                           stderr.contains("yuva") || 
                           stderr.contains("gbra") ||
                           stderr.contains("ya8") ||
                           stderr.contains("ya16") ||
                           stderr.contains("yuva420p") ||
                           stderr.contains("yuva422p") ||
                           stderr.contains("yuva444p");
            
            info!("Has transparency: {}", has_alpha);
            return has_alpha;
        }
    }
    
    warn!("Could not check transparency - no tools available");
    false
}

async fn execute_conversion(
    tool_name: &str,
    input_path: &PathBuf,
    output_path: &PathBuf,
    advanced_options: Option<String>,
) -> Result<(), String> {
    // Handle special "rename" tool for JPG <-> JPEG conversions
    if tool_name == "rename" {
        info!("Performing file rename/copy from {} to {}", input_path.display(), output_path.display());
        std::fs::copy(input_path, output_path)
            .map_err(|e| format!("Failed to copy file: {}", e))?;
        return Ok(());
    }
    
    // Determine the actual tool to use (with ImageMagick fallback logic)
    let (actual_tool, tool_path) = match get_tool_path(tool_name) {
        Ok(path) => (tool_name, path),
        Err(e) => {
            // If ImageMagick is not available, try to fallback to FFmpeg for image conversions
            if tool_name == "imagemagick" {
                let output_ext = output_path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                
                // HEIC/HEIF requires ImageMagick, no fallback available
                if output_ext == "heic" || output_ext == "heif" {
                    return Err(format!(
                        "ImageMagick is required for HEIC/HEIF encoding but is not installed.\n\n\
                        Please install ImageMagick from the Tools Manager in Settings."
                    ));
                }
                
                // X Window System formats require ImageMagick, no fallback available
                if output_ext == "xbm" || output_ext == "xpm" || output_ext == "xwd" {
                    return Err(format!(
                        "ImageMagick is required for {} format but is not installed.\n\n\
                        {} is an X Window System format not supported by FFmpeg.\n\n\
                        Please install ImageMagick from the Tools Manager in Settings.",
                        output_ext.to_uppercase(), output_ext.to_uppercase()
                    ));
                }
                
                // Try to use FFmpeg as fallback for other image formats
                match get_tool_path("ffmpeg") {
                    Ok(ffmpeg_path) => {
                        info!("ImageMagick not available, using FFmpeg fallback for image conversion");
                        ("ffmpeg", ffmpeg_path)
                    }
                    Err(_) => {
                        return Err(format!(
                            "ImageMagick is not installed and FFmpeg fallback failed.\n\n{}", e
                        ));
                    }
                }
            } else {
                return Err(e);
            }
        }
    };
    
    let mut command = create_command(&tool_path);
    
    // On macOS, set environment variables for ImageMagick to find its bundled libraries
    #[cfg(target_os = "macos")]
    if actual_tool == "imagemagick" {
        // tool_path is: ~/Library/Application Support/com.convertsave/imagemagick/bin/magick
        if let Some(bin_dir) = tool_path.parent() {
            if let Some(imagemagick_dir) = bin_dir.parent() {
                let lib_dir = imagemagick_dir.join("lib");
                let etc_dir = imagemagick_dir.join("etc").join("ImageMagick-7");
                
                info!("Setting DYLD_LIBRARY_PATH: {}", lib_dir.display());
                info!("Setting MAGICK_HOME: {}", imagemagick_dir.display());
                
                command.env("DYLD_LIBRARY_PATH", &lib_dir);
                command.env("MAGICK_HOME", &imagemagick_dir);
                
                // Set configuration path if it exists
                if etc_dir.exists() {
                    info!("Setting MAGICK_CONFIGURE_PATH: {}", etc_dir.display());
                    command.env("MAGICK_CONFIGURE_PATH", &etc_dir);
                } else {
                    warn!("Configuration directory not found: {}", etc_dir.display());
                    // List what's actually in the imagemagick directory
                    if let Ok(entries) = std::fs::read_dir(&imagemagick_dir) {
                        warn!("Contents of imagemagick directory:");
                        for entry in entries.flatten() {
                            warn!("  - {}", entry.path().display());
                        }
                    }
                }
                
                // Set module path for builds with --with-modules enabled
                // Look for ImageMagick-* directory in lib
                if let Ok(entries) = std::fs::read_dir(&lib_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() && path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.starts_with("ImageMagick-"))
                            .unwrap_or(false) {
                            let modules_coders = path.join("modules-Q16HDRI").join("coders");
                            if modules_coders.exists() {
                                info!("Setting MAGICK_CODER_MODULE_PATH: {}", modules_coders.display());
                                command.env("MAGICK_CODER_MODULE_PATH", &modules_coders);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    match actual_tool {
        "imagemagick" => {
            // ImageMagick 7 syntax: magick input.jpg [options] output.heic
            // Note: ImageMagick 7 doesn't use "convert" as a subcommand
            
            // Check input and output formats
            let input_ext = input_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            let output_ext = output_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            // Animation formats that can have multiple frames
            let animation_formats = ["gif", "webp", "apng", "mng"];
            
            // If converting from an animation format to a static format, extract first frame only
            // This prevents animated GIFs from creating artifacts when converted to static images
            if animation_formats.contains(&input_ext.as_str()) && !animation_formats.contains(&output_ext.as_str()) {
                // Use [0] syntax to select only the first frame
                let input_with_frame = format!("{}[0]", input_path.display());
                info!("Extracting first frame from animated {}: {}", input_ext.to_uppercase(), input_with_frame);
                command.arg(&input_with_frame);
            } else {
                command.arg(input_path);
            }
            
            // Check if we need to handle transparency -> opaque conversion
            // Formats that don't support alpha transparency (or only binary transparency like GIF)
            let formats_without_transparency = [
                "jpg", "jpeg", "bmp", "gif", "j2k", "jp2", "jpc", "jpf", "jpx", "jpm",
                "hdr", "pbm", "pgm", "ppm"
            ];
            
            // If input has transparency and output format doesn't support it, flatten with white background
            if formats_without_transparency.contains(&output_ext.as_str()) && has_transparency(input_path) {
                info!("Detected transparency in input image, flattening with white background for {} output", output_ext);
                command.arg("-background").arg("white");
                command.arg("-flatten");
            }
            
            // Format-specific quality and options
            match output_ext.as_str() {
                // ICO format requires special handling - must be resized to fit icon size limits
                "ico" => {
                    // ICO files have size limitations (typically max 256x256)
                    // Resize to 256x256 maintaining aspect ratio, then use extent to make it square
                    command.arg("-resize").arg("256x256");
                    command.arg("-gravity").arg("center");
                    command.arg("-extent").arg("256x256");
                    command.arg("-background").arg("transparent");
                }
                // Modern compressed formats
                "heic" | "heif" => {
                    command.arg("-quality").arg("85");
                }
                "avif" => {
                    command.arg("-quality").arg("85");
                }
                "jxl" => {
                    command.arg("-quality").arg("90"); // JPEG XL benefits from higher quality
                }
                "webp" => {
                    command.arg("-quality").arg("90");
                }
                // Standard lossy formats
                "jpg" | "jpeg" => {
                    command.arg("-quality").arg("90");
                }
                // JPEG 2000 variants
                "j2k" | "jp2" | "jpc" | "jpf" | "jpx" | "jpm" => {
                    command.arg("-quality").arg("85");
                }
                // Professional formats (high quality)
                "tiff" | "tif" | "exr" | "hdr" | "dpx" => {
                    command.arg("-quality").arg("100");
                }
                // PDF output from images - use JPEG compression for reasonable file size
                "pdf" => {
                    command.arg("-compress").arg("jpeg"); // Compress images appropriately, preserves transparency
                    command.arg("-density").arg("300");   // Display at correct zoom level
                }
                // Vector formats
                "svg" | "svgz" => {
                    command.arg("-density").arg("300"); // 300 DPI for vector
                }
                // Everything else uses ImageMagick defaults
                _ => {}
            }
            
            // Add advanced options if provided (will override defaults)
            if let Some(options) = advanced_options {
                let options_parts: Vec<&str> = options.split_whitespace().collect();
                for part in options_parts {
                    command.arg(part);
                }
            }
            
            command.arg(output_path);
        }
        "ffmpeg" => {
            // Check input format for special HEIC handling
            let input_ext = input_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            // Check output format
            let output_ext = output_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            // HEIC/HEIF files need special tile reassembly handling
            if input_ext == "heic" || input_ext == "heif" {
                return convert_heic_with_tiles(&tool_path, input_path, output_path);
            }
            
            command.arg("-i").arg(input_path);
            
            // Animation formats that can have multiple frames
            let animation_formats = ["gif", "webp", "apng", "mng"];
            
            // If converting from an animation format to a static format, extract first frame only
            if animation_formats.contains(&input_ext.as_str()) && !animation_formats.contains(&output_ext.as_str()) {
                info!("Extracting first frame from animated {}", input_ext.to_uppercase());
                command.arg("-frames:v").arg("1");
            }
            
            // Check if we need to handle transparency -> opaque conversion
            // Formats that don't support alpha transparency (or only binary transparency like GIF)
            let formats_without_transparency = [
                "jpg", "jpeg", "bmp", "gif", "j2k", "jp2", "jpc", "jpf", "jpx", "jpm",
                "hdr", "pbm", "pgm", "ppm"
            ];
            
            // If input has transparency and output format doesn't support it, flatten with white background
            // For FFmpeg, we use a filter to composite the image over a white background
            let needs_transparency_handling = formats_without_transparency.contains(&output_ext.as_str()) && has_transparency(input_path);
            
            // Handle transparency flattening if needed  
            if needs_transparency_handling {
                info!("🎨 TRANSPARENCY DETECTED! Adding white background for {} output using FFmpeg", output_ext);
                // Exact command: -f lavfi -i color=c=white -filter_complex "[1][0]scale=rw:rh[bg];[bg][0]overlay=shortest=1" -q:v 1
                command.arg("-f").arg("lavfi");
                command.arg("-i").arg("color=c=white");
                command.arg("-filter_complex");
                
                // Build filter string based on format requirements
                let mut filter = String::from("[1][0]scale=rw:rh[bg];[bg][0]overlay=shortest=1");
                
                // Some formats need explicit pixel format conversion for proper color handling
                let problematic_formats = ["hdr", "pbm", "pgm", "ppm"];
                if problematic_formats.contains(&output_ext.as_str()) {
                    filter.push_str(",format=rgb24");
                }
                
                command.arg(&filter);
                command.arg("-q:v").arg("1");
            }
            
            // AVIF codec options must come AFTER inputs and filters
            if output_ext == "avif" {
                // Clear previous arguments to ensure correct order for AVIF
                // We need to restructure the command completely for AVIF
                // Logic below handles the reconstruction
            } else {
                // For non-AVIF formats, add the input argument here if not already added
                // (Note: we already added -i input_path above, so this else block is just a placeholder concept)
            }

            // AVIF Special Handling based on platform and transparency
            if output_ext == "avif" {
                // Reset command arguments for AVIF to ensure correct order
                // We need to rebuild the command from scratch because AVIF requires specific parameter ordering
                command = create_command(&tool_path);
                
                // Add input file
                command.arg("-i").arg(input_path);

                // Check for transparency
                let has_alpha = has_transparency(input_path);
                
                if has_alpha {
                    // Transparent AVIF settings
                    command.arg("-map").arg("0:v").arg("-map").arg("0:v");
                    command.arg("-filter:v:1").arg("alphaextract");
                    command.arg("-frames:v").arg("1");
                    command.arg("-c:v").arg("libaom-av1");
                    command.arg("-still-picture").arg("1");
                    command.arg("-cpu-used").arg("6");
                    command.arg("-crf").arg("28");
                    command.arg("-b:v").arg("0");
                    command.arg("-row-mt").arg("1");
                } else {
                    // Non-transparent AVIF settings
                    command.arg("-frames:v").arg("1");
                    command.arg("-c:v").arg("libaom-av1");
                    command.arg("-still-picture").arg("1");
                    command.arg("-cpu-used").arg("6");
                    command.arg("-crf").arg("28");
                    command.arg("-b:v").arg("0");
                    command.arg("-row-mt").arg("1");
                }

                // Add advanced options if provided
                if let Some(options) = advanced_options {
                    let options_parts: Vec<&str> = options.split_whitespace().collect();
                    for part in options_parts {
                        command.arg(part);
                    }
                }

                command.arg("-y").arg(output_path);
            } else {
                // Standard handling for other formats (continuation of previous logic)
                
                // ICO format requires resizing to max 256x256
                if output_ext == "ico" {
                    command.arg("-vf");
                    command.arg("scale='min(256,iw)':'min(256,ih)':force_original_aspect_ratio=decrease");
                }
                
                // MP4 format: Use compatible settings for broad playback support
                if output_ext == "mp4" {
                    command.arg("-pix_fmt").arg("yuv420p");
                    command.arg("-profile:v").arg("main");
                    command.arg("-movflags").arg("+faststart");
                }
                
                // Add advanced options if provided
                if let Some(options) = advanced_options {
                    let options_parts: Vec<&str> = options.split_whitespace().collect();
                    for part in options_parts {
                        command.arg(part);
                    }
                }
                
                // For video/multi-frame input to single image output, specify one frame
                // NOTE: gif is excluded because video->gif should create animated GIF
                let video_formats = ["mp4", "mov", "avi", "mkv", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "3gp", "ogv"];
                let static_image_formats = ["jpg", "jpeg", "png", "webp", "bmp", "tiff", "tif", "ico"];
                
                if video_formats.contains(&input_ext.as_str()) && static_image_formats.contains(&output_ext.as_str()) {
                    command.arg("-frames:v").arg("1");
                }
                
                // For single image output (not animated GIF), use -update flag to write one file (not a sequence)
                if static_image_formats.contains(&output_ext.as_str()) {
                    command.arg("-update").arg("1");
                }
                
                command.arg("-y").arg(output_path); // -y to overwrite output file
            }
        }
        "pandoc" if ENABLE_PANDOC => {
            command.arg(input_path).arg("-o").arg(output_path);
            
            // Add advanced options if provided
            if let Some(options) = advanced_options {
                let options_parts: Vec<&str> = options.split_whitespace().collect();
                for part in options_parts {
                    command.arg(part);
                }
            }
        }
        _ => return Err(format!("Unknown tool: {}", tool_name)),
    }
    
    // Log the actual command being executed
    debug!("Executing command: {:?}", command);
    
    let output = command.output()
        .map_err(|e| format!("Failed to execute {}: {}", tool_name, e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Log full output for debugging
        error!("=== COMMAND FAILED ===");
        error!("Exit status: {:?}", output.status);
        error!("STDOUT:\n{}", stdout);
        error!("STDERR:\n{}", stderr);
        error!("======================");
        
        // Check for empty output (binary failed to start)
        if stderr.is_empty() && stdout.is_empty() {
            error!("Binary produced no output - likely failed to start");
            
            // Additional diagnostics
            #[cfg(target_os = "macos")]
            if tool_name == "imagemagick" {
                use std::process::Command as StdCommand;
                
                // Check binary architecture
                if let Ok(file_output) = StdCommand::new("file").arg(&tool_path).output() {
                    let file_info = String::from_utf8_lossy(&file_output.stdout);
                    error!("Binary file info: {}", file_info);
                    
                    // Check machine architecture
                    if let Ok(uname_output) = StdCommand::new("uname").arg("-m").output() {
                        let machine_arch = String::from_utf8_lossy(&uname_output.stdout);
                        error!("Machine architecture: {}", machine_arch.trim());
                    }
                }
                
                // Check if binary is executable
                if let Ok(metadata) = std::fs::metadata(&tool_path) {
                    error!("Binary permissions: {:?}", metadata.permissions());
                } else {
                    error!("Cannot read binary metadata");
                }
                
                // Check dylib dependencies
                if let Ok(otool_output) = StdCommand::new("otool").arg("-L").arg(&tool_path).output() {
                    let dylib_info = String::from_utf8_lossy(&otool_output.stdout);
                    error!("Binary dependencies:\n{}", dylib_info);
                    
                    // Check if all dylibs exist and are correct architecture
                    for line in dylib_info.lines().skip(1) {
                        if let Some(dylib_path) = line.trim().split_whitespace().next() {
                            if dylib_path.starts_with('@') {
                                // @rpath or @loader_path - need to resolve
                                continue;
                            }
                            
                            if std::path::Path::new(dylib_path).exists() {
                                if let Ok(file_output) = StdCommand::new("file").arg(dylib_path).output() {
                                    let file_info = String::from_utf8_lossy(&file_output.stdout);
                                    if !file_info.contains("arm64") {
                                        error!("WARNING: Dependency {} is not arm64: {}", dylib_path, file_info.trim());
                                    }
                                }
                            } else {
                                error!("WARNING: Missing dependency: {}", dylib_path);
                            }
                        }
                    }
                }
                
                // Check for code signing/quarantine issues
                error!("Checking quarantine attribute...");
                if let Ok(xattr_output) = StdCommand::new("xattr").arg("-l").arg(&tool_path).output() {
                    let xattr_info = String::from_utf8_lossy(&xattr_output.stdout);
                    error!("Quarantine attributes: {}", if xattr_info.is_empty() { "none" } else { xattr_info.trim() });
                    
                    if xattr_info.contains("com.apple.quarantine") {
                        error!("Binary is quarantined by macOS! Attempting to remove quarantine...");
                        if let Ok(_) = StdCommand::new("xattr").arg("-d").arg("com.apple.quarantine").arg(&tool_path).output() {
                            error!("Quarantine removed. Please try the conversion again.");
                        }
                    }
                }
            }
        }
        
        // Provide user-friendly error messages for common issues
        let error_msg = if stderr.contains("does not contain any stream") {
            if tool_name == "ffmpeg" {
                "This video file has no audio stream. Cannot convert to audio format. Try converting to a video format instead.".to_string()
            } else {
                "The file does not contain the required streams for this conversion.".to_string()
            }
        } else if stderr.contains("Unable to choose an output format") || (stderr.contains("use a standard extension") && (stderr.contains("heic") || stderr.contains("heif") || stderr.contains("avif"))) {
            // Detect which format is being converted to
            let output_ext = output_path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_uppercase();
            
            // Check if this is an X Window format
            let x_window_formats = ["XBM", "XPM", "XWD"];
            if x_window_formats.contains(&output_ext.as_str()) {
                format!("{} format is not supported by FFmpeg.\n\nThis format requires ImageMagick. Please install ImageMagick from the Tools Manager in Settings.", output_ext)
            } else {
                format!("{} format encoding is not supported by this {} build.\n\nThis format may be available with ImageMagick. Try:\n• Installing ImageMagick from Tools Manager\n• Converting to JPG, PNG, or WebP", output_ext, tool_name)
            }
        } else if stderr.contains("Unknown encoder") || stderr.contains("Encoder not found") || stderr.contains("libx265") || stderr.contains("libaom-av1") {
            format!("The required codec is not available in this {} build.\n\nTry converting to a different format like JPG, PNG, or WebP.", tool_name)
        } else if stderr.contains("Invalid argument") && stderr.contains("Error opening output file") {
            "Cannot write to the output location. This may be due to:\n- Network drive access issues\n- Insufficient permissions\n- Invalid file path\n\nTry saving to a local drive instead.".to_string()
        } else if stderr.contains("No such file or directory") || stderr.contains("does not exist") {
            "Input file not found. The file may have been moved or deleted.".to_string()
        } else if stderr.is_empty() && stdout.is_empty() {
            // Binary failed to start
            #[cfg(target_os = "macos")]
            {
                let status_code = output.status.code().unwrap_or(-1);
                if status_code == 9 {
                    format!("ImageMagick binary was killed by macOS (SIGKILL).\n\nThis is usually caused by:\n• Missing or incompatible dylib dependencies\n• macOS Gatekeeper/quarantine (check logs above)\n• Code signing issues\n\nCheck the detailed logs above for:\n- Missing dependencies\n- Wrong architecture dependencies\n- Quarantine status\n\nExit status: {:?}", output.status)
                } else {
                    format!("ImageMagick binary failed to start.\n\nThis is usually caused by:\n• Architecture mismatch (wrong Intel/ARM build)\n• Missing dependencies\n• Corrupted download\n\nCheck the detailed logs above.\n\nExit status: {:?}", output.status)
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                format!("Binary failed to start. Exit status: {:?}", output.status)
            }
        } else {
            // For other errors, show the technical details
            format!("Conversion failed. Error details: {}", stderr)
        };
        
        Err(error_msg)
    }
}

// Binary download functions

/// Create a configured HTTP client for downloads
fn create_http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300)) // 5 minute timeout
        .user_agent("ConvertSave/1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))
}

/// Check if Homebrew is available on the system
#[cfg(target_os = "macos")]
fn is_homebrew_available() -> bool {
    create_command("brew")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get macOS version as (major, minor)
#[cfg(target_os = "macos")]
fn get_macos_version() -> (u32, u32) {
    use std::process::Command;
    
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output();
    
    if let Ok(output) = output {
        if let Ok(version_str) = String::from_utf8(output.stdout) {
            let parts: Vec<&str> = version_str.trim().split('.').collect();
            if parts.len() >= 2 {
                let major = parts[0].parse::<u32>().unwrap_or(13); // Default to 13 if parse fails
                let minor = parts[1].parse::<u32>().unwrap_or(0);
                return (major, minor);
            }
        }
    }
    
    // Default to macOS 13 if detection fails (use newer build)
    (13, 0)
}

#[cfg(not(target_os = "macos"))]
fn get_macos_version() -> (u32, u32) {
    // Not macOS, return dummy version
    (13, 0)
}

/// Get macOS machine architecture at runtime (arm64 or x86_64)
/// This detects the actual hardware, even if running an x86_64 app under Rosetta 2
#[cfg(target_os = "macos")]
fn get_macos_architecture() -> String {
    use std::process::Command;
    
    // Use uname -m to get the actual machine architecture
    let output = Command::new("uname")
        .arg("-m")
        .output();
    
    if let Ok(output) = output {
        if let Ok(arch_str) = String::from_utf8(output.stdout) {
            let arch = arch_str.trim();
            // uname -m returns "arm64" on Apple Silicon, "x86_64" on Intel
            if arch == "arm64" || arch == "aarch64" {
                return "arm64".to_string();
            } else if arch == "x86_64" {
                return "x86_64".to_string();
            }
        }
    }
    
    // Fallback to compile-time detection
    if cfg!(target_arch = "aarch64") {
        "arm64".to_string()
    } else {
        "x86_64".to_string()
    }
}

#[cfg(not(target_os = "macos"))]
fn get_macos_architecture() -> String {
    "x86_64".to_string()
}

/// Install a package via Homebrew on macOS
#[cfg(target_os = "macos")]
async fn install_via_homebrew(app: AppHandle, package: &str) -> Result<String, String> {
    // Homebrew should be checked before calling this function
    if !is_homebrew_available() {
        return Err("Homebrew is not available".to_string());
    }
    
    app.emit("download-progress", DownloadProgress {
        status: "checking".to_string(),
        message: format!("Checking if {} is already installed...", package),
    }).map_err(|e| e.to_string())?;
    
    // Check if package is already installed
    let list_output = create_command("brew")
        .arg("list")
        .arg(package)
        .output()
        .map_err(|e| e.to_string())?;
    
    if list_output.status.success() {
        // Package is installed, try to upgrade it
        app.emit("download-progress", DownloadProgress {
            status: "upgrading".to_string(),
            message: format!("Upgrading {}...", package),
        }).map_err(|e| e.to_string())?;
        
        let upgrade_output = create_command("brew")
            .arg("upgrade")
            .arg(package)
            .output()
            .map_err(|e| e.to_string())?;
        
        if upgrade_output.status.success() {
            app.emit("download-progress", DownloadProgress {
                status: "complete".to_string(),
                message: format!("{} upgraded successfully!", package),
            }).map_err(|e| e.to_string())?;
            
            return Ok(format!("{} upgraded successfully via Homebrew", package));
        } else {
            // Upgrade failed, but package is still installed
            app.emit("download-progress", DownloadProgress {
                status: "complete".to_string(),
                message: format!("{} is already up to date", package),
            }).map_err(|e| e.to_string())?;
            
            return Ok(format!("{} is already installed and up to date", package));
        }
    }
    
    // Package not installed, install it
    app.emit("download-progress", DownloadProgress {
        status: "installing".to_string(),
        message: format!("Installing {} via Homebrew...", package),
    }).map_err(|e| e.to_string())?;
    
    let install_output = create_command("brew")
        .arg("install")
        .arg(package)
        .output()
        .map_err(|e| e.to_string())?;
    
    if !install_output.status.success() {
        let stderr = String::from_utf8_lossy(&install_output.stderr);
        return Err(format!("Failed to install {} via Homebrew: {}", package, stderr));
    }
    
    app.emit("download-progress", DownloadProgress {
        status: "complete".to_string(),
        message: format!("{} installed successfully!", package),
    }).map_err(|e| e.to_string())?;
    
    Ok(format!("{} installed successfully via Homebrew", package))
}

#[tauri::command]
async fn download_ffmpeg(app: AppHandle) -> Result<String, String> {
    // On macOS, prefer Homebrew but fall back to manual download
    #[cfg(target_os = "macos")]
    {
        if is_homebrew_available() {
            app.emit("download-progress", DownloadProgress {
                status: "checking".to_string(),
                message: "Using Homebrew for installation...".to_string(),
            }).ok();
            return install_via_homebrew(app, "ffmpeg").await;
        }
        // Fall through to manual download if Homebrew not available
    }
    
    // Manual download for all platforms
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        
        let (download_url, filename, is_zip) = get_ffmpeg_download_info().await?;
        let ffmpeg_dir = data_dir.join("ffmpeg");
        let ffmpeg_path = ffmpeg_dir.join(if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" });
        
        // If FFmpeg already exists, remove it to allow updating
        if ffmpeg_dir.exists() {
            println!("Removing existing FFmpeg installation for update...");
            std::fs::remove_dir_all(&ffmpeg_dir).map_err(|e| format!("Failed to remove old FFmpeg: {}", e))?;
        }
        
        app.emit("download-progress", DownloadProgress {
            status: "downloading".to_string(),
            message: "Downloading FFmpeg...".to_string(),
        }).map_err(|e| e.to_string())?;
        
        let client = create_http_client()?;
        let response = client.get(&download_url).send().await.map_err(|e| {
            format!("Failed to download FFmpeg: {}. Try again or check your internet connection.", e)
        })?;
        
        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}. The file may not be available.", response.status()));
        }
        
        let bytes = response.bytes().await.map_err(|e| e.to_string())?;
        
        let archive_path = data_dir.join(&filename);
        std::fs::write(&archive_path, bytes).map_err(|e| e.to_string())?;
        
        app.emit("download-progress", DownloadProgress {
            status: "extracting".to_string(),
            message: "Extracting FFmpeg...".to_string(),
        }).map_err(|e| e.to_string())?;
        
        let extract_dir = data_dir.join("ffmpeg");
        std::fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;
        
        if is_zip {
            extract_zip(&archive_path, &extract_dir, "ffmpeg")?;
        } else {
            extract_tar_gz(&archive_path, &extract_dir, "ffmpeg")?;
        }
        
        // Verify the file was actually extracted
        if !ffmpeg_path.exists() {
            return Err(format!("FFmpeg binary not found after extraction at: {}", ffmpeg_path.display()));
        }
        
        std::fs::remove_file(&archive_path).map_err(|e| e.to_string())?;
        
    app.emit("download-progress", DownloadProgress {
        status: "complete".to_string(),
        message: "FFmpeg downloaded successfully!".to_string(),
    }).map_err(|e| e.to_string())?;
    
    Ok("FFmpeg downloaded successfully".to_string())
}

#[tauri::command]
async fn download_pandoc(app: AppHandle) -> Result<String, String> {
    // On macOS, prefer Homebrew but fall back to manual download
    #[cfg(target_os = "macos")]
    {
        if is_homebrew_available() {
            app.emit("download-progress", DownloadProgress {
                status: "checking".to_string(),
                message: "Using Homebrew for installation...".to_string(),
            }).ok();
            return install_via_homebrew(app, "pandoc").await;
        }
        // Fall through to manual download if Homebrew not available
    }
    
    // Manual download for all platforms
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        
        let (download_url, filename, is_zip) = get_pandoc_download_info().await?;
        let pandoc_dir = data_dir.join("pandoc");
        let pandoc_path = pandoc_dir.join(if cfg!(windows) { "pandoc.exe" } else { "pandoc" });
        
        // If Pandoc already exists, remove it to allow updating
        if pandoc_dir.exists() {
            println!("Removing existing Pandoc installation for update...");
            std::fs::remove_dir_all(&pandoc_dir).map_err(|e| format!("Failed to remove old Pandoc: {}", e))?;
        }
        
        app.emit("download-progress", DownloadProgress {
            status: "downloading".to_string(),
            message: "Downloading Pandoc...".to_string(),
        }).map_err(|e| e.to_string())?;
        
        let client = create_http_client()?;
        let response = client.get(&download_url).send().await.map_err(|e| {
            format!("Failed to download Pandoc: {}. Try again or check your internet connection.", e)
        })?;
        
        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}. The file may not be available.", response.status()));
        }
        
        let bytes = response.bytes().await.map_err(|e| e.to_string())?;
        
        let archive_path = data_dir.join(&filename);
        std::fs::write(&archive_path, bytes).map_err(|e| e.to_string())?;
        
        app.emit("download-progress", DownloadProgress {
            status: "extracting".to_string(),
            message: "Extracting Pandoc...".to_string(),
        }).map_err(|e| e.to_string())?;
        
        let extract_dir = data_dir.join("pandoc");
        std::fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;
        
        if is_zip {
            extract_zip(&archive_path, &extract_dir, "pandoc")?;
        } else {
            extract_tar_gz(&archive_path, &extract_dir, "pandoc")?;
        }
        
        // Verify the file was actually extracted
        if !pandoc_path.exists() {
            return Err(format!("Pandoc binary not found after extraction at: {}", pandoc_path.display()));
        }
        
        std::fs::remove_file(&archive_path).map_err(|e| e.to_string())?;
        
    app.emit("download-progress", DownloadProgress {
        status: "complete".to_string(),
        message: "Pandoc downloaded successfully!".to_string(),
    }).map_err(|e| e.to_string())?;
    
    Ok("Pandoc downloaded successfully".to_string())
}

#[tauri::command]
async fn download_imagemagick(app: AppHandle) -> Result<String, String> {
    // On macOS, prefer Homebrew but fall back to manual download
    #[cfg(target_os = "macos")]
    {
        if is_homebrew_available() {
            app.emit("download-progress", DownloadProgress {
                status: "checking".to_string(),
                message: "Using Homebrew for installation...".to_string(),
            }).ok();
            return install_via_homebrew(app, "imagemagick").await;
        }
        // Fall through to manual download if Homebrew not available
    }
    
    // Manual download for all platforms
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    
    let (download_url, filename, is_sevenz) = get_imagemagick_download_info().await?;
    println!("=== IMAGEMAGICK DOWNLOAD ===");
    println!("Download URL: {}", download_url);
    println!("Data dir: {}", data_dir.display());
    println!("Filename: {}", filename);
    println!("Is 7z: {}", is_sevenz);
    
    let magick_exe = if cfg!(windows) { "magick.exe" } else { "magick" };
    let imagemagick_dir = data_dir.join("imagemagick");
    
    // On macOS, magick binary will be in bin/ subdirectory
    #[cfg(target_os = "macos")]
    let magick_path = imagemagick_dir.join("bin").join(magick_exe);
    
    // On other platforms, it's in the root
    #[cfg(not(target_os = "macos"))]
    let magick_path = imagemagick_dir.join(magick_exe);
    
    // If ImageMagick already exists, remove it to allow updating
    if imagemagick_dir.exists() {
        println!("Removing existing ImageMagick installation for update...");
        std::fs::remove_dir_all(&imagemagick_dir).map_err(|e| format!("Failed to remove old ImageMagick: {}", e))?;
    }
    
    app.emit("download-progress", DownloadProgress {
        status: "downloading".to_string(),
        message: "Downloading ImageMagick...".to_string(),
    }).map_err(|e| e.to_string())?;
    
    println!("Starting download from: {}", download_url);
    
    // Create a properly configured HTTP client
    let client = create_http_client()?;
    
    let response = client.get(&download_url)
        .send()
        .await
        .map_err(|e| {
            println!("Download request failed: {}", e);
            // Provide more helpful error message
            if e.is_timeout() {
                format!("Download timed out. Please check your internet connection and try again.")
            } else if e.is_connect() {
                format!("Could not connect to imagemagick.org. Please check your internet connection.")
            } else {
                format!("Failed to download ImageMagick: {}. Try again or check your internet connection.", e)
            }
        })?;
    
    println!("Download response status: {:?}", response.status());
    
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}. The file may not be available.", response.status()));
    }
    
    let bytes = response.bytes().await.map_err(|e| {
        println!("Failed to read bytes: {}", e);
        format!("Failed to read download data: {}", e)
    })?;
    
    println!("Downloaded {} bytes", bytes.len());
    
    let extract_dir = data_dir.join("imagemagick");
    std::fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;
    
    // Linux downloads a raw binary, no extraction needed
    if cfg!(target_os = "linux") {
        app.emit("download-progress", DownloadProgress {
            status: "installing".to_string(),
            message: "Installing ImageMagick...".to_string(),
        }).map_err(|e| e.to_string())?;
        
        println!("Writing binary directly to: {}", magick_path.display());
        std::fs::write(&magick_path, bytes).map_err(|e| e.to_string())?;
        
        // Make executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&magick_path, std::fs::Permissions::from_mode(0o755))
                .map_err(|e| e.to_string())?;
        }
    } else {
        // Windows and macOS need extraction
        let archive_path = data_dir.join(&filename);
        println!("Writing archive to: {}", archive_path.display());
        
        std::fs::write(&archive_path, bytes).map_err(|e| {
            println!("Failed to write archive: {}", e);
            e.to_string()
        })?;
        
        println!("Archive written successfully, size: {} bytes", std::fs::metadata(&archive_path).map(|m| m.len()).unwrap_or(0));
        
        app.emit("download-progress", DownloadProgress {
            status: "extracting".to_string(),
            message: "Extracting ImageMagick...".to_string(),
        }).map_err(|e| e.to_string())?;
        
        println!("Starting extraction to: {}", extract_dir.display());
        println!("Looking for binary: {}", magick_exe);
        
        // Windows uses .7z, macOS uses .tar.gz
        if is_sevenz {
            // ImageMagick portable .7z archive (Windows)
            println!("Extracting ImageMagick .7z archive...");
            sevenz_rust::decompress_file(&archive_path, &extract_dir)
                .map_err(|e| {
                    println!("7z extraction failed: {}", e);
                    std::fs::remove_file(&archive_path).ok();
                    format!("Failed to extract ImageMagick .7z: {}", e)
                })?;
            println!("ImageMagick extraction successful!");
            
            // Check if files are in a subdirectory and move them up if needed
            if !magick_path.exists() {
                println!("magick.exe not found at root, searching subdirectories...");
                
                // Find magick.exe in subdirectories
                fn find_magick_exe(dir: &std::path::Path) -> Option<std::path::PathBuf> {
                    if let Ok(entries) = std::fs::read_dir(dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() && path.file_name().and_then(|n| n.to_str()) == Some("magick.exe") {
                                return Some(path);
                            } else if path.is_dir() {
                                if let Some(found) = find_magick_exe(&path) {
                                    return Some(found);
                                }
                            }
                        }
                    }
                    None
                }
                
                if let Some(found_magick) = find_magick_exe(&extract_dir) {
                    println!("Found magick.exe at: {}", found_magick.display());
                    
                    // Get the directory containing magick.exe
                    if let Some(source_dir) = found_magick.parent() {
                        println!("Moving files from {} to {}", source_dir.display(), extract_dir.display());
                        
                        // Move all files from source_dir to extract_dir
                        if let Ok(entries) = std::fs::read_dir(source_dir) {
                            for entry in entries.flatten() {
                                let source_path = entry.path();
                                let file_name = source_path.file_name().unwrap();
                                let dest_path = extract_dir.join(file_name);
                                
                                if let Err(e) = std::fs::rename(&source_path, &dest_path) {
                                    println!("Failed to move {}: {}", source_path.display(), e);
                                } else {
                                    println!("Moved: {} -> {}", source_path.display(), dest_path.display());
                                }
                            }
                        }
                        
                        // Clean up the now-empty nested directory
                        let _ = std::fs::remove_dir_all(source_dir);
                    }
                } else {
                    println!("ERROR: Could not find magick.exe anywhere in extracted files");
                    println!("Extracted directory contents:");
                    if let Ok(entries) = std::fs::read_dir(&extract_dir) {
                        for entry in entries.flatten() {
                            println!("  - {}", entry.path().display());
                        }
                    }
                }
            }
        } else {
            // macOS tar.gz extraction - Keep ImageMagick structure as-is (bin/ and lib/ directories)
            // This matches what the official ImageMagick documentation says to do
            println!("Extracting tarball to: {}", extract_dir.display());
            extract_tar_gz_all(&archive_path, &extract_dir)?;
            
            // DEBUG: List everything that was extracted
            println!("=== EXTRACTED FILES ===");
            fn list_all_files(dir: &std::path::Path, prefix: &str) {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            println!("{}[DIR] {}", prefix, path.file_name().unwrap().to_string_lossy());
                            list_all_files(&path, &format!("{}  ", prefix));
                        } else {
                            println!("{}{}", prefix, path.file_name().unwrap().to_string_lossy());
                        }
                    }
                }
            }
            list_all_files(&extract_dir, "");
            println!("=== END EXTRACTED FILES ===");
            
            // The tarball extracts to a subdirectory like ImageMagick-7.1.2/
            // We need to move everything up one level to extract_dir
            // Look for the ImageMagick directory
            let mut imagemagick_root: Option<PathBuf> = None;
            if let Ok(entries) = std::fs::read_dir(&extract_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() && path.file_name().unwrap().to_string_lossy().starts_with("ImageMagick") {
                        imagemagick_root = Some(path);
                        break;
                    }
                }
            }
            
            if let Some(im_root) = imagemagick_root {
                println!("Found ImageMagick root directory: {}", im_root.display());
                
                // Move all subdirectories (bin/, lib/, etc.) to extract_dir
                if let Ok(entries) = std::fs::read_dir(&im_root) {
                    for entry in entries.flatten() {
                        let source_path = entry.path();
                        let name = source_path.file_name().unwrap();
                        let dest_path = extract_dir.join(name);
                        
                        println!("Moving {} to {}", source_path.display(), dest_path.display());
                        if let Err(e) = std::fs::rename(&source_path, &dest_path) {
                            println!("Failed to move {}: {}", source_path.display(), e);
                        }
                    }
                }
                
                // Clean up the now-empty ImageMagick directory
                let _ = std::fs::remove_dir_all(&im_root);
                
                println!("Final structure:");
                list_all_files(&extract_dir, "");
            }
        }
        
        println!("Removing archive file: {}", archive_path.display());
        std::fs::remove_file(&archive_path).map_err(|e| e.to_string())?;
    }
    
    // Verify the file was actually extracted
    if !magick_path.exists() {
        println!("ERROR: ImageMagick binary still not found at: {}", magick_path.display());
        println!("Final directory contents:");
        if let Ok(entries) = std::fs::read_dir(extract_dir) {
            for entry in entries.flatten() {
                println!("  - {}", entry.path().display());
            }
        }
        return Err(format!("ImageMagick binary not found after extraction at: {}", magick_path.display()));
    }
    
    app.emit("download-progress", DownloadProgress {
        status: "complete".to_string(),
        message: "ImageMagick downloaded successfully!".to_string(),
    }).map_err(|e| e.to_string())?;
    
    Ok("ImageMagick downloaded successfully".to_string())
}

#[tauri::command]
async fn test_tool(tool_name: String) -> Result<String, String> {
    let tool_path = match get_tool_path(&tool_name) {
        Ok(path) => path,
        Err(_) => {
            return Err(format!("{} not found. Please download it first.", tool_name));
        }
    };
    
    // ImageMagick uses -version, FFmpeg and Pandoc use -version too
    let mut command = create_command(&tool_path);
    command.arg("-version");
    
    // On macOS, set environment variables for ImageMagick
    #[cfg(target_os = "macos")]
    if tool_name == "imagemagick" {
        if let Some(bin_dir) = tool_path.parent() {
            if let Some(imagemagick_dir) = bin_dir.parent() {
                let lib_dir = imagemagick_dir.join("lib");
                let etc_dir = imagemagick_dir.join("etc").join("ImageMagick-7");
                
                info!("Setting DYLD_LIBRARY_PATH for test: {}", lib_dir.display());
                info!("Setting MAGICK_HOME for test: {}", imagemagick_dir.display());
                
                command.env("DYLD_LIBRARY_PATH", &lib_dir);
                command.env("MAGICK_HOME", &imagemagick_dir);
                
                if etc_dir.exists() {
                    info!("Setting MAGICK_CONFIGURE_PATH for test: {}", etc_dir.display());
                    command.env("MAGICK_CONFIGURE_PATH", &etc_dir);
                }
                
                // Don't set MAGICK_CODER_MODULE_PATH - let ImageMagick find modules automatically
            }
        }
    }
    
    let output = command.output()
        .map_err(|e| e.to_string())?;
    
    // Some tools output version info to stderr instead of stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined_output = format!("{}{}", stdout, stderr);
    
    // Check if we got version information (some tools use stderr)
    let version_info = if !stdout.trim().is_empty() {
        &stdout
    } else {
        &stderr
    };
    
    let first_line = version_info.lines().next().unwrap_or("Unknown version");
    
    // Verify the output contains expected strings
    let is_valid = match tool_name.as_str() {
        "ffmpeg" => combined_output.to_lowercase().contains("ffmpeg version"),
        "pandoc" => combined_output.to_lowercase().contains("pandoc"),
        "imagemagick" => {
            let lower = combined_output.to_lowercase();
            lower.contains("imagemagick") || lower.contains("version: imagemagick")
        },
        _ => output.status.success(),
    };
    
    if is_valid {
        Ok(format!("{} is working! {}\n\nLocation: {}", tool_name, first_line, tool_path.display()))
    } else {
        Err(format!("{} test failed", tool_name))
    }
}

#[tauri::command]
async fn get_thumbnail(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    
    // Read the file
    let data = std::fs::read(&path)
        .map_err(|e| format!("Failed to read image file: {}", e))?;
    
    // Get the file extension to determine MIME type
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let mime_type = match extension.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "tiff" | "tif" => "image/tiff",
        _ => "image/jpeg", // default
    };
    
    // Convert to base64
    let base64_data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data);
    
    // Return as data URL
    Ok(format!("data:{};base64,{}", mime_type, base64_data))
}

#[tauri::command]
async fn check_tools_status() -> Result<serde_json::Value, String> {
    let mut status = serde_json::Map::new();
    
    // Check ffmpeg
    let ffmpeg_status = match get_tool_path("ffmpeg") {
        Ok(path) => {
            serde_json::json!({
                "available": true,
                "path": path.to_string_lossy().to_string()
            })
        }
        Err(_) => {
            serde_json::json!({
                "available": false,
                "path": null
            })
        }
    };
    status.insert("ffmpeg".to_string(), ffmpeg_status);
    
    // Check pandoc
    let pandoc_status = match get_tool_path("pandoc") {
        Ok(path) => {
            serde_json::json!({
                "available": true,
                "path": path.to_string_lossy().to_string()
            })
        }
        Err(_) => {
            serde_json::json!({
                "available": false,
                "path": null
            })
        }
    };
    status.insert("pandoc".to_string(), pandoc_status);
    
    // Check imagemagick
    let imagemagick_status = match get_tool_path("imagemagick") {
        Ok(path) => {
            serde_json::json!({
                "available": true,
                "path": path.to_string_lossy().to_string()
            })
        }
        Err(_) => {
            serde_json::json!({
                "available": false,
                "path": null
            })
        }
    };
    status.insert("imagemagick".to_string(), imagemagick_status);
    
    Ok(serde_json::Value::Object(status))
}

#[tauri::command]
async fn set_custom_tool_path(tool_name: String, path: String) -> Result<(), String> {
    info!("Attempting to set custom path for {}: {}", tool_name, path);
    
    // Verify the path exists and is executable
    let tool_path = PathBuf::from(&path);
    if !tool_path.exists() {
        let error_msg = format!("File does not exist: {}", path);
        error!("{}", error_msg);
        return Err(error_msg);
    }
    
    info!("Path exists, verifying it's a valid {} executable...", tool_name);
    
    // Verify it's the correct tool by running -version
    let mut command = create_command(&path);
    
    // FFmpeg uses -version (single dash), while most other tools use --version
    match tool_name.as_str() {
        "ffmpeg" => command.arg("-version"),
        _ => command.arg("--version"),
    };
    
    // On macOS, set environment variables for ImageMagick
    #[cfg(target_os = "macos")]
    if tool_name == "imagemagick" {
        if let Some(bin_dir) = tool_path.parent() {
            if let Some(imagemagick_dir) = bin_dir.parent() {
                let lib_dir = imagemagick_dir.join("lib");
                let etc_dir = imagemagick_dir.join("etc").join("ImageMagick-7");
                
                info!("Setting DYLD_LIBRARY_PATH for verification: {}", lib_dir.display());
                info!("Setting MAGICK_HOME for verification: {}", imagemagick_dir.display());
                
                command.env("DYLD_LIBRARY_PATH", &lib_dir);
                command.env("MAGICK_HOME", &imagemagick_dir);
                
                if etc_dir.exists() {
                    info!("Setting MAGICK_CONFIGURE_PATH for verification: {}", etc_dir.display());
                    command.env("MAGICK_CONFIGURE_PATH", &etc_dir);
                }
                
                // Don't set MAGICK_CODER_MODULE_PATH - let ImageMagick find modules automatically
            }
        }
    }
    
    let version_check = command.output();
    
    match version_check {
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Check if the output contains expected version strings
            // Some tools (like ffmpeg) output version info to stderr instead of stdout
            let combined_output = format!("{}{}", stdout, stderr).to_lowercase();
            
            let is_valid = match tool_name.as_str() {
                "ffmpeg" => combined_output.contains("ffmpeg version"),
                "pandoc" => combined_output.contains("pandoc"),
                "imagemagick" => combined_output.contains("imagemagick") || combined_output.contains("version: imagemagick"),
                _ => output.status.success(),
            };
            
            if is_valid {
                info!("{} verified successfully", tool_name);
                
                // Load config, update it, and save
                let mut config = load_config().unwrap_or_default();
                
                match tool_name.as_str() {
                    "ffmpeg" => config.ffmpeg_path = Some(path.clone()),
                    "pandoc" => config.pandoc_path = Some(path.clone()),
                    "imagemagick" => config.imagemagick_path = Some(path.clone()),
                    _ => return Err(format!("Unknown tool: {}", tool_name)),
                }
                
                save_config(&config)?;
                info!("Custom path saved for {}: {}", tool_name, path);
                Ok(())
            } else {
                let error_msg = format!(
                    "The selected file does not appear to be a valid {} executable.\n\nStdout: {}\n\nStderr: {}", 
                    tool_name, stdout, stderr
                );
                error!("{}", error_msg);
                Err(error_msg)
            }
        }
        Err(e) => {
            let error_msg = format!("Failed to verify tool: {}", e);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
async fn clear_custom_tool_path(tool_name: String) -> Result<(), String> {
    let mut config = load_config().unwrap_or_default();
    
    match tool_name.as_str() {
        "ffmpeg" => config.ffmpeg_path = None,
        "pandoc" => config.pandoc_path = None,
        "imagemagick" => config.imagemagick_path = None,
        _ => return Err(format!("Unknown tool: {}", tool_name)),
    }
    
    save_config(&config)?;
    Ok(())
}

/// Check for updates via Homebrew on macOS
#[cfg(target_os = "macos")]
async fn check_homebrew_updates(package: &str) -> Result<serde_json::Value, String> {
    // Check if package is installed via Homebrew
    let list_output = create_command("brew")
        .arg("list")
        .arg(package)
        .output();
    
    match list_output {
        Ok(output) if output.status.success() => {
            // Get current version
            let info_output = create_command("brew")
                .arg("info")
                .arg(package)
                .output()
                .map_err(|e| e.to_string())?;
            
            let info_str = String::from_utf8_lossy(&info_output.stdout);
            
            // Parse version from brew info output
            // Format is like: "ffmpeg: stable 7.1 (bottled), HEAD"
            let current_version = info_str
                .lines()
                .next()
                .and_then(|line| {
                    // Extract version after "stable"
                    if let Some(stable_pos) = line.find("stable ") {
                        let after_stable = &line[stable_pos + 7..];
                        after_stable.split_whitespace().next().map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or("unknown".to_string());
            
            // Check if updates are available using brew outdated
            let outdated_output = create_command("brew")
                .arg("outdated")
                .arg(package)
                .output()
                .map_err(|e| e.to_string())?;
            
            let update_available = outdated_output.status.success() && 
                                  !outdated_output.stdout.is_empty();
            
            let latest_version = if update_available {
                "newer version available".to_string()
            } else {
                current_version.clone()
            };
            
            Ok(serde_json::json!({
                "installed": true,
                "currentVersion": current_version,
                "updateAvailable": update_available,
                "latestVersion": latest_version
            }))
        }
        _ => {
            Ok(serde_json::json!({
                "installed": false,
                "currentVersion": null,
                "updateAvailable": false,
                "latestVersion": null
            }))
        }
    }
}

#[tauri::command]
async fn check_for_updates() -> Result<serde_json::Value, String> {
    let mut updates = serde_json::Map::new();
    
    // On macOS with Homebrew, check via Homebrew
    #[cfg(target_os = "macos")]
    {
        if is_homebrew_available() {
            let ffmpeg_update = check_homebrew_updates("ffmpeg").await?;
            updates.insert("ffmpeg".to_string(), ffmpeg_update);
            
            let pandoc_update = check_homebrew_updates("pandoc").await?;
            updates.insert("pandoc".to_string(), pandoc_update);
            
            let imagemagick_update = check_homebrew_updates("imagemagick").await?;
            updates.insert("imagemagick".to_string(), imagemagick_update);
            
            return Ok(serde_json::Value::Object(updates));
        }
        // Fall through to manual checking if Homebrew not available
    }
    
    // Manual update checking for all platforms (including macOS without Homebrew)
    // Check FFmpeg
    let ffmpeg_update = match get_tool_path("ffmpeg") {
        Ok(path) => {
            // Get current version
            let output = create_command(&path)
                .arg("-version")
                .output()
                .map_err(|e| e.to_string())?;
            
            let version_str = String::from_utf8_lossy(&output.stdout);
            let current_version = version_str
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(2))
                .unwrap_or("unknown")
                .to_string();
            
            // Try to get latest version from GitHub
            let latest_result = fetch_latest_ffmpeg_version().await;
            let (update_available, latest_version) = match latest_result {
                Ok(latest_tag) => {
                    // GitHub tag is like "autobuild-2024-11-06-12-55"
                    // FFmpeg version output is like "N-121405-g469aad3897-20241009" or "n6.1-39-gde20d6085d"
                    
                    // Extract date from latest tag (format: autobuild-YYYY-MM-DD-HH-MM)
                    let latest_date = if latest_tag.starts_with("autobuild-") {
                        let parts: Vec<&str> = latest_tag.split('-').collect();
                        if parts.len() >= 4 {
                            // Combine YYYY-MM-DD into YYYYMMDD for comparison
                            format!("{}{}{}", parts[1], parts[2], parts[3])
                                .parse::<u32>().ok()
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    
                    // Try to extract date from current version
                    // Format 1: "N-121405-g469aad3897-20241009" → last segment is YYYYMMDD
                    // Format 2: "n6.1-39-gde20d6085d" → no date, always show as update available
                    let current_date = current_version.split('-')
                        .last()
                        .and_then(|last_part| {
                            // Check if it's 8 digits (YYYYMMDD format)
                            if last_part.len() == 8 && last_part.chars().all(|c| c.is_numeric()) {
                                last_part.parse::<u32>().ok()
                            } else {
                                None
                            }
                        });
                    
                    // Compare dates if both available, otherwise assume update is available
                    let update_available = match (current_date, latest_date) {
                        (Some(curr), Some(latest)) => latest > curr,
                        (None, Some(_)) => true, // No date in current version, assume update available
                        _ => false // Can't determine, don't show update
                    };
                    
                    (update_available, Some(latest_tag))
                }
                Err(_) => (false, None)
            };
            
            serde_json::json!({
                "installed": true,
                "currentVersion": current_version,
                "updateAvailable": update_available,
                "latestVersion": latest_version
            })
        }
        Err(_) => {
            serde_json::json!({
                "installed": false,
                "currentVersion": null,
                "updateAvailable": false,
                "latestVersion": null
            })
        }
    };
    updates.insert("ffmpeg".to_string(), ffmpeg_update);
    
    // Check Pandoc
    let pandoc_update = match get_tool_path("pandoc") {
        Ok(path) => {
            let output = create_command(&path)
                .arg("-version")
                .output()
                .map_err(|e| e.to_string())?;
            
            let version_str = String::from_utf8_lossy(&output.stdout);
            let current_version = version_str
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap_or("unknown")
                .to_string();
            
            // Try to get latest version from GitHub
            let latest_result = fetch_latest_pandoc_version().await;
            let (update_available, latest_version) = match latest_result {
                Ok(latest_tag) => {
                    // GitHub tag is like "3.5" or "3.5.1", current_version is also like "3.5"
                    // Remove 'v' prefix from tag if present
                    let latest_clean = latest_tag.trim_start_matches('v').to_string();
                    let update_available = current_version != "unknown" && 
                                          current_version != latest_clean;
                    (update_available, Some(latest_clean))
                }
                Err(_) => (false, None)
            };
            
            serde_json::json!({
                "installed": true,
                "currentVersion": current_version,
                "updateAvailable": update_available,
                "latestVersion": latest_version
            })
        }
        Err(_) => {
            serde_json::json!({
                "installed": false,
                "currentVersion": null,
                "updateAvailable": false,
                "latestVersion": null
            })
        }
    };
    updates.insert("pandoc".to_string(), pandoc_update);
    
    // Check ImageMagick - with dynamic version checking
    let imagemagick_update = match get_tool_path("imagemagick") {
        Ok(path) => {
            let output = create_command(&path)
                .arg("-version")
                .output()
                .map_err(|e| e.to_string())?;
            
            let version_str = String::from_utf8_lossy(&output.stdout);
            let current_version = version_str
                .lines()
                .next()
                .and_then(|line| {
                    // Extract version like "7.1.2-8" from "Version: ImageMagick 7.1.2-8 Q16-HDRI"
                    line.split_whitespace()
                        .find(|s| s.starts_with("7."))
                        .map(|s| s.to_string())
                })
                .unwrap_or("unknown".to_string());
            
            // Try to get latest version
            let latest_result = fetch_latest_imagemagick_version().await;
            let (update_available, latest_version) = match latest_result {
                Ok(latest_filename) => {
                    // Extract version from filename like "ImageMagick-7.1.2-8-portable-Q16-HDRI-x64.7z"
                    // Split by '-' and get parts: ["ImageMagick", "7.1.2", "8", "portable", ...]
                    let parts: Vec<&str> = latest_filename.split('-').collect();
                    let latest_version = if parts.len() >= 3 {
                        // Reconstruct as "7.1.2-8"
                        format!("{}-{}", parts[1], parts[2])
                    } else {
                        "unknown".to_string()
                    };
                    
                    let update_available = if current_version != "unknown" && latest_version != "unknown" {
                        current_version != latest_version
                    } else {
                        false
                    };
                    
                    (update_available, Some(latest_version))
                }
                Err(_) => (false, None)
            };
            
            serde_json::json!({
                "installed": true,
                "currentVersion": current_version,
                "updateAvailable": update_available,
                "latestVersion": latest_version
            })
        }
        Err(_) => {
            serde_json::json!({
                "installed": false,
                "currentVersion": null,
                "updateAvailable": false,
                "latestVersion": null
            })
        }
    };
    updates.insert("imagemagick".to_string(), imagemagick_update);
    
    Ok(serde_json::Value::Object(updates))
}

async fn get_ffmpeg_download_info() -> Result<(String, String, bool), String> {
    // Always use /latest/ endpoint to get the newest version
    if cfg!(target_os = "windows") {
        Ok((
            "https://github.com/BtbN/FFmpeg-Builds/releases/latest/download/ffmpeg-master-latest-win64-gpl.zip".to_string(),
            "ffmpeg-windows.zip".to_string(),
            true,
        ))
    } else if cfg!(target_os = "macos") {
        Ok((
            "https://evermeet.cx/ffmpeg/getrelease/zip".to_string(),
            "ffmpeg-macos.zip".to_string(),
            true,
        ))
    } else {
        Ok((
            "https://github.com/BtbN/FFmpeg-Builds/releases/latest/download/ffmpeg-master-latest-linux64-gpl.tar.xz".to_string(),
            "ffmpeg-linux.tar.xz".to_string(),
            false,
        ))
    }
}

async fn get_pandoc_download_info() -> Result<(String, String, bool), String> {
    // Dynamically fetch the latest version
    let latest_version = fetch_latest_pandoc_version().await?;
    let version_clean = latest_version.trim_start_matches('v');
    
    if cfg!(target_os = "windows") {
        Ok((
            format!("https://github.com/jgm/pandoc/releases/download/{}/pandoc-{}-windows-x86_64.zip", latest_version, version_clean),
            "pandoc-windows.zip".to_string(),
            true,
        ))
    } else if cfg!(target_os = "macos") {
        // For macOS, we'll use the Intel version as it works on both via Rosetta
        Ok((
            format!("https://github.com/jgm/pandoc/releases/download/{}/pandoc-{}-x86_64-macOS.zip", latest_version, version_clean),
            "pandoc-macos.zip".to_string(),
            true,
        ))
    } else {
        Ok((
            format!("https://github.com/jgm/pandoc/releases/download/{}/pandoc-{}-linux-amd64.tar.gz", latest_version, version_clean),
            "pandoc-linux.tar.gz".to_string(),
            false,
        ))
    }
}

async fn get_imagemagick_download_info() -> Result<(String, String, bool), String> {
    if cfg!(target_os = "windows") {
        // For Windows - download from ConvertSave-Libraries GitHub releases
        let github_release_url = "https://github.com/Hunter-Boone/ConvertSave-Libraries/releases/download/latest/imagemagick-windows-x64.7z".to_string();
        
        info!("Downloading ImageMagick for Windows from ConvertSave-Libraries");
        
        Ok((
            github_release_url,
            "imagemagick-windows-x64.7z".to_string(),
            true, // .7z file
        ))
    } else if cfg!(target_os = "macos") {
        // For macOS - download from ConvertSave-Libraries GitHub releases
        // Detect actual machine architecture at runtime (not compile-time)
        // This is important for Universal Binaries or x86_64 apps running under Rosetta 2
        let arch = get_macos_architecture();
        
        // Use ConvertSave-Libraries builds for all macOS versions (built with MACOSX_DEPLOYMENT_TARGET=11.0)
        // These are built on GitHub Actions and should work on macOS 11+
        let macos_version = get_macos_version();
        info!("Detected macOS {}.{} on {} architecture - downloading ImageMagick build (compatible with macOS 11+)", 
              macos_version.0, macos_version.1, arch);
        
        let github_release_url = format!(
            "https://github.com/Hunter-Boone/ConvertSave-Libraries/releases/download/latest/imagemagick-macos-{}.tar.gz",
            arch
        );
        
        Ok((
            github_release_url,
            format!("imagemagick-macos-{}.tar.gz", arch),
            false,
        ))
    } else {
        // For Linux - download from ConvertSave-Libraries GitHub releases
        let github_release_url = "https://github.com/Hunter-Boone/ConvertSave-Libraries/releases/download/latest/imagemagick-linux-x64.tar.gz".to_string();
        
        info!("Downloading ImageMagick for Linux from ConvertSave-Libraries");
        
        Ok((
            github_release_url,
            "imagemagick-linux-x64.tar.gz".to_string(),
            false,
        ))
    }
}

/// Fetches the latest ImageMagick portable version from the binaries page
async fn fetch_latest_imagemagick_version() -> Result<String, String> {
    println!("Fetching latest ImageMagick version from binaries page...");
    
    let url = "https://imagemagick.org/archive/binaries/";
    let client = create_http_client()?;
    let response = client.get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch binaries page: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to fetch binaries page: HTTP {}", response.status()));
    }
    
    let html = response.text()
        .await
        .map_err(|e| format!("Failed to read binaries page: {}", e))?;
    
    // Parse HTML to find latest portable Q16-HDRI-x64.7z file
    // Looking for pattern: ImageMagick-7.1.X-XX-portable-Q16-HDRI-x64.7z
    let pattern = r#"ImageMagick-7\.\d+\.\d+-\d+-portable-Q16-HDRI-x64\.7z"#;
    let re = regex::Regex::new(pattern).map_err(|e| format!("Regex error: {}", e))?;
    
    let mut versions: Vec<String> = re
        .find_iter(&html)
        .map(|m| m.as_str().to_string())
        .collect();
    
    if versions.is_empty() {
        return Err("No portable ImageMagick versions found on binaries page".to_string());
    }
    
    // Sort versions to get the latest (lexicographic sort works for this format)
    versions.sort();
    versions.reverse();
    
    let latest = versions[0].clone();
    println!("Found latest ImageMagick version: {}", latest);
    
    Ok(latest)
}

/// Fetches the latest FFmpeg version from GitHub API
async fn fetch_latest_ffmpeg_version() -> Result<String, String> {
    println!("Fetching latest FFmpeg version from GitHub...");
    
    // Fetch the most recent releases (not /latest, as that might return a "latest" tag)
    let url = "https://api.github.com/repos/BtbN/FFmpeg-Builds/releases?per_page=10";
    let client = create_http_client()?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch FFmpeg releases: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to fetch FFmpeg releases: HTTP {}", response.status()));
    }
    
    let json: serde_json::Value = response.json()
        .await
        .map_err(|e| format!("Failed to parse FFmpeg release data: {}", e))?;
    
    // Get the first release that starts with "autobuild-" (skip any "latest" or other tags)
    let releases = json.as_array()
        .ok_or("Expected array of releases")?;
    
    for release in releases {
        if let Some(tag_name) = release["tag_name"].as_str() {
            if tag_name.starts_with("autobuild-") {
                println!("Found latest FFmpeg version: {}", tag_name);
                return Ok(tag_name.to_string());
            }
        }
    }
    
    Err("Could not find any autobuild releases".to_string())
}

/// Fetches the latest Pandoc version from GitHub API
async fn fetch_latest_pandoc_version() -> Result<String, String> {
    println!("Fetching latest Pandoc version from GitHub...");
    
    let url = "https://api.github.com/repos/jgm/pandoc/releases/latest";
    let client = create_http_client()?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Pandoc releases: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to fetch Pandoc releases: HTTP {}", response.status()));
    }
    
    let json: serde_json::Value = response.json()
        .await
        .map_err(|e| format!("Failed to parse Pandoc release data: {}", e))?;
    
    let tag_name = json["tag_name"]
        .as_str()
        .ok_or("Could not find Pandoc tag_name")?
        .to_string();
    
    println!("Found latest Pandoc version: {}", tag_name);
    Ok(tag_name)
}

/// Fix hardcoded library paths in ImageMagick binary on macOS
#[cfg(target_os = "macos")]
#[allow(dead_code)]
fn fix_imagemagick_library_paths(binary_path: &PathBuf, install_dir: &PathBuf) -> Result<(), String> {
    use std::process::Command;
    
    println!("Fixing library paths in ImageMagick binary...");
    
    // Get list of library dependencies using otool
    let otool_output = Command::new("otool")
        .arg("-L")
        .arg(binary_path)
        .output()
        .map_err(|e| format!("Failed to run otool: {}", e))?;
    
    let dependencies = String::from_utf8_lossy(&otool_output.stdout);
    println!("Current library dependencies:\n{}", dependencies);
    
    // Find all absolute paths that need to be fixed
    let mut libs_to_fix = Vec::new();
    for line in dependencies.lines().skip(1) { // Skip first line (the binary itself)
        let trimmed = line.trim();
        if trimmed.starts_with('/') {
            // Extract the library path (before the version info in parentheses)
            if let Some(lib_path) = trimmed.split_whitespace().next() {
                // Fix paths that are absolute (ImageMagick libs, X11 libs, etc.)
                if lib_path.contains("ImageMagick") || 
                   lib_path.contains("/lib/") || 
                   lib_path.contains("/opt/") ||
                   lib_path.contains("/usr/local/") {
                    libs_to_fix.push(lib_path.to_string());
                }
            }
        }
    }
    
    println!("Found {} libraries to fix", libs_to_fix.len());
    
    // Fix each library path
    for old_path in libs_to_fix {
        // Extract just the library filename
        if let Some(lib_name) = std::path::Path::new(&old_path).file_name() {
            let lib_name_str = lib_name.to_string_lossy();
            
            // Check if this library exists in the same directory as the binary
            let new_lib_path = install_dir.join(lib_name_str.as_ref());
            if new_lib_path.exists() {
                // Use @executable_path to make the path relative to the binary
                // Dylibs are now in the same directory, so no lib/ subdirectory
                let relative_path = format!("@executable_path/{}", lib_name_str);
                
                println!("Changing {} -> {}", old_path, relative_path);
                
                let result = Command::new("install_name_tool")
                    .arg("-change")
                    .arg(&old_path)
                    .arg(&relative_path)
                    .arg(binary_path)
                    .output();
                
                match result {
                    Ok(output) if output.status.success() => {
                        println!("  ✓ Successfully updated");
                    }
                    Ok(output) => {
                        println!("  ⚠ Warning: {}", String::from_utf8_lossy(&output.stderr));
                    }
                    Err(e) => {
                        println!("  ✗ Failed: {}", e);
                    }
                }
                
                // Also fix the library file itself if it references other libraries
                if let Err(e) = fix_library_references(&new_lib_path, &install_dir) {
                    println!("  Warning: Failed to fix references in {}: {}", lib_name_str, e);
                }
            } else {
                println!("  ⚠ Library not found in install directory: {}", lib_name_str);
            }
        }
    }
    
    println!("Library path fixing complete!");
    Ok(())
}

/// Fix library references within a dylib file
#[cfg(target_os = "macos")]
#[allow(dead_code)]
fn fix_library_references(lib_path: &PathBuf, install_dir: &PathBuf) -> Result<(), String> {
    use std::process::Command;
    
    // Get dependencies of this library
    let otool_output = Command::new("otool")
        .arg("-L")
        .arg(lib_path)
        .output()
        .map_err(|e| format!("Failed to run otool on library: {}", e))?;
    
    let dependencies = String::from_utf8_lossy(&otool_output.stdout);
    
    for line in dependencies.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.starts_with('/') {
            if let Some(dep_path) = trimmed.split_whitespace().next() {
                if dep_path.contains("ImageMagick") || dep_path.contains("/lib/") {
                    if let Some(dep_name) = std::path::Path::new(dep_path).file_name() {
                        let dep_name_str = dep_name.to_string_lossy();
                        let new_dep_path = install_dir.join(dep_name_str.as_ref());
                        
                        if new_dep_path.exists() {
                            // Use @loader_path since dylibs are in the same directory
                            let relative_path = format!("@loader_path/{}", dep_name_str);
                            
                            let _ = Command::new("install_name_tool")
                                .arg("-change")
                                .arg(dep_path)
                                .arg(&relative_path)
                                .arg(lib_path)
                                .output();
                        }
                    }
                }
            }
        }
    }
    
    // Also fix the library's own ID if it's absolute
    let _id_output = Command::new("otool")
        .arg("-D")
        .arg(lib_path)
        .output()
        .map_err(|e| format!("Failed to get library ID: {}", e))?;
    
    if let Some(lib_name) = lib_path.file_name() {
        let lib_name_str = lib_name.to_string_lossy();
        let new_id = format!("@loader_path/{}", lib_name_str);
        
        let _ = Command::new("install_name_tool")
            .arg("-id")
            .arg(&new_id)
            .arg(lib_path)
            .output();
    }
    
    Ok(())
}

fn extract_zip(archive_path: &PathBuf, extract_dir: &PathBuf, binary_name: &str) -> Result<(), String> {
    let file = std::fs::File::open(archive_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    
    println!("ZIP archive has {} files", archive.len());
    
    let exe_name = if cfg!(windows) {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    };
    
    // First, try to find the binary directly in the ZIP
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let path = file.enclosed_name().unwrap_or_else(|| std::path::Path::new(""));
        
        println!("Checking file in ZIP: {}", path.display());
        
        if let Some(filename) = path.file_name() {
            if filename == exe_name.as_str() || filename.to_string_lossy().ends_with(&exe_name) {
                println!("Found binary directly in ZIP: {}", path.display());
                let outpath = extract_dir.join(&exe_name);
                let mut outfile = std::fs::File::create(&outpath).map_err(|e| e.to_string())?;
                std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
                
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(0o755))
                        .map_err(|e| e.to_string())?;
                }
                return Ok(());
            }
        }
    }
    
    // If not found directly, extract everything and search recursively
    // This handles nested archives (like .7z.zip files)
    println!("Binary not found directly in ZIP, extracting all files...");
    let temp_extract = extract_dir.join("temp_extract");
    std::fs::create_dir_all(&temp_extract).map_err(|e| e.to_string())?;
    
    let file = std::fs::File::open(archive_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => temp_extract.join(path),
            None => continue,
        };
        
        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
            }
            let mut outfile = std::fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }
    
    // Now search for the binary in the extracted files
    fn find_binary(dir: &std::path::Path, name: &str) -> Option<std::path::PathBuf> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        if filename.to_string_lossy() == name {
                            return Some(path);
                        }
                    }
                } else if path.is_dir() {
                    if let Some(found) = find_binary(&path, name) {
                        return Some(found);
                    }
                }
            }
        }
        None
    }
    
    if let Some(binary_path) = find_binary(&temp_extract, &exe_name) {
        println!("Found binary at: {}", binary_path.display());
        let outpath = extract_dir.join(&exe_name);
        std::fs::copy(&binary_path, &outpath).map_err(|e| e.to_string())?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(0o755))
                .map_err(|e| e.to_string())?;
        }
        
        // Clean up temp extraction
        std::fs::remove_dir_all(&temp_extract).ok();
        return Ok(());
    }
    
    // Clean up temp extraction
    std::fs::remove_dir_all(&temp_extract).ok();
    
    Err(format!("{} binary not found in archive (checked all files)", binary_name))
}

// Extract ALL files from a tar.gz archive (used for ImageMagick to get dylibs)
fn extract_tar_gz_all(archive_path: &PathBuf, extract_dir: &PathBuf) -> Result<(), String> {
    let file = std::fs::File::open(archive_path).map_err(|e| e.to_string())?;
    
    if archive_path.extension().and_then(|s| s.to_str()) == Some("xz") {
        // Decompress XZ file to memory first, then create tar archive
        let mut buf_reader = std::io::BufReader::new(file);
        let mut decompressed_data = Vec::new();
        lzma_rs::xz_decompress(&mut buf_reader, &mut decompressed_data).map_err(|e| e.to_string())?;
        let mut archive = tar::Archive::new(std::io::Cursor::new(decompressed_data));
        
        // Extract all files
        archive.unpack(extract_dir).map_err(|e| e.to_string())?;
    } else {
        let dec = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(dec);
        
        // Extract all files
        archive.unpack(extract_dir).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

fn extract_tar_gz(archive_path: &PathBuf, extract_dir: &PathBuf, binary_name: &str) -> Result<(), String> {
    let file = std::fs::File::open(archive_path).map_err(|e| e.to_string())?;
    
    let exe_name = if cfg!(windows) {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    };
    
    if archive_path.extension().and_then(|s| s.to_str()) == Some("xz") {
        // Decompress XZ file to memory first, then create tar archive
        let mut buf_reader = std::io::BufReader::new(file);
        let mut decompressed_data = Vec::new();
        lzma_rs::xz_decompress(&mut buf_reader, &mut decompressed_data).map_err(|e| e.to_string())?;
        let mut archive = tar::Archive::new(std::io::Cursor::new(decompressed_data));
        
        for entry in archive.entries().map_err(|e| e.to_string())? {
            let mut entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path().map_err(|e| e.to_string())?;
            
            if let Some(filename) = path.file_name() {
                if filename == std::ffi::OsStr::new(&exe_name) || filename.to_string_lossy().ends_with(&exe_name) {
                    let outpath = extract_dir.join(&exe_name);
                    entry.unpack(&outpath).map_err(|e| e.to_string())?;
                    
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(0o755))
                            .map_err(|e| e.to_string())?;
                    }
                    return Ok(());
                }
            }
        }
    } else {
        let dec = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(dec);
        
        for entry in archive.entries().map_err(|e| e.to_string())? {
            let mut entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path().map_err(|e| e.to_string())?;
            
            if let Some(filename) = path.file_name() {
                if filename == std::ffi::OsStr::new(&exe_name) || filename.to_string_lossy().ends_with(&exe_name) {
                    let outpath = extract_dir.join(&exe_name);
                    entry.unpack(&outpath).map_err(|e| e.to_string())?;
                    
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(0o755))
                            .map_err(|e| e.to_string())?;
                    }
                    return Ok(());
                }
            }
        }
    }
    
    Err(format!("{} binary not found in archive", binary_name))
}

// ═══════════════════════════════════════════════════════════════════════════
// LICENSE COMMANDS
// ═══════════════════════════════════════════════════════════════════════════

/// Check the current license status
/// Called on app startup to determine if user is licensed
#[tauri::command]
async fn check_license_status() -> Result<license::LicenseStatus, String> {
    info!("Checking license status...");
    let status = license::check_license_status().await;
    info!("License status: {:?}", status);
    Ok(status)
}

/// Activate the app with a product key
#[tauri::command]
async fn activate_license(product_key: String, device_name: Option<String>) -> Result<license::LicenseStatus, String> {
    info!("Activating license with product key...");
    match license::activate_with_product_key(&product_key, device_name.as_deref()).await {
        Ok(status) => {
            info!("License activated successfully");
            Ok(status)
        }
        Err(e) => {
            error!("License activation failed: {}", e);
            Err(e)
        }
    }
}

/// Deactivate this device
#[tauri::command]
async fn deactivate_license() -> Result<(), String> {
    info!("Deactivating license...");
    match license::deactivate_device().await {
        Ok(()) => {
            info!("License deactivated successfully");
            Ok(())
        }
        Err(e) => {
            error!("License deactivation failed: {}", e);
            Err(e)
        }
    }
}

/// Get the device's MAC address (for display in settings)
#[tauri::command]
fn get_device_id() -> Result<String, String> {
    license::get_mac_address()
}

/// Get the current product key from local license
#[tauri::command]
fn get_current_product_key() -> Result<String, String> {
    license::get_current_product_key()
}

/// Change the product key for this device
#[tauri::command]
async fn change_product_key(new_product_key: String, device_name: Option<String>) -> Result<license::LicenseStatus, String> {
    info!("Changing product key...");
    match license::change_product_key(&new_product_key, device_name.as_deref()).await {
        Ok(status) => {
            info!("Product key changed successfully");
            Ok(status)
        }
        Err(e) => {
            error!("Product key change failed: {}", e);
            Err(e)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { 
                        file_name: Some("convertsave".to_string()) 
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .setup(|app| {
            // Log the actual log directory being used
            if let Some(log_dir) = app.path().app_log_dir().ok() {
                println!("Logs will be written to: {}", log_dir.display());
            }
            
            info!("ConvertSave application started");
            info!("Version: {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_available_formats,
            convert_file,
            convert_images_to_multipage_pdf,
            get_file_info,
            get_thumbnail,
            test_directories,
            open_folder,
            download_ffmpeg,
            download_pandoc,
            download_imagemagick,
            test_tool,
            check_tools_status,
            check_for_updates,
            set_custom_tool_path,
            clear_custom_tool_path,
            get_log_directory,
            open_log_directory,
            check_app_update,
            install_app_update,
            // License commands
            check_license_status,
            activate_license,
            deactivate_license,
            get_device_id,
            get_current_product_key,
            change_product_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
