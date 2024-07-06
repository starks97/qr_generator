use crate::custom_error::CustomError;

use image::{io::Reader as ImageReader, DynamicImage};

use std::fs::{self};

pub struct ImageProcessor {
    pub input_path: String,
}

impl ImageProcessor {
    pub fn new(input_path: &str) -> Self {
        Self {
            input_path: input_path.to_string(),
        }
    }
    pub async fn process_image(&self) -> Result<DynamicImage, CustomError> {
        let image_size = self.get_img_size().await?;
        println!("Image size: {} bytes", image_size);

        if image_size > 30_000 {
            println!("Image exceeds 30 KB, compressing...");
            self.compress_image().await
        } else {
            println!("Image is 30 KB or smaller, no compression needed.");
            self.read_image().await
        }
    }
    async fn compress_image(&self) -> Result<DynamicImage, CustomError> {
        let img = self.read_image().await?;
        let resized_img = img
            .resize(100, 100, image::imageops::FilterType::Gaussian)
            .rotate90();
        Ok(resized_img)
    }

    async fn read_image(&self) -> Result<DynamicImage, CustomError> {
        let img = ImageReader::open(&self.input_path)
            .map_err(|e| CustomError::FileError(e.to_string()))?
            .decode()
            .map_err(|e| CustomError::ImageError(e.into()))?;
        Ok(img)
    }

    async fn get_img_size(&self) -> Result<u64, CustomError> {
        let metadata =
            fs::metadata(&self.input_path).map_err(|e| CustomError::OtherError(e.to_string()))?;
        Ok(metadata.len())
    }
}
