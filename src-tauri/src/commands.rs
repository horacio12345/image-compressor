use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::models::{ProcessOptions, QualityPreset, OutputFormat, PrivacyLevel};
use crate::image_processor;

/// Tauri Command: process multiple images
#[tauri::command]
pub fn process_images_command(
    paths: Vec<String>,
    quality: String,
    format: String,
    privacy: String,
    width: Option<u32>,
    output_dir: String,
) -> Result<ProgressInfoResponse, String> {
    // Convert Strings to PathBuf
    let image_paths: Vec<PathBuf> = paths.iter().map(|p| PathBuf::from(p)).collect();
    let output_path = PathBuf::from(output_dir);
    
    // Parse parameters from strings
    let quality_preset = parse_quality(&quality)?;
    let output_format = parse_format(&format)?;
    let privacy_level = parse_privacy(&privacy)?;
    
    // Create options
    let options = ProcessOptions {
        quality: quality_preset,
        format: output_format,
        privacy: privacy_level,
        width,
        output_dir: output_path,
    };
    
    // Process images
    let progress = image_processor::process_images(image_paths, options)
        .map_err(|e| e.to_string())?;
    
    // Convert to serializable type for Tauri
    Ok(ProgressInfoResponse {
        total_images: progress.total_images,
        successful: progress.successful,
        failed: progress.failed,
        current_file: progress.current_file,
    })
}

/// Serializable type to return to the frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressInfoResponse {
    pub total_images: usize,
    pub successful: usize,
    pub failed: usize,
    pub current_file: Option<String>,
}

// Helper functions to parse strings from the frontend
fn parse_quality(quality: &str) -> Result<QualityPreset, String> {
    match quality.to_lowercase().as_str() {
        "high" | "alta" => Ok(QualityPreset::High),
        "medium" | "media" => Ok(QualityPreset::Medium),
        "low" | "baja" => Ok(QualityPreset::Low),
        _ => Err(format!("Invalid quality: {}", quality)),
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, String> {
    match format.to_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(OutputFormat::Jpeg),
        "png" => Ok(OutputFormat::Png),
        "webp" => Ok(OutputFormat::Webp),
        _ => Err(format!("Invalid format: {}", format)),
    }
}

fn parse_privacy(privacy: &str) -> Result<PrivacyLevel, String> {
    match privacy.to_lowercase().as_str() {
        "keep_all" | "todo" => Ok(PrivacyLevel::KeepAll),
        "remove_sensitive" | "sensible" => Ok(PrivacyLevel::RemoveAll),
        "remove_all" | "nada" => Ok(PrivacyLevel::RemoveAll),
        _ => Err(format!("Invalid privacy level: {}", privacy)),
    }
}