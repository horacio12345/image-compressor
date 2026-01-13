use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use image::{DynamicImage, codecs::jpeg::JpegEncoder};

use crate::models::{ProcessOptions, ProgressInfo, ImageError, OutputFormat};

/// Procesa múltiples imágenes en paralelo
pub fn process_images(
    paths: Vec<PathBuf>,
    options: ProcessOptions,
) -> Result<ProgressInfo, ImageError> {
    // Validación inicial
    if paths.is_empty() {
        return Err(ImageError::InternalError {
            message: "No images provided".to_string(),
        });
    }

    // ProgressInfo compartido entre threads (Arc<Mutex<T>>)
    let progress = Arc::new(Mutex::new(ProgressInfo::new(paths.len())));
    
    // Procesar en paralelo con Rayon
    paths.par_iter().for_each(|path| {
        let result = process_single_image(path, &options);
        
        // Actualizar progreso (thread-safe)
        let mut prog = progress.lock().unwrap();
        match result {
            Ok(_) => prog.successful += 1,
            Err(_) => prog.failed += 1,
        }
        prog.current_file = Some(path.to_string_lossy().to_string());
    });

    // Retornar resultado final
    let final_progress = progress.lock().unwrap().clone();
    Ok(final_progress)
}

/// Procesa una imagen individual
fn process_single_image(
    input_path: &Path,
    options: &ProcessOptions,
) -> Result<(), ImageError> {
    // 1. Cargar imagen
    let img = load_image(input_path)?;
    
    // 2. Redimensionar si es necesario
    let img = resize_if_needed(img, options.width);
    
    // 3. Convertir y comprimir según formato
    let output_path = build_output_path(input_path, options);
    save_image(&img, &output_path, options)?;
    
    Ok(())
}

/// Carga imagen desde disco
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

/// Redimensiona imagen si width está especificado (mantiene aspect ratio)
fn resize_if_needed(img: DynamicImage, target_width: Option<u32>) -> DynamicImage {
    match target_width {
        Some(width) if width < img.width() => {
            let height = (width as f32 * img.height() as f32 / img.width() as f32) as u32;
            img.resize(width, height, image::imageops::FilterType::Lanczos3)
        }
        _ => img,
    }
}

/// Construye ruta de salida para imagen procesada
fn build_output_path(input_path: &Path, options: &ProcessOptions) -> PathBuf {
    let file_stem = input_path.file_stem().unwrap().to_string_lossy();
    let extension = match options.format {
        OutputFormat::Jpeg => "jpg",
        OutputFormat::Png => "png",
        OutputFormat::Webp => "webp",
    };
    
    options.output_dir.join(format!("{}_compressed.{}", file_stem, extension))
}

/// Guarda imagen con compresión según formato
fn save_image(
    img: &DynamicImage,
    output_path: &Path,
    options: &ProcessOptions,
) -> Result<(), ImageError> {
    let quality = options.quality.to_percentage();
    
    match options.format {
        OutputFormat::Jpeg => {
            let rgb_img = img.to_rgb8();
            let mut file = std::fs::File::create(output_path).map_err(|e| {
                ImageError::SaveFailed {
                    path: output_path.to_string_lossy().to_string(),
                    reason: e.to_string(),
                }
            })?;
            
            let mut encoder = JpegEncoder::new_with_quality(&mut file, quality);
            encoder.encode(
                rgb_img.as_raw(),
                rgb_img.width(),
                rgb_img.height(),
                image::ExtendedColorType::Rgb8,
            ).map_err(|e| {
                ImageError::SaveFailed {
                    path: output_path.to_string_lossy().to_string(),
                    reason: e.to_string(),
                }
            })?;
        }
        OutputFormat::Png => {
            img.save(output_path).map_err(|e| {
                ImageError::SaveFailed {
                    path: output_path.to_string_lossy().to_string(),
                    reason: e.to_string(),
                }
            })?;
        }
        OutputFormat::Webp => {
            return Err(ImageError::InternalError {
                message: "WebP format not supported yet".to_string(),
            });
        }
    }
    
    Ok(())
}