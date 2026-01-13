use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use exif::{In, Tag, Reader};

use crate::models::ImageError;

/// EXIF Orientation values according to EXIF spec
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExifOrientation {
    Horizontal = 1,        // Normal
    MirrorHorizontal = 2,  // Flipped horizontally
    Rotate180 = 3,         // Rotated 180°
    MirrorVertical = 4,    // Flipped vertically
    MirrorHorizontalRotate270CW = 5, // Flipped horizontally, then rotated 270° CW
    Rotate90CW = 6,        // Rotated 90° CW
    MirrorHorizontalRotate90CW = 7,  // Flipped horizontally, then rotated 90° CW
    Rotate270CW = 8,       // Rotated 270° CW (or 90° CCW)
}

impl Default for ExifOrientation {
    fn default() -> Self {
        ExifOrientation::Horizontal
    }
}

impl ExifOrientation {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => ExifOrientation::Horizontal,
            2 => ExifOrientation::MirrorHorizontal,
            3 => ExifOrientation::Rotate180,
            4 => ExifOrientation::MirrorVertical,
            5 => ExifOrientation::MirrorHorizontalRotate270CW,
            6 => ExifOrientation::Rotate90CW,
            7 => ExifOrientation::MirrorHorizontalRotate90CW,
            8 => ExifOrientation::Rotate270CW,
            _ => ExifOrientation::Horizontal,
        }
    }
}

/// Reads EXIF orientation from an image file
pub fn read_orientation(path: &Path) -> Result<ExifOrientation, ImageError> {
    let file = File::open(path).map_err(|e| ImageError::ExifReadError {
        path: path.to_string_lossy().to_string(),
        details: format!("Failed to open file: {}", e),
    })?;

    let mut bufreader = BufReader::new(&file);
    let exifreader = Reader::new();

    let exif = exifreader.read_from_container(&mut bufreader).map_err(|e| ImageError::ExifReadError {
        path: path.to_string_lossy().to_string(),
        details: format!("Failed to read EXIF data: {}", e),
    })?;

    // Try to find the Orientation tag
    if let Some(field) = exif.get_field(Tag::Orientation, In::PRIMARY) {
        if let Some(orientation_value) = field.value.get_uint(0) {
            return Ok(ExifOrientation::from_u32(orientation_value));
        }
    }

    // If no orientation tag found, assume normal orientation
    Ok(ExifOrientation::Horizontal)
}
