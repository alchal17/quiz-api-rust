use base64::{Engine, engine::general_purpose};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub fn save_base64_image(base64_data: &str, upload_dir: &str) -> Result<String, String> {
    let raw = if let Some(comma) = base64_data.find(',') {
        &base64_data[comma + 1..]
    } else {
        base64_data
    };

    let bytes = general_purpose::STANDARD
        .decode(raw)
        .map_err(|e| format!("Invalid base64: {}", e))?;

    let extension =
        detect_image_type(&bytes).ok_or_else(|| "Not a recognized image format".to_string())?;

    const MAX_SIZE: usize = 5 * 1024 * 1024;
    if bytes.len() > MAX_SIZE {
        return Err(format!(
            "Image too large: {} bytes (max {})",
            bytes.len(),
            MAX_SIZE
        ));
    }

    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let path = PathBuf::from(upload_dir).join(&filename);

    fs::write(&path, &bytes).map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(filename)
}

pub fn delete_image(filename: &str, upload_dir: &str) -> Result<(), String> {
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err(format!("Invalid filename: {}", filename));
    }

    let path = PathBuf::from(upload_dir).join(filename);

    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("Failed to delete image {}: {}", filename, e)),
    }
}

fn detect_image_type(bytes: &[u8]) -> Option<&'static str> {
    if bytes.len() < 4 {
        return None;
    }
    // PNG: 89 50 4E 47
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return Some("png");
    }
    // JPEG: FF D8 FF
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("jpg");
    }
    // GIF: "GIF8"
    if bytes.starts_with(b"GIF8") {
        return Some("gif");
    }
    // WebP: "RIFF"..."WEBP"
    if bytes.starts_with(b"RIFF") && bytes.len() > 11 && &bytes[8..12] == b"WEBP" {
        return Some("webp");
    }
    None
}

pub fn read_image(filename: &str, upload_dir: &str) -> Result<(Vec<u8>, &'static str), String> {
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err(format!("Invalid filename: {}", filename));
    }

    let path = PathBuf::from(upload_dir).join(filename);

    let bytes = fs::read(&path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => format!("Image not found: {}", filename),
        _ => format!("Failed to read image {}: {}", filename, e),
    })?;

    let content_type =
        detect_content_type(&bytes).ok_or_else(|| "Unrecognized image format".to_string())?;

    Ok((bytes, content_type))
}

fn detect_content_type(bytes: &[u8]) -> Option<&'static str> {
    if bytes.len() < 4 {
        return None;
    }
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return Some("image/png");
    }
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg");
    }
    if bytes.starts_with(b"GIF8") {
        return Some("image/gif");
    }
    if bytes.starts_with(b"RIFF") && bytes.len() > 11 && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    None
}
