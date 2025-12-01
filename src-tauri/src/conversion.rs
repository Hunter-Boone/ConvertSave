//! Conversion logic module - Contains testable conversion functions
//! 
//! This module extracts the core conversion logic from main.rs to make it testable.

use serde::{Deserialize, Serialize};

/// Represents a conversion option that can be presented to the user
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConversionOption {
    pub format: String,
    pub tool: String,
    pub display_name: String,
    pub color: String,
}

/// Feature flag for Pandoc support
pub const ENABLE_PANDOC: bool = false;

/// Supported video input formats
pub const VIDEO_INPUTS: &[&str] = &[
    "mp4", "mov", "avi", "mkv", "webm", "flv", "wmv", "m4v", "mpg", "mpeg", "3gp"
];

/// Supported audio input formats
pub const AUDIO_INPUTS: &[&str] = &[
    "mp3", "wav", "flac", "ogg", "m4a", "wma", "aac"
];

/// Supported image input formats
pub const IMAGE_INPUTS: &[&str] = &[
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
    "ppm", "pgm", "pbm", "pam", "pnm",
    // X Window System formats
    "xbm", "xpm", "xwd",
    // Gaming/3D formats
    "dds", "vtf",
    // Vector/Document formats (rasterized)
    "svg", "svgz", "ai", "eps", "ps", "pdf",
    // Digital camera RAW formats
    "arw", "cr2", "cr3", "crw", "dng", "nef", "nrw", "orf", "raf", "raw", "rw2", "rwl", "srw",
    // Animation formats
    "mng", "apng",
    // Windows formats
    "cur", "dib", "emf", "wmf",
    // Other formats
    "fits", "flif", "jbig", "jng", "miff", "otb", "pal", "palm", "pcd",
    "pix", "plasma", "pwp", "rgf", "sfw", "uyvy", "vicar",
    "viff", "wbmp", "xcf", "xv", "yuv"
];

/// Supported image outputs for FFmpeg
pub const IMAGE_OUTPUTS_FFMPEG: &[&str] = &[
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
];

/// Supported image outputs for ImageMagick
pub const IMAGE_OUTPUTS_IMAGEMAGICK: &[&str] = &[
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
    "dds", "vtf",
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

/// Audio/Video output formats
pub const AV_OUTPUTS: &[&str] = &[
    "mp4", "mov", "avi", "mkv", "webm", "mp3", "wav", "flac", "ogg", "m4a", "aac", "gif"
];

/// Document input formats (for Pandoc)
pub const DOC_INPUTS: &[&str] = &[
    "md", "markdown", "txt", "html", "htm", "docx", "odt", "rtf", "tex", "latex", "epub", "rst"
];

/// Document output formats (for Pandoc)
pub const DOC_OUTPUTS: &[&str] = &[
    "md", "html", "docx", "odt", "rtf", "tex", "epub", "txt"
];

/// Office input formats (for LibreOffice)
pub const OFFICE_INPUTS: &[&str] = &[
    "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf"
];

/// Office output formats (for LibreOffice)
pub const OFFICE_OUTPUTS: &[&str] = &[
    "pdf", "html", "txt", "docx", "odt", "rtf"
];

/// Determines which conversion tool should be used for a given input/output format pair.
/// 
/// # Arguments
/// * `input_ext` - The input file extension (lowercase, without dot)
/// * `output_ext` - The desired output format (lowercase, without dot)
/// 
/// # Returns
/// * `Some(&'static str)` - The name of the tool to use ("ffmpeg", "imagemagick", "pandoc", "libreoffice", "rename")
/// * `None` - If no conversion is available for this format pair
/// 
/// # Examples
/// ```
/// use convertsave_lib::conversion::determine_conversion_tool;
/// 
/// assert_eq!(determine_conversion_tool("mp4", "mp3"), Some("ffmpeg"));
/// assert_eq!(determine_conversion_tool("png", "jpg"), Some("imagemagick"));
/// assert_eq!(determine_conversion_tool("jpg", "jpeg"), Some("rename"));
/// ```
pub fn determine_conversion_tool(input_ext: &str, output_ext: &str) -> Option<&'static str> {
    // JPG <-> JPEG simple rename (no conversion needed, same format)
    if (input_ext == "jpg" && output_ext == "jpeg") || (input_ext == "jpeg" && output_ext == "jpg") {
        return Some("rename");
    }
    
    // Use ffmpeg for media and image conversions
    if (VIDEO_INPUTS.contains(&input_ext) || AUDIO_INPUTS.contains(&input_ext)) 
        && AV_OUTPUTS.contains(&output_ext) {
        return Some("ffmpeg");
    }
    
    // HEIC/HEIF encoding requires ImageMagick
    if IMAGE_INPUTS.contains(&input_ext) && (output_ext == "heic" || output_ext == "heif") {
        return Some("imagemagick");
    }
    
    // X Window System formats require ImageMagick
    if IMAGE_INPUTS.contains(&input_ext) 
        && (output_ext == "xbm" || output_ext == "xpm" || output_ext == "xwd") {
        return Some("imagemagick");
    }
    
    // Try ImageMagick first for image conversions
    if IMAGE_INPUTS.contains(&input_ext) && IMAGE_OUTPUTS_IMAGEMAGICK.contains(&output_ext) {
        return Some("imagemagick");
    }
    
    // Fallback to ffmpeg for formats ImageMagick doesn't support well
    if IMAGE_INPUTS.contains(&input_ext) && IMAGE_OUTPUTS_FFMPEG.contains(&output_ext) {
        return Some("ffmpeg");
    }
    
    // Document conversions via Pandoc (when enabled)
    if ENABLE_PANDOC && DOC_INPUTS.contains(&input_ext) && DOC_OUTPUTS.contains(&output_ext) {
        return Some("pandoc");
    }
    
    // Office conversions via LibreOffice
    if OFFICE_INPUTS.contains(&input_ext) && OFFICE_OUTPUTS.contains(&output_ext) {
        return Some("libreoffice");
    }
    
    None
}

/// Checks if an extension is a valid video format
pub fn is_video_format(ext: &str) -> bool {
    VIDEO_INPUTS.contains(&ext.to_lowercase().as_str())
}

/// Checks if an extension is a valid audio format
pub fn is_audio_format(ext: &str) -> bool {
    AUDIO_INPUTS.contains(&ext.to_lowercase().as_str())
}

/// Checks if an extension is a valid image format
pub fn is_image_format(ext: &str) -> bool {
    IMAGE_INPUTS.contains(&ext.to_lowercase().as_str())
}

/// Checks if an extension is a valid document format
pub fn is_document_format(ext: &str) -> bool {
    DOC_INPUTS.contains(&ext.to_lowercase().as_str()) 
        || OFFICE_INPUTS.contains(&ext.to_lowercase().as_str())
}

/// Normalizes a file extension by removing the leading dot and converting to lowercase
pub fn normalize_extension(ext: &str) -> String {
    ext.trim_start_matches('.').to_lowercase()
}

/// Returns the display name for a given format
pub fn get_format_display_name(format: &str) -> &'static str {
    match format {
        // Video
        "mp4" => "MP4 Video",
        "mov" => "QuickTime Video",
        "avi" => "AVI Video",
        "mkv" => "Matroska Video",
        "webm" => "WebM Video",
        "flv" => "Flash Video",
        "wmv" => "Windows Media Video",
        "m4v" => "M4V Video",
        "gif" => "Animated GIF",
        // Audio
        "mp3" => "MP3 Audio",
        "wav" => "WAV Audio",
        "flac" => "FLAC Audio (Lossless)",
        "ogg" => "OGG Audio",
        "m4a" => "M4A Audio",
        "aac" => "AAC Audio",
        "wma" => "Windows Media Audio",
        // Images
        "jpg" | "jpeg" => "JPEG Image",
        "png" => "PNG Image",
        "bmp" => "BMP Image",
        "tiff" | "tif" => "TIFF Image",
        "webp" => "WebP Image",
        "heic" | "heif" => "HEIC Image",
        "avif" => "AVIF Image",
        "ico" => "Icon",
        "svg" => "SVG Vector",
        "psd" => "Photoshop Document",
        // Documents
        "pdf" => "PDF Document",
        "docx" => "Word Document",
        "doc" => "Word Document (Legacy)",
        "txt" => "Plain Text",
        "html" => "HTML Document",
        "md" => "Markdown",
        "epub" => "E-Book",
        "rtf" => "Rich Text",
        "odt" => "OpenDocument Text",
        _ => "Unknown Format",
    }
}

/// Returns the color category for a given format (for UI styling)
pub fn get_format_color(format: &str) -> &'static str {
    match format {
        // Video - blue
        "mp4" | "mov" | "avi" | "mkv" | "m4v" | "flv" | "wmv" => "blue",
        // Web video - green
        "webm" => "green",
        // Audio - green variants
        "mp3" | "wav" => "green",
        "flac" => "aquamarine",
        "ogg" => "orange",
        "m4a" => "light-purple",
        "aac" => "yellow",
        // Images - varies
        "jpg" | "jpeg" | "png" | "bmp" => "light-tan",
        "gif" => "pink",
        "webp" | "avif" | "heic" | "heif" => "green",
        "ico" => "blue",
        "svg" => "orange",
        "psd" => "blue",
        "tiff" | "tif" => "lavender",
        // Documents - varies
        "pdf" => "pink",
        "docx" | "doc" => "blue",
        "html" => "orange",
        "txt" => "lavender",
        "md" => "light-tan",
        "epub" => "pink",
        _ => "gray",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // VIDEO CONVERSION TESTS
    // ==========================================

    mod video_conversions {
        use super::*;

        #[test]
        fn test_all_video_to_video_conversions() {
            let video_formats = ["mp4", "mov", "avi", "mkv", "webm"];
            
            for input in &video_formats {
                for output in &video_formats {
                    let result = determine_conversion_tool(input, output);
                    assert_eq!(
                        result, Some("ffmpeg"),
                        "Video conversion {} -> {} should use ffmpeg",
                        input, output
                    );
                }
            }
        }

        #[test]
        fn test_all_video_inputs_recognized() {
            for format in VIDEO_INPUTS {
                assert!(
                    is_video_format(format),
                    "Video format {} should be recognized",
                    format
                );
            }
        }

        #[test]
        fn test_video_to_mp4() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "mp4"), Some("ffmpeg"),
                    "{} -> mp4 should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_mov() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "mov"), Some("ffmpeg"),
                    "{} -> mov should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_avi() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "avi"), Some("ffmpeg"),
                    "{} -> avi should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_mkv() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "mkv"), Some("ffmpeg"),
                    "{} -> mkv should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_webm() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "webm"), Some("ffmpeg"),
                    "{} -> webm should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_gif() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "gif"), Some("ffmpeg"),
                    "{} -> gif should use ffmpeg", input
                );
            }
        }
    }

    // ==========================================
    // VIDEO TO AUDIO EXTRACTION TESTS
    // ==========================================

    mod video_to_audio_conversions {
        use super::*;

        #[test]
        fn test_video_to_mp3() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "mp3"), Some("ffmpeg"),
                    "{} -> mp3 should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_wav() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "wav"), Some("ffmpeg"),
                    "{} -> wav should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_flac() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "flac"), Some("ffmpeg"),
                    "{} -> flac should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_ogg() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "ogg"), Some("ffmpeg"),
                    "{} -> ogg should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_m4a() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "m4a"), Some("ffmpeg"),
                    "{} -> m4a should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_video_to_aac() {
            for input in VIDEO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "aac"), Some("ffmpeg"),
                    "{} -> aac should use ffmpeg", input
                );
            }
        }
    }

    // ==========================================
    // AUDIO CONVERSION TESTS
    // ==========================================

    mod audio_conversions {
        use super::*;

        #[test]
        fn test_all_audio_inputs_recognized() {
            for format in AUDIO_INPUTS {
                assert!(
                    is_audio_format(format),
                    "Audio format {} should be recognized",
                    format
                );
            }
        }

        #[test]
        fn test_all_audio_to_audio_conversions() {
            let audio_outputs = ["mp3", "wav", "flac", "ogg", "m4a", "aac"];
            
            for input in AUDIO_INPUTS {
                for output in &audio_outputs {
                    let result = determine_conversion_tool(input, output);
                    assert_eq!(
                        result, Some("ffmpeg"),
                        "Audio conversion {} -> {} should use ffmpeg",
                        input, output
                    );
                }
            }
        }

        #[test]
        fn test_audio_to_mp3() {
            for input in AUDIO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "mp3"), Some("ffmpeg"),
                    "{} -> mp3 should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_audio_to_wav() {
            for input in AUDIO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "wav"), Some("ffmpeg"),
                    "{} -> wav should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_audio_to_flac() {
            for input in AUDIO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "flac"), Some("ffmpeg"),
                    "{} -> flac should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_audio_to_ogg() {
            for input in AUDIO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "ogg"), Some("ffmpeg"),
                    "{} -> ogg should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_audio_to_m4a() {
            for input in AUDIO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "m4a"), Some("ffmpeg"),
                    "{} -> m4a should use ffmpeg", input
                );
            }
        }

        #[test]
        fn test_audio_to_aac() {
            for input in AUDIO_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "aac"), Some("ffmpeg"),
                    "{} -> aac should use ffmpeg", input
                );
            }
        }
    }

    // ==========================================
    // IMAGE CONVERSION TESTS
    // ==========================================

    mod image_conversions {
        use super::*;

        // Common image formats that should always work
        const COMMON_IMAGE_INPUTS: &[&str] = &[
            "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"
        ];

        const COMMON_IMAGE_OUTPUTS: &[&str] = &[
            "jpg", "png", "gif", "bmp", "tiff", "webp"
        ];

        #[test]
        fn test_common_image_inputs_recognized() {
            for format in COMMON_IMAGE_INPUTS {
                assert!(
                    is_image_format(format),
                    "Image format {} should be recognized",
                    format
                );
            }
        }

        #[test]
        fn test_all_image_inputs_recognized() {
            for format in IMAGE_INPUTS {
                assert!(
                    is_image_format(format),
                    "Image format {} should be recognized",
                    format
                );
            }
        }

        #[test]
        fn test_common_image_conversions() {
            for input in COMMON_IMAGE_INPUTS {
                for output in COMMON_IMAGE_OUTPUTS {
                    let result = determine_conversion_tool(input, output);
                    assert!(
                        result.is_some(),
                        "Image conversion {} -> {} should have a tool",
                        input, output
                    );
                }
            }
        }

        #[test]
        fn test_image_to_jpg() {
            for input in COMMON_IMAGE_INPUTS {
                if *input != "jpg" && *input != "jpeg" {
                    assert_eq!(
                        determine_conversion_tool(input, "jpg"), Some("imagemagick"),
                        "{} -> jpg should use imagemagick", input
                    );
                }
            }
        }

        #[test]
        fn test_image_to_png() {
            for input in COMMON_IMAGE_INPUTS {
                if *input != "png" {
                    assert_eq!(
                        determine_conversion_tool(input, "png"), Some("imagemagick"),
                        "{} -> png should use imagemagick", input
                    );
                }
            }
        }

        #[test]
        fn test_image_to_webp() {
            for input in COMMON_IMAGE_INPUTS {
                if *input != "webp" {
                    assert_eq!(
                        determine_conversion_tool(input, "webp"), Some("imagemagick"),
                        "{} -> webp should use imagemagick", input
                    );
                }
            }
        }

        #[test]
        fn test_image_to_gif() {
            for input in COMMON_IMAGE_INPUTS {
                if *input != "gif" {
                    assert_eq!(
                        determine_conversion_tool(input, "gif"), Some("imagemagick"),
                        "{} -> gif should use imagemagick", input
                    );
                }
            }
        }

        #[test]
        fn test_image_to_bmp() {
            for input in COMMON_IMAGE_INPUTS {
                if *input != "bmp" {
                    assert_eq!(
                        determine_conversion_tool(input, "bmp"), Some("imagemagick"),
                        "{} -> bmp should use imagemagick", input
                    );
                }
            }
        }

        #[test]
        fn test_image_to_tiff() {
            for input in COMMON_IMAGE_INPUTS {
                if *input != "tiff" {
                    assert_eq!(
                        determine_conversion_tool(input, "tiff"), Some("imagemagick"),
                        "{} -> tiff should use imagemagick", input
                    );
                }
            }
        }

        #[test]
        fn test_image_to_ico() {
            for input in COMMON_IMAGE_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "ico"), Some("imagemagick"),
                    "{} -> ico should use imagemagick", input
                );
            }
        }

        #[test]
        fn test_image_to_avif() {
            for input in COMMON_IMAGE_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "avif"), Some("imagemagick"),
                    "{} -> avif should use imagemagick", input
                );
            }
        }

        #[test]
        fn test_jpg_jpeg_rename() {
            assert_eq!(determine_conversion_tool("jpg", "jpeg"), Some("rename"));
            assert_eq!(determine_conversion_tool("jpeg", "jpg"), Some("rename"));
        }
    }

    // ==========================================
    // HEIC/HEIF CONVERSION TESTS
    // ==========================================

    mod heic_conversions {
        use super::*;

        const HEIC_INPUTS: &[&str] = &["jpg", "jpeg", "png", "bmp", "tiff", "webp", "gif"];

        #[test]
        fn test_image_to_heic() {
            for input in HEIC_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "heic"), Some("imagemagick"),
                    "{} -> heic should use imagemagick", input
                );
            }
        }

        #[test]
        fn test_image_to_heif() {
            for input in HEIC_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "heif"), Some("imagemagick"),
                    "{} -> heif should use imagemagick", input
                );
            }
        }

        #[test]
        fn test_heic_to_common_formats() {
            let outputs = ["jpg", "png", "bmp", "tiff", "webp"];
            for output in &outputs {
                assert_eq!(
                    determine_conversion_tool("heic", output), Some("imagemagick"),
                    "heic -> {} should use imagemagick", output
                );
            }
        }

        #[test]
        fn test_heif_to_common_formats() {
            let outputs = ["jpg", "png", "bmp", "tiff", "webp"];
            for output in &outputs {
                assert_eq!(
                    determine_conversion_tool("heif", output), Some("imagemagick"),
                    "heif -> {} should use imagemagick", output
                );
            }
        }
    }

    // ==========================================
    // PROFESSIONAL IMAGE FORMAT TESTS
    // ==========================================

    mod professional_image_conversions {
        use super::*;

        #[test]
        fn test_tga_conversions() {
            assert_eq!(determine_conversion_tool("tga", "png"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("tga", "jpg"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("png", "tga"), Some("imagemagick"));
        }

        #[test]
        fn test_exr_conversions() {
            assert_eq!(determine_conversion_tool("exr", "png"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("exr", "jpg"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("png", "exr"), Some("imagemagick"));
        }

        #[test]
        fn test_hdr_conversions() {
            assert_eq!(determine_conversion_tool("hdr", "png"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("hdr", "jpg"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("png", "hdr"), Some("imagemagick"));
        }

        #[test]
        fn test_psd_conversions() {
            assert_eq!(determine_conversion_tool("psd", "png"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("psd", "jpg"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("png", "psd"), Some("imagemagick"));
        }

        #[test]
        fn test_jpeg2000_conversions() {
            assert_eq!(determine_conversion_tool("j2k", "png"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("jp2", "png"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("png", "j2k"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("png", "jp2"), Some("imagemagick"));
        }
    }

    // ==========================================
    // X WINDOW SYSTEM FORMAT TESTS
    // ==========================================

    mod x_window_conversions {
        use super::*;

        #[test]
        fn test_xbm_conversions() {
            assert_eq!(determine_conversion_tool("png", "xbm"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("jpg", "xbm"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("xbm", "png"), Some("imagemagick"));
        }

        #[test]
        fn test_xpm_conversions() {
            assert_eq!(determine_conversion_tool("png", "xpm"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("jpg", "xpm"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("xpm", "png"), Some("imagemagick"));
        }

        #[test]
        fn test_xwd_conversions() {
            assert_eq!(determine_conversion_tool("png", "xwd"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("jpg", "xwd"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("xwd", "png"), Some("imagemagick"));
        }
    }

    // ==========================================
    // RAW IMAGE FORMAT TESTS
    // ==========================================

    mod raw_image_conversions {
        use super::*;

        const RAW_FORMATS: &[&str] = &[
            "arw", "cr2", "cr3", "dng", "nef", "orf", "raf", "raw", "rw2"
        ];

        #[test]
        fn test_raw_to_jpg() {
            for raw in RAW_FORMATS {
                assert_eq!(
                    determine_conversion_tool(raw, "jpg"), Some("imagemagick"),
                    "{} -> jpg should use imagemagick", raw
                );
            }
        }

        #[test]
        fn test_raw_to_png() {
            for raw in RAW_FORMATS {
                assert_eq!(
                    determine_conversion_tool(raw, "png"), Some("imagemagick"),
                    "{} -> png should use imagemagick", raw
                );
            }
        }

        #[test]
        fn test_raw_to_tiff() {
            for raw in RAW_FORMATS {
                assert_eq!(
                    determine_conversion_tool(raw, "tiff"), Some("imagemagick"),
                    "{} -> tiff should use imagemagick", raw
                );
            }
        }
    }

    // ==========================================
    // OFFICE DOCUMENT CONVERSION TESTS
    // ==========================================

    mod office_conversions {
        use super::*;

        #[test]
        fn test_word_to_pdf() {
            assert_eq!(determine_conversion_tool("doc", "pdf"), Some("libreoffice"));
            assert_eq!(determine_conversion_tool("docx", "pdf"), Some("libreoffice"));
        }

        #[test]
        fn test_excel_to_pdf() {
            assert_eq!(determine_conversion_tool("xls", "pdf"), Some("libreoffice"));
            assert_eq!(determine_conversion_tool("xlsx", "pdf"), Some("libreoffice"));
        }

        #[test]
        fn test_powerpoint_to_pdf() {
            assert_eq!(determine_conversion_tool("ppt", "pdf"), Some("libreoffice"));
            assert_eq!(determine_conversion_tool("pptx", "pdf"), Some("libreoffice"));
        }

        #[test]
        fn test_odt_to_pdf() {
            assert_eq!(determine_conversion_tool("odt", "pdf"), Some("libreoffice"));
        }

        #[test]
        fn test_office_to_html() {
            for input in OFFICE_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "html"), Some("libreoffice"),
                    "{} -> html should use libreoffice", input
                );
            }
        }

        #[test]
        fn test_office_to_txt() {
            for input in OFFICE_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "txt"), Some("libreoffice"),
                    "{} -> txt should use libreoffice", input
                );
            }
        }

        #[test]
        fn test_office_to_docx() {
            for input in OFFICE_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "docx"), Some("libreoffice"),
                    "{} -> docx should use libreoffice", input
                );
            }
        }

        #[test]
        fn test_office_to_odt() {
            for input in OFFICE_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "odt"), Some("libreoffice"),
                    "{} -> odt should use libreoffice", input
                );
            }
        }

        #[test]
        fn test_office_to_rtf() {
            for input in OFFICE_INPUTS {
                assert_eq!(
                    determine_conversion_tool(input, "rtf"), Some("libreoffice"),
                    "{} -> rtf should use libreoffice", input
                );
            }
        }
    }

    // ==========================================
    // UNSUPPORTED CONVERSION TESTS
    // ==========================================

    mod unsupported_conversions {
        use super::*;

        #[test]
        fn test_unknown_input_format() {
            assert_eq!(determine_conversion_tool("xyz", "mp4"), None);
            assert_eq!(determine_conversion_tool("abc", "png"), None);
            assert_eq!(determine_conversion_tool("unknown", "pdf"), None);
        }

        #[test]
        fn test_unknown_output_format() {
            assert_eq!(determine_conversion_tool("mp4", "xyz"), None);
            assert_eq!(determine_conversion_tool("png", "abc"), None);
            assert_eq!(determine_conversion_tool("docx", "unknown"), None);
        }

        #[test]
        fn test_cross_category_unsupported() {
            // Video to document
            assert_eq!(determine_conversion_tool("mp4", "docx"), None);
            assert_eq!(determine_conversion_tool("avi", "pdf"), None);
            
            // Audio to image
            assert_eq!(determine_conversion_tool("mp3", "png"), None);
            assert_eq!(determine_conversion_tool("wav", "jpg"), None);
            
            // Image to audio
            assert_eq!(determine_conversion_tool("png", "mp3"), None);
            assert_eq!(determine_conversion_tool("jpg", "wav"), None);
        }

        #[test]
        fn test_empty_formats() {
            assert_eq!(determine_conversion_tool("", "mp4"), None);
            assert_eq!(determine_conversion_tool("mp4", ""), None);
            assert_eq!(determine_conversion_tool("", ""), None);
        }
    }

    // ==========================================
    // FORMAT DETECTION TESTS
    // ==========================================

    mod format_detection {
        use super::*;

        #[test]
        fn test_is_video_format_comprehensive() {
            // All video formats should be recognized
            for format in VIDEO_INPUTS {
                assert!(is_video_format(format), "{} should be video", format);
            }
            
            // Case insensitivity
            assert!(is_video_format("MP4"));
            assert!(is_video_format("MoV"));
            assert!(is_video_format("AVI"));
            
            // Non-video formats should not match
            assert!(!is_video_format("mp3"));
            assert!(!is_video_format("png"));
            assert!(!is_video_format("docx"));
        }

        #[test]
        fn test_is_audio_format_comprehensive() {
            // All audio formats should be recognized
            for format in AUDIO_INPUTS {
                assert!(is_audio_format(format), "{} should be audio", format);
            }
            
            // Case insensitivity
            assert!(is_audio_format("MP3"));
            assert!(is_audio_format("WaV"));
            assert!(is_audio_format("FLAC"));
            
            // Non-audio formats should not match
            assert!(!is_audio_format("mp4"));
            assert!(!is_audio_format("png"));
            assert!(!is_audio_format("docx"));
        }

        #[test]
        fn test_is_image_format_comprehensive() {
            // All image formats should be recognized
            for format in IMAGE_INPUTS {
                assert!(is_image_format(format), "{} should be image", format);
            }
            
            // Case insensitivity
            assert!(is_image_format("PNG"));
            assert!(is_image_format("JpG"));
            assert!(is_image_format("HEIC"));
            
            // Non-image formats should not match
            assert!(!is_image_format("mp4"));
            assert!(!is_image_format("mp3"));
            assert!(!is_image_format("docx"));
        }

        #[test]
        fn test_is_document_format_comprehensive() {
            // Document inputs should be recognized
            for format in DOC_INPUTS {
                assert!(is_document_format(format), "{} should be document", format);
            }
            
            // Office inputs should be recognized
            for format in OFFICE_INPUTS {
                assert!(is_document_format(format), "{} should be document", format);
            }
            
            // Non-document formats should not match
            assert!(!is_document_format("mp4"));
            assert!(!is_document_format("mp3"));
            assert!(!is_document_format("png"));
        }
    }

    // ==========================================
    // UTILITY FUNCTION TESTS
    // ==========================================

    mod utility_functions {
        use super::*;

        #[test]
        fn test_normalize_extension_comprehensive() {
            // With leading dot
            assert_eq!(normalize_extension(".png"), "png");
            assert_eq!(normalize_extension(".MP4"), "mp4");
            assert_eq!(normalize_extension(".JpEg"), "jpeg");
            
            // Without leading dot
            assert_eq!(normalize_extension("png"), "png");
            assert_eq!(normalize_extension("MP4"), "mp4");
            assert_eq!(normalize_extension("JpEg"), "jpeg");
            
            // Multiple dots
            assert_eq!(normalize_extension("...test"), "test");
            assert_eq!(normalize_extension("..png"), "png");
            
            // Empty
            assert_eq!(normalize_extension(""), "");
            assert_eq!(normalize_extension("."), "");
        }

        #[test]
        fn test_get_format_display_name_comprehensive() {
            // Video formats
            assert_eq!(get_format_display_name("mp4"), "MP4 Video");
            assert_eq!(get_format_display_name("mov"), "QuickTime Video");
            assert_eq!(get_format_display_name("avi"), "AVI Video");
            assert_eq!(get_format_display_name("mkv"), "Matroska Video");
            assert_eq!(get_format_display_name("webm"), "WebM Video");
            
            // Audio formats
            assert_eq!(get_format_display_name("mp3"), "MP3 Audio");
            assert_eq!(get_format_display_name("wav"), "WAV Audio");
            assert_eq!(get_format_display_name("flac"), "FLAC Audio (Lossless)");
            assert_eq!(get_format_display_name("ogg"), "OGG Audio");
            assert_eq!(get_format_display_name("m4a"), "M4A Audio");
            assert_eq!(get_format_display_name("aac"), "AAC Audio");
            
            // Image formats
            assert_eq!(get_format_display_name("jpg"), "JPEG Image");
            assert_eq!(get_format_display_name("jpeg"), "JPEG Image");
            assert_eq!(get_format_display_name("png"), "PNG Image");
            assert_eq!(get_format_display_name("gif"), "Animated GIF");
            assert_eq!(get_format_display_name("bmp"), "BMP Image");
            assert_eq!(get_format_display_name("webp"), "WebP Image");
            assert_eq!(get_format_display_name("heic"), "HEIC Image");
            assert_eq!(get_format_display_name("avif"), "AVIF Image");
            assert_eq!(get_format_display_name("ico"), "Icon");
            assert_eq!(get_format_display_name("svg"), "SVG Vector");
            assert_eq!(get_format_display_name("psd"), "Photoshop Document");
            
            // Document formats
            assert_eq!(get_format_display_name("pdf"), "PDF Document");
            assert_eq!(get_format_display_name("docx"), "Word Document");
            assert_eq!(get_format_display_name("txt"), "Plain Text");
            assert_eq!(get_format_display_name("html"), "HTML Document");
            assert_eq!(get_format_display_name("md"), "Markdown");
            assert_eq!(get_format_display_name("epub"), "E-Book");
            
            // Unknown
            assert_eq!(get_format_display_name("unknown"), "Unknown Format");
            assert_eq!(get_format_display_name("xyz"), "Unknown Format");
        }

        #[test]
        fn test_get_format_color_comprehensive() {
            // Video - blue
            assert_eq!(get_format_color("mp4"), "blue");
            assert_eq!(get_format_color("mov"), "blue");
            assert_eq!(get_format_color("avi"), "blue");
            assert_eq!(get_format_color("mkv"), "blue");
            
            // WebM - green
            assert_eq!(get_format_color("webm"), "green");
            
            // Audio - various
            assert_eq!(get_format_color("mp3"), "green");
            assert_eq!(get_format_color("wav"), "green");
            assert_eq!(get_format_color("flac"), "aquamarine");
            assert_eq!(get_format_color("ogg"), "orange");
            assert_eq!(get_format_color("m4a"), "light-purple");
            assert_eq!(get_format_color("aac"), "yellow");
            
            // Images
            assert_eq!(get_format_color("jpg"), "light-tan");
            assert_eq!(get_format_color("png"), "light-tan");
            assert_eq!(get_format_color("gif"), "pink");
            assert_eq!(get_format_color("webp"), "green");
            assert_eq!(get_format_color("heic"), "green");
            
            // Documents
            assert_eq!(get_format_color("pdf"), "pink");
            assert_eq!(get_format_color("docx"), "blue");
            assert_eq!(get_format_color("html"), "orange");
            
            // Unknown
            assert_eq!(get_format_color("unknown"), "gray");
        }
    }

    // ==========================================
    // EDGE CASE TESTS
    // ==========================================

    mod edge_cases {
        use super::*;

        #[test]
        fn test_same_format_conversion() {
            // Same format conversions should still return a tool (for re-encoding)
            assert_eq!(determine_conversion_tool("mp4", "mp4"), Some("ffmpeg"));
            assert_eq!(determine_conversion_tool("mp3", "mp3"), Some("ffmpeg"));
            assert_eq!(determine_conversion_tool("png", "png"), Some("imagemagick"));
            assert_eq!(determine_conversion_tool("jpg", "jpg"), Some("imagemagick"));
        }

        #[test]
        fn test_case_sensitivity() {
            // Tool determination uses lowercase, but format detection handles case
            assert!(is_video_format("MP4"));
            assert!(is_video_format("mp4"));
            assert!(is_audio_format("MP3"));
            assert!(is_audio_format("mp3"));
            assert!(is_image_format("PNG"));
            assert!(is_image_format("png"));
        }

        #[test]
        fn test_tiff_tif_equivalence() {
            // Both tiff and tif should work the same
            assert_eq!(
                determine_conversion_tool("png", "tiff"),
                determine_conversion_tool("png", "tif")
            );
            assert_eq!(
                determine_conversion_tool("tiff", "png"),
                determine_conversion_tool("tif", "png")
            );
        }

        #[test]
        fn test_jpeg_jpg_equivalence_as_input() {
            // jpeg and jpg should work the same as input
            assert_eq!(
                determine_conversion_tool("jpeg", "png"),
                determine_conversion_tool("jpg", "png")
            );
        }
    }
}

