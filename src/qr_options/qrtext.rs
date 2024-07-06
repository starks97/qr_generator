use qrcode::QrCode;

use crate::models::qr_model::QRDetails;

use crate::custom_error::CustomError;

use crate::qr_strategy::{QRStrategy, QrStrategyResult};

use crate::utils::create_file;

pub struct QrText {
    pub common: QRDetails,
}

impl QRStrategy for QrText {
    fn generate(&self, output_file: &str) -> QrStrategyResult<()> {
        println!("QRDetails: {}", self.common);

        let code =
            QrCode::new(format!("{}", self.common)).map_err(|e| CustomError::QrError(Err(e)))?;

        let svg_image = code.render::<qrcode::render::svg::Color>().build();

        create_file(output_file, svg_image)?;

        println!("QR code generated and saved to {}", output_file);
        Ok(())
    }
}
