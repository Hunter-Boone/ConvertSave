//! Integration tests for ConvertSave
//! 
//! These tests verify actual file conversions work correctly.
//! Tests are designed to skip gracefully if required tools are not installed.

use std::path::PathBuf;
use std::fs;
use std::process::Command;

/// Get the test fixtures directory
fn get_fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// Get the test output directory (created if doesn't exist)
fn get_output_dir() -> PathBuf {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("output");
    fs::create_dir_all(&output).ok();
    output
}

/// Check if a tool is available on the system
fn tool_available(tool: &str) -> bool {
    let exe_name = if cfg!(target_os = "windows") {
        match tool {
            "ffmpeg" => "ffmpeg.exe",
            "imagemagick" => "magick.exe",
            "pandoc" => "pandoc.exe",
            _ => return false,
        }
    } else {
        match tool {
            "ffmpeg" => "ffmpeg",
            "imagemagick" => "magick",
            "pandoc" => "pandoc",
            _ => return false,
        }
    };
    
    // Check if tool is in PATH
    let result = Command::new(exe_name)
        .arg("--version")
        .output();
    
    result.is_ok() && result.unwrap().status.success()
}

/// Assert that an output file exists and is not empty
fn assert_output_valid(path: &PathBuf) {
    assert!(path.exists(), "Output file does not exist: {:?}", path);
    let metadata = fs::metadata(path).expect("Failed to get metadata");
    assert!(metadata.len() > 0, "Output file is empty: {:?}", path);
}

/// Clean up a test output file
fn cleanup(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

// ==========================================
// FFmpeg Video Conversion Tests
// ==========================================

mod ffmpeg_video_tests {
    use super::*;

    fn skip_if_no_ffmpeg() -> bool {
        if !tool_available("ffmpeg") {
            eprintln!("Skipping FFmpeg test - ffmpeg not available");
            return true;
        }
        false
    }

    fn run_ffmpeg(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_video_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("video").join("sample.mp4");
        if path.exists() { Some(path) } else { None }
    }

    // Video to Video conversions
    #[test]
    fn test_mp4_to_mov() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_mp4_to_mov.mov");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "1", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_avi() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_mp4_to_avi.avi");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "1", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_mkv() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_mp4_to_mkv.mkv");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "1", "-c", "copy", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_webm() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_mp4_to_webm.webm");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "1", "-c:v", "libvpx-vp9", "-crf", "30", "-b:v", "0", "-c:a", "libopus", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_gif() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_mp4_to_gif.gif");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "1", "-vf", "fps=10,scale=320:-1", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// FFmpeg Audio Extraction Tests
// ==========================================

mod ffmpeg_audio_extraction_tests {
    use super::*;

    fn skip_if_no_ffmpeg() -> bool {
        if !tool_available("ffmpeg") {
            eprintln!("Skipping FFmpeg test - ffmpeg not available");
            return true;
        }
        false
    }

    fn run_ffmpeg(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_video_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("video").join("sample.mp4");
        if path.exists() { Some(path) } else { None }
    }

    #[test]
    fn test_mp4_to_mp3() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_video_to_mp3.mp3");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", "-acodec", "libmp3lame", "-b:a", "192k", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_wav() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_video_to_wav.wav");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_flac() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_video_to_flac.flac");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_ogg() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_video_to_ogg.ogg");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", "-acodec", "libvorbis", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_m4a() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_video_to_m4a.m4a");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", "-acodec", "aac", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp4_to_aac() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_video_fixture() else { return; };
        let output = get_output_dir().join("test_video_to_aac.aac");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", "-acodec", "aac", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// FFmpeg Audio to Audio Tests
// ==========================================

mod ffmpeg_audio_conversion_tests {
    use super::*;

    fn skip_if_no_ffmpeg() -> bool {
        if !tool_available("ffmpeg") {
            eprintln!("Skipping FFmpeg test - ffmpeg not available");
            return true;
        }
        false
    }

    fn run_ffmpeg(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_audio_fixture() -> Option<PathBuf> {
        // Try the audio fixtures directory first
        let audio_path = get_fixtures_dir().join("audio").join("sample.mp3");
        if audio_path.exists() {
            return Some(audio_path);
        }
        
        // Fallback: create from video
        let output_path = get_output_dir().join("test_source_audio.mp3");
        if output_path.exists() {
            return Some(output_path);
        }
        
        let video = get_fixtures_dir().join("video").join("sample.mp4");
        if !video.exists() {
            return None;
        }
        
        let exe = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
        let result = Command::new(exe)
            .args(&["-i", video.to_str().unwrap(), "-y", "-vn", "-t", "2", output_path.to_str().unwrap()])
            .output();
        
        if result.is_ok() && result.unwrap().status.success() {
            Some(output_path)
        } else {
            None
        }
    }

    fn get_wav_fixture() -> Option<PathBuf> {
        let wav_path = get_fixtures_dir().join("audio").join("sample.wav");
        if wav_path.exists() { Some(wav_path) } else { None }
    }

    fn get_flac_fixture() -> Option<PathBuf> {
        let flac_path = get_fixtures_dir().join("audio").join("sample.flac");
        if flac_path.exists() { Some(flac_path) } else { None }
    }

    fn get_ogg_fixture() -> Option<PathBuf> {
        let ogg_path = get_fixtures_dir().join("audio").join("sample.ogg");
        if ogg_path.exists() { Some(ogg_path) } else { None }
    }

    // MP3 conversions
    #[test]
    fn test_mp3_to_wav() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_audio_fixture() else { return; };
        let output = get_output_dir().join("test_mp3_to_wav.wav");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp3_to_flac() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_audio_fixture() else { return; };
        let output = get_output_dir().join("test_mp3_to_flac.flac");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp3_to_ogg() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_audio_fixture() else { return; };
        let output = get_output_dir().join("test_mp3_to_ogg.ogg");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-acodec", "libvorbis", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp3_to_m4a() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_audio_fixture() else { return; };
        let output = get_output_dir().join("test_mp3_to_m4a.m4a");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-acodec", "aac", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mp3_to_aac() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_audio_fixture() else { return; };
        let output = get_output_dir().join("test_mp3_to_aac.aac");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-acodec", "aac", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // WAV conversions
    #[test]
    fn test_wav_to_mp3() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_wav_fixture() else { 
            eprintln!("Skipping - wav fixture not found. Run: npm run test:fixtures");
            return; 
        };
        let output = get_output_dir().join("test_wav_to_mp3.mp3");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-acodec", "libmp3lame", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_wav_to_flac() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_wav_fixture() else { return; };
        let output = get_output_dir().join("test_wav_to_flac.flac");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_wav_to_ogg() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_wav_fixture() else { return; };
        let output = get_output_dir().join("test_wav_to_ogg.ogg");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-acodec", "libvorbis", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // FLAC conversions
    #[test]
    fn test_flac_to_mp3() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_flac_fixture() else { return; };
        let output = get_output_dir().join("test_flac_to_mp3.mp3");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-acodec", "libmp3lame", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_flac_to_wav() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_flac_fixture() else { return; };
        let output = get_output_dir().join("test_flac_to_wav.wav");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // OGG conversions
    #[test]
    fn test_ogg_to_mp3() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_ogg_fixture() else { return; };
        let output = get_output_dir().join("test_ogg_to_mp3.mp3");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-acodec", "libmp3lame", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_ogg_to_wav() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_ogg_fixture() else { return; };
        let output = get_output_dir().join("test_ogg_to_wav.wav");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// ImageMagick Common Image Conversions
// ==========================================

mod imagemagick_common_tests {
    use super::*;

    fn skip_if_no_imagemagick() -> bool {
        if !tool_available("imagemagick") {
            eprintln!("Skipping ImageMagick test - magick not available");
            return true;
        }
        false
    }

    fn run_imagemagick(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "magick.exe" } else { "magick" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_png_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.png");
        if path.exists() { Some(path) } else { None }
    }

    fn get_jpg_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.jpg");
        if path.exists() { Some(path) } else { None }
    }

    // PNG conversions
    #[test]
    fn test_png_to_jpg() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_jpg.jpg");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "90", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_gif() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_gif.gif");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_bmp() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_bmp.bmp");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_tiff() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_tiff.tiff");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_webp() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_webp.webp");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "85", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_ico() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_ico.ico");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-resize", "256x256", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_tga() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_tga.tga");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_ppm() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_ppm.ppm");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // JPG conversions
    #[test]
    fn test_jpg_to_png() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_jpg_fixture() else { return; };
        let output = get_output_dir().join("test_jpg_to_png.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_jpg_to_gif() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_jpg_fixture() else { return; };
        let output = get_output_dir().join("test_jpg_to_gif.gif");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_jpg_to_bmp() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_jpg_fixture() else { return; };
        let output = get_output_dir().join("test_jpg_to_bmp.bmp");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_jpg_to_tiff() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_jpg_fixture() else { return; };
        let output = get_output_dir().join("test_jpg_to_tiff.tiff");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_jpg_to_webp() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_jpg_fixture() else { return; };
        let output = get_output_dir().join("test_jpg_to_webp.webp");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "85", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_jpg_to_ico() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_jpg_fixture() else { return; };
        let output = get_output_dir().join("test_jpg_to_ico.ico");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-resize", "256x256", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// ImageMagick Advanced Format Tests
// ==========================================

mod imagemagick_advanced_tests {
    use super::*;

    fn skip_if_no_imagemagick() -> bool {
        if !tool_available("imagemagick") {
            eprintln!("Skipping ImageMagick test - magick not available");
            return true;
        }
        false
    }

    fn run_imagemagick(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "magick.exe" } else { "magick" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_png_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.png");
        if path.exists() { Some(path) } else { None }
    }

    // X Window System formats
    #[test]
    fn test_png_to_xbm() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_xbm.xbm");
        
        // XBM is monochrome, so we convert to grayscale first
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-colorspace", "gray", "-threshold", "50%", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_xpm() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_xpm.xpm");
        
        // Reduce colors for XPM
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-colors", "256", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // Professional formats
    #[test]
    fn test_png_to_exr() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_exr.exr");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_hdr() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_hdr.hdr");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // JPEG 2000
    #[test]
    fn test_png_to_jp2() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_jp2.jp2");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // Raw formats
    #[test]
    fn test_png_to_pgm() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_pgm.pgm");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-colorspace", "gray", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_pbm() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_pbm.pbm");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-colorspace", "gray", "-threshold", "50%", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // Special formats
    #[test]
    fn test_png_to_pcx() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_pcx.pcx");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_png_to_sun() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_png_to_sun.sun");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// ImageMagick Image Operations Tests
// ==========================================

mod imagemagick_operations_tests {
    use super::*;

    fn skip_if_no_imagemagick() -> bool {
        if !tool_available("imagemagick") {
            eprintln!("Skipping ImageMagick test - magick not available");
            return true;
        }
        false
    }

    fn run_imagemagick(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "magick.exe" } else { "magick" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_png_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.png");
        if path.exists() { Some(path) } else { None }
    }

    #[test]
    fn test_resize_50_percent() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_resize_50.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-resize", "50%", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_resize_specific_dimensions() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_resize_200x200.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-resize", "200x200", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_grayscale_conversion() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_grayscale.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-colorspace", "gray", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_quality_compression() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_quality_50.jpg");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "50", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_rotate() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_rotate_90.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-rotate", "90", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_flip_horizontal() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_png_fixture() else { return; };
        let output = get_output_dir().join("test_flip.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-flop", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// Animated GIF Tests
// ==========================================

mod animated_gif_tests {
    use super::*;

    fn skip_if_no_imagemagick() -> bool {
        if !tool_available("imagemagick") {
            eprintln!("Skipping ImageMagick test - magick not available");
            return true;
        }
        false
    }

    fn run_imagemagick(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "magick.exe" } else { "magick" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_animated_gif_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("animated.gif");
        if path.exists() { Some(path) } else { None }
    }

    #[test]
    fn test_animated_gif_to_webp() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_animated_gif_fixture() else { return; };
        let output = get_output_dir().join("test_animated_to_webp.webp");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_animated_gif_first_frame() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_animated_gif_fixture() else { return; };
        let output = get_output_dir().join("test_animated_first_frame.png");
        
        // Extract first frame
        let input_with_frame = format!("{}[0]", input.to_str().unwrap());
        assert!(run_imagemagick(&[&input_with_frame, output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// Unit Test Import Verification
// ==========================================

mod unit_test_verification {
    use convertsave_lib::conversion;

    #[test]
    fn test_module_is_accessible() {
        let tool = conversion::determine_conversion_tool("mp4", "mp3");
        assert_eq!(tool, Some("ffmpeg"));
    }

    #[test]
    fn test_format_helpers() {
        assert!(conversion::is_video_format("mp4"));
        assert!(conversion::is_audio_format("mp3"));
        assert!(conversion::is_image_format("png"));
    }

    #[test]
    fn test_all_video_formats() {
        for format in conversion::VIDEO_INPUTS {
            assert!(conversion::is_video_format(format));
        }
    }

    #[test]
    fn test_all_audio_formats() {
        for format in conversion::AUDIO_INPUTS {
            assert!(conversion::is_audio_format(format));
        }
    }

    #[test]
    fn test_all_image_formats() {
        for format in conversion::IMAGE_INPUTS {
            assert!(conversion::is_image_format(format));
        }
    }
}

// ==========================================
// Additional Image Format Tests
// ==========================================

mod imagemagick_additional_formats {
    use super::*;

    fn skip_if_no_imagemagick() -> bool {
        if !tool_available("imagemagick") {
            eprintln!("Skipping ImageMagick test - magick not available");
            return true;
        }
        false
    }

    fn run_imagemagick(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "magick.exe" } else { "magick" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_bmp_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.bmp");
        if path.exists() { Some(path) } else { None }
    }

    fn get_tiff_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.tiff");
        if path.exists() { Some(path) } else { None }
    }

    fn get_webp_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.webp");
        if path.exists() { Some(path) } else { None }
    }

    fn get_gif_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.gif");
        if path.exists() { Some(path) } else { None }
    }

    fn get_tga_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("images").join("sample.tga");
        if path.exists() { Some(path) } else { None }
    }

    // BMP conversions
    #[test]
    fn test_bmp_to_png() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_bmp_fixture() else { return; };
        let output = get_output_dir().join("test_bmp_to_png.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_bmp_to_jpg() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_bmp_fixture() else { return; };
        let output = get_output_dir().join("test_bmp_to_jpg.jpg");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "90", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_bmp_to_webp() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_bmp_fixture() else { return; };
        let output = get_output_dir().join("test_bmp_to_webp.webp");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // TIFF conversions
    #[test]
    fn test_tiff_to_png() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_tiff_fixture() else { return; };
        let output = get_output_dir().join("test_tiff_to_png.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_tiff_to_jpg() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_tiff_fixture() else { return; };
        let output = get_output_dir().join("test_tiff_to_jpg.jpg");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "90", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_tiff_to_webp() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_tiff_fixture() else { return; };
        let output = get_output_dir().join("test_tiff_to_webp.webp");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // WebP conversions
    #[test]
    fn test_webp_to_png() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_webp_fixture() else { return; };
        let output = get_output_dir().join("test_webp_to_png.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_webp_to_jpg() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_webp_fixture() else { return; };
        let output = get_output_dir().join("test_webp_to_jpg.jpg");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "90", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_webp_to_gif() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_webp_fixture() else { return; };
        let output = get_output_dir().join("test_webp_to_gif.gif");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // GIF conversions
    #[test]
    fn test_gif_to_png() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_gif_fixture() else { return; };
        let output = get_output_dir().join("test_gif_to_png.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_gif_to_jpg() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_gif_fixture() else { return; };
        let output = get_output_dir().join("test_gif_to_jpg.jpg");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "90", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // TGA conversions
    #[test]
    fn test_tga_to_png() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_tga_fixture() else { return; };
        let output = get_output_dir().join("test_tga_to_png.png");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_tga_to_jpg() {
        if skip_if_no_imagemagick() { return; }
        let Some(input) = get_tga_fixture() else { return; };
        let output = get_output_dir().join("test_tga_to_jpg.jpg");
        
        assert!(run_imagemagick(&[input.to_str().unwrap(), "-quality", "90", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// Video Format Conversion Tests
// ==========================================

mod ffmpeg_video_format_tests {
    use super::*;

    fn skip_if_no_ffmpeg() -> bool {
        if !tool_available("ffmpeg") {
            eprintln!("Skipping FFmpeg test - ffmpeg not available");
            return true;
        }
        false
    }

    fn run_ffmpeg(args: &[&str]) -> bool {
        let exe = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
        let result = Command::new(exe).args(args).output();
        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn get_mov_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("video").join("sample.mov");
        if path.exists() { Some(path) } else { None }
    }

    fn get_avi_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("video").join("sample.avi");
        if path.exists() { Some(path) } else { None }
    }

    fn get_mkv_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("video").join("sample.mkv");
        if path.exists() { Some(path) } else { None }
    }

    fn get_webm_fixture() -> Option<PathBuf> {
        let path = get_fixtures_dir().join("video").join("sample.webm");
        if path.exists() { Some(path) } else { None }
    }

    // MOV conversions
    #[test]
    fn test_mov_to_mp4() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_mov_fixture() else { return; };
        let output = get_output_dir().join("test_mov_to_mp4.mp4");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "2", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mov_to_avi() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_mov_fixture() else { return; };
        let output = get_output_dir().join("test_mov_to_avi.avi");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "2", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mov_to_mp3() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_mov_fixture() else { return; };
        let output = get_output_dir().join("test_mov_to_mp3.mp3");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", "-acodec", "libmp3lame", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // AVI conversions
    #[test]
    fn test_avi_to_mp4() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_avi_fixture() else { return; };
        let output = get_output_dir().join("test_avi_to_mp4.mp4");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "2", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_avi_to_mov() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_avi_fixture() else { return; };
        let output = get_output_dir().join("test_avi_to_mov.mov");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "2", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_avi_to_mp3() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_avi_fixture() else { return; };
        let output = get_output_dir().join("test_avi_to_mp3.mp3");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", "-acodec", "libmp3lame", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // MKV conversions
    #[test]
    fn test_mkv_to_mp4() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_mkv_fixture() else { return; };
        let output = get_output_dir().join("test_mkv_to_mp4.mp4");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "2", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_mkv_to_webm() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_mkv_fixture() else { return; };
        let output = get_output_dir().join("test_mkv_to_webm.webm");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "2", "-c:v", "libvpx-vp9", "-crf", "30", "-b:v", "0", "-c:a", "libopus", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    // WebM conversions
    #[test]
    fn test_webm_to_mp4() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_webm_fixture() else { return; };
        let output = get_output_dir().join("test_webm_to_mp4.mp4");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-t", "2", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }

    #[test]
    fn test_webm_to_mp3() {
        if skip_if_no_ffmpeg() { return; }
        let Some(input) = get_webm_fixture() else { return; };
        let output = get_output_dir().join("test_webm_to_mp3.mp3");
        
        assert!(run_ffmpeg(&["-i", input.to_str().unwrap(), "-y", "-vn", "-acodec", "libmp3lame", output.to_str().unwrap()]));
        assert_output_valid(&output);
        cleanup(&output);
    }
}

// ==========================================
// Fixture Verification Tests
// ==========================================

mod fixture_tests {
    use super::*;

    #[test]
    fn test_fixtures_directory_exists() {
        let fixtures = get_fixtures_dir();
        assert!(fixtures.exists(), "Fixtures directory should exist");
    }

    #[test]
    fn test_images_fixtures_exist() {
        let images = get_fixtures_dir().join("images");
        assert!(images.exists(), "Images fixtures directory should exist");
    }

    #[test]
    fn test_video_fixtures_exist() {
        let video = get_fixtures_dir().join("video");
        assert!(video.exists(), "Video fixtures directory should exist");
    }

    #[test]
    fn test_audio_fixtures_directory() {
        let audio = get_fixtures_dir().join("audio");
        if !audio.exists() {
            eprintln!("Note: Audio fixtures directory not found. Run: npm run test:fixtures");
        }
    }

    #[test]
    fn test_documents_fixtures_directory() {
        let docs = get_fixtures_dir().join("documents");
        assert!(docs.exists(), "Documents fixtures directory should exist");
    }

    #[test]
    fn test_output_dir_creation() {
        let output = get_output_dir();
        assert!(output.exists(), "Output directory should be created");
        assert!(output.is_dir(), "Output should be a directory");
    }

    #[test]
    fn test_sample_png_exists() {
        let sample = get_fixtures_dir().join("images").join("sample.png");
        if sample.exists() {
            let metadata = fs::metadata(&sample).unwrap();
            assert!(metadata.len() > 0, "sample.png should not be empty");
        }
    }

    #[test]
    fn test_sample_jpg_exists() {
        let sample = get_fixtures_dir().join("images").join("sample.jpg");
        if sample.exists() {
            let metadata = fs::metadata(&sample).unwrap();
            assert!(metadata.len() > 0, "sample.jpg should not be empty");
        }
    }

    #[test]
    fn test_sample_mp4_exists() {
        let sample = get_fixtures_dir().join("video").join("sample.mp4");
        if sample.exists() {
            let metadata = fs::metadata(&sample).unwrap();
            assert!(metadata.len() > 0, "sample.mp4 should not be empty");
        }
    }

    #[test]
    fn test_sample_mp3_exists() {
        let sample = get_fixtures_dir().join("audio").join("sample.mp3");
        if sample.exists() {
            let metadata = fs::metadata(&sample).unwrap();
            assert!(metadata.len() > 0, "sample.mp3 should not be empty");
        } else {
            eprintln!("Note: sample.mp3 not found. Run: npm run test:fixtures");
        }
    }

    #[test]
    fn test_count_image_fixtures() {
        let images = get_fixtures_dir().join("images");
        if images.exists() {
            let count = fs::read_dir(&images)
                .map(|entries| entries.filter_map(|e| e.ok()).count())
                .unwrap_or(0);
            println!("Image fixtures found: {}", count);
        }
    }

    #[test]
    fn test_count_video_fixtures() {
        let video = get_fixtures_dir().join("video");
        if video.exists() {
            let count = fs::read_dir(&video)
                .map(|entries| entries.filter_map(|e| e.ok()).count())
                .unwrap_or(0);
            println!("Video fixtures found: {}", count);
        }
    }

    #[test]
    fn test_count_audio_fixtures() {
        let audio = get_fixtures_dir().join("audio");
        if audio.exists() {
            let count = fs::read_dir(&audio)
                .map(|entries| entries.filter_map(|e| e.ok()).count())
                .unwrap_or(0);
            println!("Audio fixtures found: {}", count);
        }
    }
}
