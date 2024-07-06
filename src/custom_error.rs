use qrcode::QrResult as QrError;

use image::ImageError;

use sqlx::Error as DbError;

use actix_web::{HttpResponse, ResponseError};

use serde_json::json;

use thiserror::Error as ThisError;

use validator::ValidationErrors;

#[derive(Debug)]
pub enum ValidationModelsErrors {
    Error(String),
}

impl ValidationModelsErrors {
    fn error_message(&self) -> String {
        match self {
            ValidationModelsErrors::Error(msg) => msg.to_string(),
        }
    }
}

impl std::fmt::Display for ValidationModelsErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error_message())
    }
}

impl ResponseError for ValidationModelsErrors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": self.error_message()
        }))
    }
}

#[derive(Debug, ThisError)]
pub enum CustomError {
    #[error("QR code error: {0:?}")]
    QrError(QrError<()>),

    #[error("Other error: {0}")]
    OtherError(String),

    #[error("File error: {0}")]
    FileError(String),

    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),

    #[error("Database error: {0}")]
    DataBaseError(#[from] DbError),

    #[error("Validation error: {0}")]
    ValidationError(ValidationModelsErrors),
}

impl CustomError {
    fn log_error(&self) {
        match self {
            CustomError::QrError(_) => log::error!("QR code generation error: {:?}", self),
            CustomError::OtherError(_) => log::error!("Other error: {:?}", self),
            CustomError::FileError(_) => log::error!("File error: {:?}", self),
            CustomError::ImageError(_) => log::error!("Image processing error: {:?}", self),
            CustomError::DataBaseError(_) => log::error!("Database error: {:?}", self),
            CustomError::ValidationError(_) => log::error!("Validation error:  {:?}", self),
        }
    }

    fn error_response(&self, status_code: actix_web::http::StatusCode) -> HttpResponse {
        self.log_error();
        HttpResponse::build(status_code).json(json!({
            "status": "error",
            "message": format!("{}", self)
        }))
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::QrError(_) => {
                self.error_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            CustomError::OtherError(_) => {
                self.error_response(actix_web::http::StatusCode::BAD_REQUEST)
            }
            CustomError::FileError(_) => {
                self.error_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            CustomError::ImageError(_) => {
                self.error_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            CustomError::DataBaseError(_) => {
                self.error_response(actix_web::http::StatusCode::SERVICE_UNAVAILABLE)
            }
            CustomError::ValidationError(_) => {
                self.error_response(actix_web::http::StatusCode::BAD_REQUEST)
            }
        }
    }
}

impl From<ValidationModelsErrors> for CustomError {
    fn from(error: ValidationModelsErrors) -> Self {
        CustomError::ValidationError(error)
    }
}

pub fn handle_validation_error(
    validation_error: ValidationErrors,
) -> Result<HttpResponse, CustomError> {
    let error_message = validation_error
        .field_errors()
        .values()
        .map(|errors| {
            errors
                .iter()
                .map(|err| err.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect::<Vec<String>>()
        .join(", ");

    Err(CustomError::ValidationError(ValidationModelsErrors::Error(
        error_message,
    )))
}
