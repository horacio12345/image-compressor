use crate::models::{ImageError, PrivacyLevel};

pub fn process_metadata(
    image_data: &[u8],
    privacy_level: &PrivacyLevel,
) -> Result<Vec<u8>, ImageError> {
    match privacy_level {
        PrivacyLevel::KeepAll => Ok(image_data.to_vec()),
        PrivacyLevel::RemoveAll => {
        
            Ok(image_data.to_vec())
        }
    }
}