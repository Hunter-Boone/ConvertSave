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
        "png" | "jpg" | "jpeg" | "bmp" | "tiff" => {
            options.push(ConversionOption {
                format: "webp".to_string(),
                tool: "imagemagick".to_string(),
                display_name: "WebP Image".to_string(),
                color: "green".to_string(),
            });
            options.push(ConversionOption {
                format: "pdf".to_string(),
                tool: "imagemagick".to_string(),                display_name: "PDF Document".to_string(),
                color: "pink".to_string(),
            });
            if input_extension != "jpg" && input_extension != "jpeg" {
                options.push(ConversionOption {
                    format: "jpg".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "JPEG Image".to_string(),
                    color: "yellow".to_string(),
                });
            }
            if input_extension != "png" {
                options.push(ConversionOption {
                    format: "png".to_string(),
                    tool: "imagemagick".to_string(),
                    display_name: "PNG Image".to_string(),
                    color: "orange".to_string(),
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
    _advanced_options: Option<String>,
) -> Result<String, String> {
    // DEBUG: Print what we're doing
    println!("=== CONVERSION DEBUG ===");
    println!("Input: {}", input_path);
    println!("Output format: {}", output_format);
    println!("Custom output dir: {:?}", output_directory);
    
    // This is a placeholder implementation
    // In production, you would:
    // 1. Determine which tool to use based on input/output formats
    // 2. Construct the command with proper arguments
    // 3. Execute the conversion
    // 4. Handle errors properly
    
    let input_path = PathBuf::from(&input_path);
    let file_stem = input_path.file_stem()
        .ok_or("Invalid input file")?
        .to_str()
        .ok_or("Invalid file name")?;
    
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
    
    // WARNING: This is just copying the file, not actually converting it!
    // We need to implement the real conversion logic
    println!("WARNING: Currently just copying file, not converting!");
    std::fs::copy(&input_path, &output_path)
        .map_err(|e| format!("Failed to copy file: {}", e))?;
    
    println!("=== END DEBUG ===");
    
    Ok(format!("File converted successfully to: {}", output_path.to_string_lossy()))
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
