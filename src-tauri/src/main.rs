// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use dirs;
use serde_json;
use tauri::{AppHandle, Emitter, Manager};

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

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_available_formats(input_extension: String) -> Vec<ConversionOption> {
    // Debug logging
    println!("=== GET_AVAILABLE_FORMATS ===");
    println!("Input extension: '{}'", input_extension);
    
    // This is a simplified version - in production, you'd have more sophisticated mapping
    let mut options = Vec::new();
    
    // Convert to lowercase for case-insensitive matching
    let input_extension = input_extension.to_lowercase();
    
    match input_extension.as_str() {
        "mp4" | "mov" | "avi" | "mkv" => {
            options.push(ConversionOption {
                format: "mp3".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "Audio Only".to_string(),
                color: "green".to_string(),
            });
            options.push(ConversionOption {
                format: "webm".to_string(),
                tool: "ffmpeg".to_string(),
                display_name: "WebM Video".to_string(),
                color: "blue".to_string(),
            });
        }        "docx" | "doc" | "odt" => {
            options.push(ConversionOption {
                format: "pdf".to_string(),
                tool: "libreoffice".to_string(),
                display_name: "PDF Document".to_string(),
                color: "pink".to_string(),
            });
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
        "png" | "jpg" | "jpeg" | "bmp" | "tiff" | "webp" | "gif" | "heic" | "heif" | "avif" | "tga" | "ppm" | "pgm" | "pbm" | "pam" | "xbm" | "xpm" | "dds" | "dpx" | "exr" | "hdr" | "ico" | "j2k" | "jp2" | "pcx" | "pfm" | "sgi" | "sun" | "xwd" => {
            // Standard formats
            if input_extension != "jpg" && input_extension != "jpeg" {
                options.push(ConversionOption {
                    format: "jpg".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "JPEG Image".to_string(),
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
            if input_extension != "heic" && input_extension != "heif" {
                options.push(ConversionOption {
                    format: "heic".to_string(),
                    tool: "ffmpeg".to_string(),
                    display_name: "HEIC (Apple)".to_string(),
                    color: "aquamarine".to_string(),
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
        }
        _ => {
            println!("No match for extension: '{}'", input_extension);
        }
    }
    
    println!("Returning {} format options", options.len());
    options
}

#[tauri::command]
async fn convert_file(
    input_path: String,
    output_format: String,
    output_directory: Option<String>,
    advanced_options: Option<String>,
) -> Result<String, String> {
    // DEBUG: Print what we're doing
    println!("=== CONVERSION DEBUG ===");
    println!("Input: {}", input_path);
    println!("Output format: {}", output_format);
    println!("Custom output dir: {:?}", output_directory);
    
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
    
    let output_path = output_dir.join(format!("{}.{}", file_stem, output_format));
    
    // Determine which tool to use and perform the actual conversion
    let output_format_lower = output_format.to_lowercase();
    let conversion_result = match determine_conversion_tool(&input_extension, &output_format_lower) {
        Some(tool) => {
            execute_conversion(tool, &input_path, &output_path, advanced_options).await
        }
        None => {
            return Err(format!("No conversion tool available for {} to {}", input_extension, output_format));
        }
    };
    
    match conversion_result {
        Ok(_) => {
            Ok(format!("File converted successfully to: {}", output_path.to_string_lossy()))
        }
        Err(e) => {
            Err(e)
        }
    }
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
    let folder = if path.is_file() {
        path.parent().ok_or("Could not find parent folder")?
    } else {
        &path
    };
    
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(folder)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    Ok(())
}

fn determine_conversion_tool(input_ext: &str, output_ext: &str) -> Option<&'static str> {
    // Image conversions - ffmpeg can handle many image formats including HEIC
    let image_inputs = [
        // Standard formats
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
        // Modern formats
        "heic", "heif", "avif",
        // Professional/High-end formats
        "tga", "exr", "hdr", "dpx", "pfm",
        // JPEG 2000
        "j2k", "jp2",
        // Legacy/Specialized formats
        "pcx", "ico", "sgi", "sun",
        // Raw/Uncompressed formats
        "ppm", "pgm", "pbm", "pam",
        // X Window System formats
        "xbm", "xpm", "xwd",
        // Gaming/3D formats
        "dds"
    ];
    let image_outputs = [
        // Standard formats
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
        // Modern formats
        "heic", "heif", "avif",
        // Professional/High-end formats
        "tga", "exr", "hdr", "dpx", "pfm",
        // JPEG 2000
        "j2k", "jp2",
        // Legacy/Specialized formats
        "pcx", "ico", "sgi", "sun",
        // Raw/Uncompressed formats
        "ppm", "pgm", "pbm", "pam",
        // X Window System formats
        "xbm", "xpm", "xwd",
        // Gaming/3D formats
        "dds"
    ];
    
    // Video/Audio conversions
    let video_inputs = ["mp4", "mov", "avi", "mkv", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "3gp"];
    let audio_inputs = ["mp3", "wav", "flac", "ogg", "m4a", "wma", "aac"];
    let av_outputs = ["mp4", "mov", "avi", "mkv", "webm", "mp3", "wav", "flac", "ogg", "m4a", "aac", "gif"];
    
    // Document conversions
    let doc_inputs = ["md", "markdown", "txt", "html", "htm", "docx", "odt", "rtf", "tex", "latex", "epub", "rst"];
    let doc_outputs = ["md", "html", "pdf", "docx", "odt", "rtf", "tex", "epub", "txt"];
    
    let office_inputs = ["doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf"];
    let office_outputs = ["pdf", "html", "txt", "docx", "odt", "rtf"];
    
    // Use ffmpeg for media and image conversions
    if (video_inputs.contains(&input_ext) || audio_inputs.contains(&input_ext)) && av_outputs.contains(&output_ext) {
        Some("ffmpeg")
    } else if image_inputs.contains(&input_ext) && image_outputs.contains(&output_ext) {
        Some("ffmpeg")
    } else if doc_inputs.contains(&input_ext) && doc_outputs.contains(&output_ext) {
        Some("pandoc")
    } else if office_inputs.contains(&input_ext) && office_outputs.contains(&output_ext) {
        Some("libreoffice")
    } else {
        None
    }
}

fn get_tool_path(tool_name: &str) -> Result<PathBuf, String> {
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
        "pandoc" => {
            if cfg!(target_os = "windows") {
                "pandoc.exe"
            } else {
                "pandoc"
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
        // Our identifier from tauri.conf.json is "com.convertsave.app"
        let app_data_path = data_dir
            .join("com.convertsave.app")
            .join(tool_name)
            .join(exe_name);
        possible_paths.push(app_data_path);
    }
    
    // 2. Project root tools directory (development)
    if let Ok(current) = std::env::current_dir() {
        possible_paths.push(current.join("tools").join(platform_name).join(exe_name));
    }
    
    // 3. Relative to executable (production)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            possible_paths.push(parent.join("tools").join(platform_name).join(exe_name));
        }
    }
    
    // 4. Parent directory of executable + tools (alternative production layout)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent().and_then(|p| p.parent()) {
            possible_paths.push(parent.join("tools").join(platform_name).join(exe_name));
        }
    }
    
    // 5. Check if we're in src-tauri directory during development
    if let Ok(current) = std::env::current_dir() {
        if let Some(parent) = current.parent() {
            possible_paths.push(parent.join("tools").join(platform_name).join(exe_name));
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
    
    Err(format!("Tool not found: {} (checked: {})", tool_name, checked_paths.join(", ")))
}

// Helper function to handle HEIC tile grid reassembly
fn convert_heic_with_tiles(
    tool_path: &PathBuf,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), String> {
    // Step 1: Get metadata to find tile grid dimensions and rotation
    let metadata_output = Command::new(tool_path)
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
        println!("HEIC tile grid: {}", tile_grid_line);
        
        use std::str::FromStr;
        for word in tile_grid_line.split_whitespace() {
            if word.contains('x') && !word.starts_with("0x") {
                if let Some((w, h)) = word.split_once('x') {
                    if let (Ok(w_val), Ok(h_val)) = (u32::from_str(w), u32::from_str(h)) {
                        if w_val >= 100 && w_val < 100000 && h_val >= 100 && h_val < 100000 {
                            width = w_val;
                            height = h_val;
                            println!("HEIC resolution: {}x{}", width, height);
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
        println!("HEIC rotation: -90 degrees");
    } else if stderr.contains("rotation of 90") {
        has_rotation = true;
        rotation_degrees = 90;
        println!("HEIC rotation: 90 degrees");
    } else if stderr.contains("rotation of 180") || stderr.contains("rotation of -180") {
        has_rotation = true;
        rotation_degrees = 180;
        println!("HEIC rotation: 180 degrees");
    }
    
    if width == 0 || height == 0 {
        return Err("Could not determine HEIC tile grid dimensions".to_string());
    }
    
    // Step 2: Create temp directory for tiles
    let temp_dir = std::env::temp_dir().join(format!("heic_tiles_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // Step 3: Extract tiles
    println!("Extracting HEIC tiles to: {}", temp_dir.display());
    let tile_pattern = temp_dir.join("tile_%02d.png");
    
    let extract_output = Command::new(tool_path)
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
    println!("Tile grid: {}x{} ({}x{} tiles)", cols, rows, cols * tile_size, rows * tile_size);
    
    // Step 5: Stitch tiles together
    let stitched_path = temp_dir.join("stitched.png");
    let tile_input = temp_dir.join("tile_%02d.png");
    
    let stitch_output = Command::new(tool_path)
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
    let mut final_command = Command::new(tool_path);
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

async fn execute_conversion(
    tool_name: &str,
    input_path: &PathBuf,
    output_path: &PathBuf,
    advanced_options: Option<String>,
) -> Result<(), String> {
    let tool_path = get_tool_path(tool_name)?;
    
    let mut command = Command::new(&tool_path);
    
    match tool_name {
        "ffmpeg" => {
            // Check input format for special HEIC handling
            let input_ext = input_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            // HEIC/HEIF files need special tile reassembly handling
            if input_ext == "heic" || input_ext == "heif" {
                return convert_heic_with_tiles(&tool_path, input_path, output_path);
            }
            
            command.arg("-i").arg(input_path);
            
            // Check if we're converting to HEIC/HEIF and add required codec settings
            let output_ext = output_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            if output_ext == "heic" || output_ext == "heif" {
                // HEIC/HEIF requires libx265 codec with specific settings
                // Force MOV format since FFmpeg doesn't have a native HEIC muxer
                command.arg("-f").arg("mov");
                command.arg("-c:v").arg("libx265");
                command.arg("-tag:v").arg("hvc1");
                // Optional: Set quality (lower CRF = higher quality, 23 is default)
                command.arg("-crf").arg("23");
            } else if output_ext == "avif" {
                // AVIF uses libaom-av1 or libsvtav1 codec
                command.arg("-c:v").arg("libaom-av1");
                command.arg("-crf").arg("30");
            }
            
            // Add advanced options if provided
            if let Some(options) = advanced_options {
                let options_parts: Vec<&str> = options.split_whitespace().collect();
                for part in options_parts {
                    command.arg(part);
                }
            }
            
            // For video/multi-frame input to single image output, specify one frame
            let video_formats = ["mp4", "mov", "avi", "mkv", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "3gp", "ogv"];
            let image_formats = ["jpg", "jpeg", "png", "webp", "bmp", "gif", "tiff", "tif", "ico"];
            
            if video_formats.contains(&input_ext.as_str()) && image_formats.contains(&output_ext.as_str()) {
                command.arg("-frames:v").arg("1");
            }
            
            // For single image output, use -update flag to write one file (not a sequence)
            if image_formats.contains(&output_ext.as_str()) {
                command.arg("-update").arg("1");
            }
            
            command.arg("-y").arg(output_path); // -y to overwrite output file
        }
        "pandoc" => {
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
    println!("Executing command: {:?}", command);
    
    let output = command.output()
        .map_err(|e| format!("Failed to execute {}: {}", tool_name, e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Log full output for debugging
        println!("=== COMMAND FAILED ===");
        println!("STDOUT:\n{}", stdout);
        println!("STDERR:\n{}", stderr);
        println!("======================");
        
        // Provide user-friendly error messages for common issues
        let error_msg = if stderr.contains("does not contain any stream") {
            if tool_name == "ffmpeg" {
                "This video file has no audio stream. Cannot convert to audio format. Try converting to a video format instead.".to_string()
            } else {
                "The file does not contain the required streams for this conversion.".to_string()
            }
        } else if stderr.contains("Unable to choose an output format") || (stderr.contains("use a standard extension") && (stderr.contains("heic") || stderr.contains("heif") || stderr.contains("avif"))) {
            "HEIC/HEIF/AVIF encoding is not supported by this FFmpeg build.\n\nThese formats require special muxers that are not available. Try converting to:\n• JPG (best compatibility)\n• PNG (lossless)\n• WebP (modern, efficient)".to_string()
        } else if stderr.contains("Unknown encoder") || stderr.contains("Encoder not found") || stderr.contains("libx265") || stderr.contains("libaom-av1") {
            "The required codec is not available in this FFmpeg build.\n\nTry converting to a different format like JPG, PNG, or WebP.".to_string()
        } else if stderr.contains("Invalid argument") && stderr.contains("Error opening output file") {
            "Cannot write to the output location. This may be due to:\n- Network drive access issues\n- Insufficient permissions\n- Invalid file path\n\nTry saving to a local drive instead.".to_string()
        } else if stderr.contains("No such file or directory") || stderr.contains("does not exist") {
            "Input file not found. The file may have been moved or deleted.".to_string()
        } else {
            // For other errors, show the technical details
            format!("Conversion failed. Error details:\n{}", stderr)
        };
        
        Err(error_msg)
    }
}

// Binary download functions

#[tauri::command]
async fn download_ffmpeg(app: AppHandle) -> Result<String, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    
    let (download_url, filename, is_zip) = get_ffmpeg_download_info()?;
    let ffmpeg_path = data_dir.join("ffmpeg").join(if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" });
    
    if ffmpeg_path.exists() {
        return Ok("FFmpeg already downloaded".to_string());
    }
    
    app.emit("download-progress", DownloadProgress {
        status: "downloading".to_string(),
        message: "Downloading FFmpeg...".to_string(),
    }).map_err(|e| e.to_string())?;
    
    let response = reqwest::get(&download_url).await.map_err(|e| e.to_string())?;
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
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    
    let (download_url, filename, is_zip) = get_pandoc_download_info()?;
    let pandoc_path = data_dir.join("pandoc").join(if cfg!(windows) { "pandoc.exe" } else { "pandoc" });
    
    if pandoc_path.exists() {
        return Ok("Pandoc already downloaded".to_string());
    }
    
    app.emit("download-progress", DownloadProgress {
        status: "downloading".to_string(),
        message: "Downloading Pandoc...".to_string(),
    }).map_err(|e| e.to_string())?;
    
    let response = reqwest::get(&download_url).await.map_err(|e| e.to_string())?;
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
async fn test_tool(tool_name: String) -> Result<String, String> {
    let tool_path = match get_tool_path(&tool_name) {
        Ok(path) => path,
        Err(_) => {
            return Err(format!("{} not found. Please download it first.", tool_name));
        }
    };
    
    let output = Command::new(&tool_path)
        .arg("-version")
        .output()
        .map_err(|e| e.to_string())?;
    
    if output.status.success() {
        let version_info = String::from_utf8_lossy(&output.stdout);
        let first_line = version_info.lines().next().unwrap_or("Unknown version");
        Ok(format!("{} is working! {}\n\nLocation: {}", tool_name, first_line, tool_path.display()))
    } else {
        Err(format!("{} test failed", tool_name))
    }
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
    
    Ok(serde_json::Value::Object(status))
}

fn get_ffmpeg_download_info() -> Result<(String, String, bool), String> {
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

fn get_pandoc_download_info() -> Result<(String, String, bool), String> {
    // Note: Pandoc releases use version-specific filenames
    // We use specific known versions that exist on GitHub
    // In the future, this could use the GitHub API to get the latest release dynamically
    if cfg!(target_os = "windows") {
        Ok((
            "https://github.com/jgm/pandoc/releases/download/3.5/pandoc-3.5-windows-x86_64.zip".to_string(),
            "pandoc-windows.zip".to_string(),
            true,
        ))
    } else if cfg!(target_os = "macos") {
        // For macOS, we'll use the Intel version as it works on both via Rosetta
        Ok((
            "https://github.com/jgm/pandoc/releases/download/3.5/pandoc-3.5-x86_64-macOS.zip".to_string(),
            "pandoc-macos.zip".to_string(),
            true,
        ))
    } else {
        Ok((
            "https://github.com/jgm/pandoc/releases/download/3.5/pandoc-3.5-linux-amd64.tar.gz".to_string(),
            "pandoc-linux.tar.gz".to_string(),
            false,
        ))
    }
}

fn extract_zip(archive_path: &PathBuf, extract_dir: &PathBuf, binary_name: &str) -> Result<(), String> {
    let file = std::fs::File::open(archive_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    
    let exe_name = if cfg!(windows) {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    };
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let path = file.enclosed_name().unwrap_or_else(|| std::path::Path::new(""));
        
        if let Some(filename) = path.file_name() {
            if filename == exe_name.as_str() || filename.to_string_lossy().ends_with(&exe_name) {
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
    
    Err(format!("{} binary not found in archive", binary_name))
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            get_available_formats,
            convert_file,
            get_file_info,
            test_directories,
            open_folder,
            download_ffmpeg,
            download_pandoc,
            test_tool,
            check_tools_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
