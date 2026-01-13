use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum QualityPreset {
    High,    // 90%
    Medium,  // 75%
    Low,     // 60%
}

impl QualityPreset {
    pub fn to_percentage(&self) -> u8 {
        match self {
            QualityPreset::High => 90,
            QualityPreset::Medium => 75,
            QualityPreset::Low => 60,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Jpeg,
    Png,
}

#[derive(Debug, Clone)]
pub enum PrivacyLevel {
    KeepAll,
    RemoveAll,
}

#[derive(Debug)]
pub enum ImageError {
    PathNotFound { path: String },
    InvalidFormat { path: String },
    ExifReadError { path: String, details: String },
    SaveFailed { path: String, reason: String },
    InvalidDimensions { width: u32, height: u32, reason: String },
    InternalError { message: String },
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageError::PathNotFound { path } => 
                write!(f, "Path not found: {}", path),
            ImageError::InvalidFormat { path } => 
                write!(f, "Invalid format: {}", path),
            ImageError::ExifReadError { path, details } => 
                write!(f, "EXIF read error in {}: {}", path, details),
            ImageError::SaveFailed { path, reason } => 
                write!(f, "Save failed for {}: {}", path, reason),
            ImageError::InvalidDimensions { width, height, reason } => 
                write!(f, "Invalid dimensions {}x{}: {}", width, height, reason),
            ImageError::InternalError { message } => 
                write!(f, "Internal error: {}", message),
        }
    }
}

impl std::error::Error for ImageError {}

#[derive(Debug, Clone)]
pub struct ProcessOptions {
    pub quality: QualityPreset,
    pub format: OutputFormat,
    pub privacy: PrivacyLevel,
    pub width: Option<u32>,
    pub output_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ProgressInfo {
    pub total_images: usize,
    pub successful: usize,
    pub failed: usize,
    pub current_file: Option<String>,
}

impl ProgressInfo {
    pub fn new(total: usize) -> Self {
        Self {
            total_images: total,
            successful: 0,
            failed: 0,
            current_file: None,
        }
    }

    pub fn processed(&self) -> usize {
        self.successful + self.failed
    }
}