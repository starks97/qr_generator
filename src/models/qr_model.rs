use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_PHONE_NUMBER: Regex = Regex::new(r#"^\+?[0-9\s]+$"#).unwrap();
}

#[derive(Debug, Serialize)]
pub struct QRDetails {
    pub custom_color: String,
    pub title: String,
    pub description: String,
}

impl std::fmt::Display for QRDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(json) => write!(f, "{}", json),
            Err(e) => write!(f, "Error serializing to JSON: {}", e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QRProfileCard {
    #[validate(length(
        min = 5,
        max = 30,
        message = "Full name must be between 5 and 30 characters",
        code = "code_str"
    ))]
    pub full_name: String,

    pub profile_image: Vec<u8>,

    #[validate(custom(function = "validate_phone", message = "Invalid phone number format"))]
    pub phone_number: String,

    #[validate(email(
        message = "Invalid email, please provide a valid email",
        code = "code_str"
    ))]
    pub email: String,

    #[validate(url(message = "Invalid URL, please provide a valid URL", code = "code_str"))]
    pub website: String,

    #[validate(length(
        min = 5,
        max = 30,
        message = "Company name must be between 5 and 30 characters",
        code = "code_str"
    ))]
    pub company_name: String,

    #[validate(length(
        min = 5,
        max = 30,
        message = "Your job name must be between 5 and 30 characters",
        code = "code_str"
    ))]
    pub your_job: String,

    #[validate(length(min = 1, message = "Address must not be empty", code = "code_str"))]
    pub address: String,

    #[validate(length(min = 1, message = "Country must not be empty", code = "code_str"))]
    pub country: String,

    #[validate(length(min = 1, message = "State must not be empty", code = "code_str"))]
    pub state: String,

    pub social_media: Vec<String>,

    #[validate(url(message = "Invalid URL format for link", code = "code_str"))]
    pub link: String,
}
fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if RE_PHONE_NUMBER.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid phone number format"))
    }
}
