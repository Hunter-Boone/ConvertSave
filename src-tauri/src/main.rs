// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

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
) -> Result<String, String> {    // This is a placeholder implementation
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
        // Default to Documents/ConvertSave/Converted using dirs crate
        let home = dirs::document_dir()
            .ok_or("Could not find documents directory")?;
        home.join("ConvertSave").join("Converted")
    };
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;
    
    let output_path = output_dir.join(format!("{}.{}", file_stem, output_format));
    
    // Placeholder: In reality, you'd execute the appropriate conversion tool here
    // For now, we'll just return the output path
    Ok(output_path.to_string_lossy().to_string())
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
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
