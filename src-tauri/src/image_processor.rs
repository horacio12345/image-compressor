use image::{codecs::jpeg::JpegEncoder, DynamicImage};
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::models::{ImageError, OutputFormat, ProcessOptions, ProgressInfo};

/// Processes multiple images in parallel
pub fn process_images(
    paths: Vec<PathBuf>,
    options: ProcessOptions,
) -> Result<ProgressInfo, ImageError> {
    // Initial validation
    if paths.is_empty() {
        return Err(ImageError::InternalError {
            message: "No images provided".to_string(),
        });
    }

    // ProgressInfo shared between threads (Arc<Mutex<T>>)
    let progress = Arc::new(Mutex::new(ProgressInfo::new(paths.len())));

    // Process in parallel using Rayon
    paths.par_iter().for_each(|path| {
        let result = process_single_image(path, &options);

        // Update progress (thread-safe)
        let mut prog = progress.lock().unwrap();
        match result {
            Ok(_) => prog.successful += 1,
            Err(_) => prog.failed += 1,
        }
        prog.current_file = Some(path.to_string_lossy().to_string());
    });

    // Return final result
    let final_progress = progress.lock().unwrap().clone();
    Ok(final_progress)
}

/// Processes a single image
fn process_single_image(input_path: &Path, options: &ProcessOptions) -> Result<(), ImageError> {
    // 1. Load image
    let mut img = load_image(input_path)?;

    // 2. Apply EXIF orientation BEFORE resizing (critical fix!)
    img = apply_exif_orientation(input_path, img)?;

    // 3. Resize if necessary
    let img = resize_if_needed(img, options.width);

    // 4. Convert and compress based on format (metadata is stripped here)
    let output_path = build_output_path(input_path, options);
    save_image(&img, &output_path, options)?;

    Ok(())
}

/// Loads image from disk
fn load_image(path: &Path) -> Result<DynamicImage, ImageError> {
    if !path.exists() {
        return Err(ImageError::PathNotFound {
            path: path.to_string_lossy().to_string(),
        });
    }

    image::open(path).map_err(|_| ImageError::InvalidFormat {
        path: path.to_string_lossy().to_string(),
    })
}

/// Reads EXIF orientation and applies the correct rotation/flip to the image
/// This fixes images from iPhones and cameras that use EXIF orientation tags
fn apply_exif_orientation(path: &Path, img: DynamicImage) -> Result<DynamicImage, ImageError> {
    // Try to read EXIF data
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(img), // No EXIF, return original
    };

    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif_data = match exifreader.read_from_container(&mut bufreader) {
        Ok(data) => data,
        Err(_) => return Ok(img), // No EXIF, return original
    };

    // Get orientation tag
    let orientation = match exif_data.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
        Some(field) => match field.value.get_uint(0) {
            Some(v) => v,
            None => return Ok(img), // No orientation value
        },
        None => return Ok(img), // No orientation tag
    };

    // Apply transformation based on EXIF orientation value
    // https://magnushoff.com/articles/jpeg-orientation/
    let transformed = match orientation {
        1 => img,                     // Normal - no transformation needed
        2 => img.fliph(),             // Flip horizontal
        3 => img.rotate180(),         // Rotate 180
        4 => img.flipv(),             // Flip vertical
        5 => img.rotate90().fliph(),  // Rotate 90 CW and flip horizontal
        6 => img.rotate90(),          // Rotate 90 CW
        7 => img.rotate270().fliph(), // Rotate 270 CW and flip horizontal
        8 => img.rotate270(),         // Rotate 270 CW (or 90 CCW)
        _ => img,                     // Unknown value, return original
    };

    Ok(transformed)
}

/// Resizes image if width is specified (maintaining aspect ratio)
fn resize_if_needed(img: DynamicImage, target_width: Option<u32>) -> DynamicImage {
    match target_width {
        Some(width) if width < img.width() => {
            let height = (width as f32 * img.height() as f32 / img.width() as f32) as u32;
            img.resize(width, height, image::imageops::FilterType::Lanczos3)
        }
        _ => img,
    }
}

/// Builds output path for the processed image
fn build_output_path(input_path: &Path, options: &ProcessOptions) -> PathBuf {
    let file_stem = input_path.file_stem().unwrap().to_string_lossy();
    let extension = match options.format {
        OutputFormat::Jpeg => "jpg",
        OutputFormat::Png => "png",
    };

    options
        .output_dir
        .join(format!("{}_compressed.{}", file_stem, extension))
}

/// Saves image with compression according to format
fn save_image(
    img: &DynamicImage,
    output_path: &Path,
    options: &ProcessOptions,
) -> Result<(), ImageError> {
    let quality = options.quality.to_percentage();

    match options.format {
        OutputFormat::Jpeg => {
            let rgb_img = img.to_rgb8();
            let mut file =
                std::fs::File::create(output_path).map_err(|e| ImageError::SaveFailed {
                    path: output_path.to_string_lossy().to_string(),
                    reason: e.to_string(),
                })?;

            let mut encoder = JpegEncoder::new_with_quality(&mut file, quality);
            encoder
                .encode(
                    rgb_img.as_raw(),
                    rgb_img.width(),
                    rgb_img.height(),
                    image::ExtendedColorType::Rgb8,
                )
                .map_err(|e| ImageError::SaveFailed {
                    path: output_path.to_string_lossy().to_string(),
                    reason: e.to_string(),
                })?;
        }
        OutputFormat::Png => {
            use image::codecs::png::{CompressionType, FilterType as PngFilter, PngEncoder};

            let compression = match quality {
                90 => CompressionType::Fast,
                75 => CompressionType::Default,
                _ => CompressionType::Best, // 60%
            };

            let file = std::fs::File::create(output_path).map_err(|e| ImageError::SaveFailed {
                path: output_path.to_string_lossy().to_string(),
                reason: e.to_string(),
            })?;

            let encoder = PngEncoder::new_with_quality(file, compression, PngFilter::Adaptive);
            img.write_with_encoder(encoder)
                .map_err(|e| ImageError::SaveFailed {
                    path: output_path.to_string_lossy().to_string(),
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}
