use std::path::PathBuf;
use std::fs;
use dirs;

// Helper function to get the tool path (same logic as main.rs)
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
    // NOTE: This must match the path used in the main application
    if let Some(data_dir) = dirs::data_dir() {
        // Tauri's app_data_dir() uses: {data_dir}/{identifier}
        // Our identifier from tauri.conf.json is "com.convertsave"
        let app_data_path = data_dir
            .join("com.convertsave")
            .join(tool_name)
            .join(exe_name);
        possible_paths.push(app_data_path);
    }
    
    // 2. Project root tools directory (development)
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
    
    // Debug: print all paths we're checking
    println!("=== CHECKING TOOL PATHS for {} ===", tool_name);
    for path in &possible_paths {
        println!("  Checking: {:?} (exists: {})", path, path.exists());
    }
    
    // Find the first path that exists
    for path in &possible_paths {
        if path.exists() {
            println!("  ✓ Using: {:?}", path);
            return Ok(path.clone());
        }
    }
    
    // If none found, list all the paths we checked
    let checked_paths: Vec<String> = possible_paths.iter()
        .map(|p| p.display().to_string())
        .collect();
    
    Err(format!("Tool not found: {} (checked: {})", tool_name, checked_paths.join(", ")))
}

// Helper function to get test fixtures path
fn get_fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

// Helper function to get output directory
fn get_output_dir() -> PathBuf {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("output");
    fs::create_dir_all(&output).unwrap();
    output
}

// Helper to check if output file exists and is not empty
fn assert_output_exists(output_path: &PathBuf) {
    assert!(output_path.exists(), "Output file does not exist: {:?}", output_path);
    let metadata = fs::metadata(output_path).unwrap();
    assert!(metadata.len() > 0, "Output file is empty: {:?}", output_path);
}

// Helper to perform conversion using the conversion logic from main
// This wraps the internal conversion functions for testing
async fn perform_conversion(
    input_path: &PathBuf,
    output_format: &str,
    output_path: &PathBuf,
) -> Result<(), String> {
    // Get the parent directory of output path
    let output_dir = output_path.parent()
        .ok_or("Could not determine output directory")?;
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;
    
    // For now, we'll use a simple command execution approach
    // In a real implementation, you would call the conversion functions directly
    // or extract them into a library that can be used by both the app and tests
    
    let input_extension = input_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // Determine the tool and execute conversion
    // This is a simplified version - you may want to import the actual functions
    let tool = match (&input_extension[..], output_format) {
        // ICO format - use ImageMagick (needs resize handling)
        (ext, "ico") if matches!(ext, "png" | "jpg" | "jpeg" | "bmp" | "tiff" | "webp" | "gif" | "heic" | "heif" | "avif") => "imagemagick",
        
        // Images - use FFmpeg
        (ext, "jpg") | (ext, "png") | (ext, "webp") | (ext, "gif") | 
        (ext, "bmp") | (ext, "tiff") | (ext, "avif") |
        (ext, "tga") | (ext, "j2k") | (ext, "exr") | (ext, "hdr")
            if matches!(ext, "png" | "jpg" | "jpeg" | "bmp" | "tiff" | "webp" | "gif" | "heic" | "heif" | "avif") => "ffmpeg",
        
        // HEIC encoding - use ImageMagick
        (ext, "heic") if matches!(ext, "png" | "jpg" | "jpeg" | "bmp" | "tiff" | "webp" | "gif") => "imagemagick",
        
        // Video/Audio - use FFmpeg
        (ext, "mp3") | (ext, "webm") | (ext, "mp4") 
            if matches!(ext, "mp4" | "mov" | "avi" | "mkv") => "ffmpeg",
        
        // Documents - use Pandoc
        (ext, "html") | (ext, "docx") | (ext, "epub") | (ext, "txt") | (ext, "md")
            if matches!(ext, "md" | "markdown" | "html" | "htm" | "txt" | "docx" | "doc" | "odt") => "pandoc",
        
        _ => return Err(format!("No conversion available for {} to {}", input_extension, output_format)),
    };
    
    // Execute the conversion based on the tool
    match tool {
        "ffmpeg" => execute_ffmpeg_conversion(input_path, output_path, output_format).await,
        "imagemagick" => execute_imagemagick_conversion(input_path, output_path, output_format).await,
        "pandoc" => execute_pandoc_conversion(input_path, output_path, output_format).await,
        _ => Err(format!("Unknown tool: {}", tool)),
    }
}

async fn execute_ffmpeg_conversion(input: &PathBuf, output: &PathBuf, format: &str) -> Result<(), String> {
    let ffmpeg_path = get_tool_path("ffmpeg")?;
    let mut cmd = std::process::Command::new(&ffmpeg_path);
    cmd.arg("-i").arg(input);
    cmd.arg("-y"); // Overwrite output files
    
    // Add format-specific arguments
    match format {
        "ico" => {
            // ICO format has a maximum size of 256x256, so we need to scale down
            cmd.arg("-vf").arg("scale='min(256,iw)':'min(256,ih)':force_original_aspect_ratio=decrease");
        }
        "jpg" | "jpeg" => {
            cmd.arg("-q:v").arg("2"); // High quality
        }
        "webp" => {
            cmd.arg("-quality").arg("90");
        }
        "avif" => {
            cmd.arg("-crf").arg("23");
        }
        "mp3" => {
            cmd.arg("-vn"); // No video
            cmd.arg("-acodec").arg("libmp3lame");
            cmd.arg("-b:a").arg("192k");
        }
        "webm" => {
            cmd.arg("-c:v").arg("libvpx-vp9");
            cmd.arg("-crf").arg("30");
            cmd.arg("-b:v").arg("0");
        }
        _ => {}
    }
    
    cmd.arg(output);
    
    let output_result = cmd.output()
        .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
    
    if !output_result.status.success() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        return Err(format!("FFmpeg conversion failed: {}", stderr));
    }
    
    Ok(())
}

async fn execute_imagemagick_conversion(input: &PathBuf, output: &PathBuf, format: &str) -> Result<(), String> {
    let imagemagick_path = get_tool_path("imagemagick")?;
    
    let mut cmd = std::process::Command::new(&imagemagick_path);
    cmd.arg(input);
    
    // Special handling for ICO format - must resize to fit icon size limits
    if format == "ico" {
        cmd.arg("-resize").arg("256x256");
        cmd.arg("-gravity").arg("center");
        cmd.arg("-extent").arg("256x256");
        cmd.arg("-background").arg("transparent");
    }
    
    cmd.arg(output);
    
    let output_result = cmd.output()
        .map_err(|e| format!("Failed to execute ImageMagick: {}", e))?;
    
    if !output_result.status.success() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        return Err(format!("ImageMagick conversion failed: {}", stderr));
    }
    
    Ok(())
}

async fn execute_pandoc_conversion(input: &PathBuf, output: &PathBuf, format: &str) -> Result<(), String> {
    let pandoc_path = get_tool_path("pandoc")?;
    let mut cmd = std::process::Command::new(&pandoc_path);
    cmd.arg(input);
    cmd.arg("-o").arg(output);
    
    // Add format-specific arguments
    match format {
        "docx" => {
            cmd.arg("--standalone");
        }
        "epub" => {
            cmd.arg("--standalone");
        }
        "html" => {
            cmd.arg("--standalone");
        }
        _ => {}
    }
    
    let output_result = cmd.output()
        .map_err(|e| format!("Failed to execute pandoc: {}", e))?;
    
    if !output_result.status.success() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        return Err(format!("Pandoc conversion failed: {}", stderr));
    }
    
    Ok(())
}

// Helper macro to create a test
macro_rules! conversion_test {
    ($test_name:ident, $input_file:expr, $output_file:expr, $format:expr) => {
        #[test]
        #[ignore]
        fn $test_name() {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let input = get_fixtures_dir().join($input_file);
                let output = get_output_dir().join($output_file);
                
                perform_conversion(&input, $format, &output).await.expect("Conversion failed");
                assert_output_exists(&output);
                
                // Cleanup
                fs::remove_file(output).ok();
            });
        }
    };
}

#[cfg(test)]
mod image_conversions {
    use super::*;

    conversion_test!(test_png_to_jpg, "images/sample.png", "png_to_jpg.jpg", "jpg");
    conversion_test!(test_jpg_to_png, "images/sample.jpg", "jpg_to_png.png", "png");
    conversion_test!(test_png_to_webp, "images/sample.png", "png_to_webp.webp", "webp");
    conversion_test!(test_jpg_to_webp, "images/sample.jpg", "jpg_to_webp.webp", "webp");
    conversion_test!(test_png_to_gif, "images/sample.png", "png_to_gif.gif", "gif");
    conversion_test!(test_png_to_bmp, "images/sample.png", "png_to_bmp.bmp", "bmp");
    conversion_test!(test_png_to_tiff, "images/sample.png", "png_to_tiff.tiff", "tiff");
    conversion_test!(test_png_to_avif, "images/sample.png", "png_to_avif.avif", "avif");
    conversion_test!(test_png_to_ico, "images/sample.png", "png_to_ico.ico", "ico");
    conversion_test!(test_jpg_to_tga, "images/sample.jpg", "jpg_to_tga.tga", "tga");
    conversion_test!(test_png_to_j2k, "images/sample.png", "png_to_j2k.j2k", "j2k");
}

#[cfg(test)]
mod document_conversions {
    use super::*;

    conversion_test!(test_md_to_html, "documents/sample.md", "md_to_html.html", "html");
    conversion_test!(test_md_to_docx, "documents/sample.md", "md_to_docx.docx", "docx");
    conversion_test!(test_md_to_epub, "documents/sample.md", "md_to_epub.epub", "epub");
    conversion_test!(test_md_to_txt, "documents/sample.md", "md_to_txt.txt", "txt");
    conversion_test!(test_html_to_md, "documents/sample.html", "html_to_md.md", "md");
    conversion_test!(test_html_to_docx, "documents/sample.html", "html_to_docx.docx", "docx");
    conversion_test!(test_txt_to_html, "documents/sample.txt", "txt_to_html.html", "html");
    conversion_test!(test_txt_to_md, "documents/sample.txt", "txt_to_md.md", "md");
}

#[cfg(test)]
mod video_audio_conversions {
    use super::*;

    conversion_test!(test_mp4_to_mp3, "video/sample.mp4", "mp4_to_mp3.mp3", "mp3");
    conversion_test!(test_mp4_to_webm, "video/sample.mp4", "mp4_to_webm.webm", "webm");
    conversion_test!(test_avi_to_mp4, "video/sample.avi", "avi_to_mp4.mp4", "mp4");
    conversion_test!(test_mov_to_mp3, "video/sample.mov", "mov_to_mp3.mp3", "mp3");
}

#[cfg(test)]
mod batch_conversions {
    use super::*;

    #[test]
    #[ignore]
    fn test_batch_png_to_jpg() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let fixtures = get_fixtures_dir().join("images");
            let output_dir = get_output_dir().join("batch_test");
            fs::create_dir_all(&output_dir).unwrap();
            
            // Test converting multiple PNG files to JPG
            let test_files = vec!["sample.png", "sample2.png", "sample3.png"];
            
            for file in test_files {
                let input = fixtures.join(file);
                if input.exists() {
                    let output = output_dir.join(file.replace(".png", ".jpg"));
                    perform_conversion(&input, "jpg", &output).await.expect("Batch conversion failed");
                    assert_output_exists(&output);
                }
            }
            
            // Cleanup
            fs::remove_dir_all(output_dir).ok();
        });
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    #[ignore]
    fn test_large_image_conversion() {
        // Test converting a very large image (e.g., 4K or 8K)
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let input = get_fixtures_dir().join("images").join("large_4k.png");
            let output = get_output_dir().join("large_4k.jpg");
            
            if input.exists() {
                perform_conversion(&input, "jpg", &output).await.expect("Large image conversion failed");
                assert_output_exists(&output);
                fs::remove_file(output).ok();
            }
        });
    }

    #[test]
    #[ignore]
    fn test_small_image_conversion() {
        // Test converting a very small image (e.g., 1x1 pixel)
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let input = get_fixtures_dir().join("images").join("tiny.png");
            let output = get_output_dir().join("tiny.jpg");
            
            if input.exists() {
                perform_conversion(&input, "jpg", &output).await.expect("Small image conversion failed");
                assert_output_exists(&output);
                fs::remove_file(output).ok();
            }
        });
    }

    #[test]
    #[ignore]
    fn test_transparent_png_to_jpg() {
        // Test handling transparency when converting to format that doesn't support it
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let input = get_fixtures_dir().join("images").join("transparent.png");
            let output = get_output_dir().join("transparent.jpg");
            
            if input.exists() {
                perform_conversion(&input, "jpg", &output).await.expect("Transparent PNG conversion failed");
                assert_output_exists(&output);
                fs::remove_file(output).ok();
            }
        });
    }

    #[test]
    #[ignore]
    fn test_animated_gif_conversion() {
        // Test converting animated GIF
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let input = get_fixtures_dir().join("images").join("animated.gif");
            let output = get_output_dir().join("animated.webp");
            
            if input.exists() {
                perform_conversion(&input, "webp", &output).await.expect("Animated GIF conversion failed");
                assert_output_exists(&output);
                fs::remove_file(output).ok();
            }
        });
    }

    #[test]
    #[ignore]
    fn test_heic_with_rotation() {
        // Test HEIC file with EXIF rotation data
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let input = get_fixtures_dir().join("images").join("rotated.heic");
            let output = get_output_dir().join("rotated.jpg");
            
            if input.exists() {
                perform_conversion(&input, "jpg", &output).await.expect("HEIC conversion failed");
                assert_output_exists(&output);
                fs::remove_file(output).ok();
            }
        });
    }
}

