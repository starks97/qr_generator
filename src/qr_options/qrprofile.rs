use qrcode::{EcLevel, QrCode, Version};

use crate::models::qr_model::QRProfileCard;

use crate::custom_error::CustomError;

use crate::qr_strategy::{QRStrategy, QrStrategyResult};

use crate::utils::create_file;

pub struct QrProfile {
    pub details: QRProfileCard,
}

impl QRStrategy for QrProfile {
    fn generate(&self, output_file: &str) -> QrStrategyResult<()> {
        let json_data = serde_json::to_string(&self.details)
            .map_err(|e| CustomError::OtherError(e.to_string()))?;

        let data_bytes = json_data.as_bytes();
        println!("{:?}", data_bytes.len());

        let code = QrCode::with_version(data_bytes, Version::Normal(30), EcLevel::L)
            .map_err(|e| CustomError::QrError(Err(e)))?;

        let svg_image = code.render::<qrcode::render::svg::Color>().build();

        create_file(output_file, svg_image)?;

        println!("QR code generated and saved to {}", output_file);
        Ok(())
    }
}
