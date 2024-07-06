use std::fs::File;

use std::io::Write;

use crate::custom_error::CustomError;

pub fn create_file(output_file: &str, svg_image: String) -> Result<(), CustomError> {
    let mut file = File::create(output_file)
        .map_err(|_| CustomError::FileError("Failed to created a file".to_string()))?;
    file.write_all(svg_image.as_bytes())
        .map_err(|_| CustomError::FileError("Failed to write a file".to_string()))?;

    Ok(())
}

pub fn get_output_filename(input_path: &str) -> String {
    let filename = std::path::Path::new(input_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("default_name");

    let output_filename = format!("{}_compressed.jpeg", filename);
    output_filename
}
