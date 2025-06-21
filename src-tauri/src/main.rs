// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use dirs;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct ConversionOption {
    format: String,
    tool: String,
    display_name: String,
    color: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_available_formats(input_extension: String) -> Vec<ConversionOption> {
    // This is a simplified version - in production, you'd have more sophisticated mapping
    let mut options = Vec::new();
    
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
        "png" | "jpg" | "jpeg" | "bmp" | "tiff" | "webp" | "gif" | "tga" | "ppm" | "pgm" | "pbm" | "pam" | "xbm" | "xpm" | "dds" | "dpx" | "exr" | "hdr" | "ico" | "j2k" | "jp2" | "pcx" | "pfm" | "sgi" | "sun" | "xwd" => {
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
        _ => {}
    }
    
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
    
    // DEBUG: Print the paths
    println!("Output directory: {}", output_dir.to_string_lossy());
    println!("Full output path: {}", output_path.to_string_lossy());
    
    // Determine which tool to use and perform the actual conversion
    let output_format_lower = output_format.to_lowercase();
    println!("Looking for conversion tool: {} -> {}", input_extension, output_format_lower);
    let conversion_result = match determine_conversion_tool(&input_extension, &output_format_lower) {
        Some(tool) => {
            println!("Using {} for conversion", tool);
            execute_conversion(tool, &input_path, &output_path, advanced_options).await
        }
        None => {
            return Err(format!("No conversion tool available for {} to {}", input_extension, output_format));
        }
    };
    
    match conversion_result {
        Ok(_) => {
            println!("=== CONVERSION SUCCESSFUL ===");
            Ok(format!("File converted successfully to: {}", output_path.to_string_lossy()))
        }
        Err(e) => {
            println!("=== CONVERSION FAILED ===");
            println!("Error: {}", e);
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
        .unwrap_or("");
    
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
    // Image conversions - ffmpeg can handle many image formats
    let image_inputs = [
        // Standard formats
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp",
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
    
    // Use ffmpeg for both media and image conversions since it's available and versatile
    if (video_inputs.contains(&input_ext) || audio_inputs.contains(&input_ext)) && av_outputs.contains(&output_ext) {
        Some("ffmpeg")
    } else if image_inputs.contains(&input_ext) && image_outputs.contains(&output_ext) {
        Some("ffmpeg")  // Use ffmpeg for image conversions
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
    let possible_paths = vec![
        // 1. Project root tools directory (development)
        std::env::current_dir()
            .map(|d| d.join("tools").join(platform_name).join(exe_name))
            .unwrap_or_else(|_| PathBuf::new()),
        
        // 2. Relative to executable (production)
        std::env::current_exe()
            .map(|exe| exe.parent().unwrap_or(&exe).join("tools").join(platform_name).join(exe_name))
            .unwrap_or_else(|_| PathBuf::new()),
        
        // 3. Parent directory of executable + tools (alternative production layout)
        std::env::current_exe()
            .map(|exe| exe.parent().and_then(|p| p.parent()).unwrap_or(&exe).join("tools").join(platform_name).join(exe_name))
            .unwrap_or_else(|_| PathBuf::new()),
            
        // 4. Check if we're in src-tauri directory during development
        std::env::current_dir()
            .map(|d| d.parent().unwrap_or(&d).join("tools").join(platform_name).join(exe_name))
            .unwrap_or_else(|_| PathBuf::new()),
    ];
    
    for path in &possible_paths {
        if path.exists() {
            println!("Found tool at: {}", path.display());
            return Ok(path.clone());
        }
    }
    
    // If none found, list all the paths we checked
    let checked_paths: Vec<String> = possible_paths.iter()
        .map(|p| p.display().to_string())
        .collect();
    
    Err(format!("Tool not found: {} (checked: {})", tool_name, checked_paths.join(", ")))
}

async fn execute_conversion(
    tool_name: &str,
    input_path: &PathBuf,
    output_path: &PathBuf,
    advanced_options: Option<String>,
) -> Result<(), String> {
    let tool_path = get_tool_path(tool_name)?;
    
    println!("Tool path: {}", tool_path.display());
    println!("Input: {}", input_path.display());
    println!("Output: {}", output_path.display());
    
    let mut command = Command::new(&tool_path);
    
    match tool_name {
        "ffmpeg" => {
            command.arg("-i").arg(input_path);
            
            // Add advanced options if provided
            if let Some(options) = advanced_options {
                let options_parts: Vec<&str> = options.split_whitespace().collect();
                for part in options_parts {
                    command.arg(part);
                }
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
    
    println!("Executing command: {:?}", command);
    
    let output = command.output()
        .map_err(|e| format!("Failed to execute {}: {}", tool_name, e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!("Conversion failed:\nSTDOUT: {}\nSTDERR: {}", stdout, stderr))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_available_formats,
            convert_file,
            get_file_info,
            test_directories,
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
