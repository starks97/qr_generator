use serde::{Deserialize, Serialize};

use validator::{Validate, ValidationError};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_PHONE_NUMBER: Regex = Regex::new(r#"^\+?[0-9\s]+$"#).unwrap();
}
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QRBusinessPage {
    #[validate(length(
        min = 5,
        max = 30,
        message = "Business name must be between 5 and 30 characters",
        code = "code_str"
    ))]
    pub business_name: String,

    pub business_image: Vec<u8>,

    #[validate(length(
        min = 10,
        max = 1000,
        message = "Business description must be between 10 and 1000 characters",
        code = "code_str"
    ))]
    pub business_description: String,

    #[validate(custom(function = "validate_phone", message = "Invalid phone number format"))]
    pub phone_number: String,

    #[validate(email(
        message = "Invalid email, please provide a valid email",
        code = "code_str"
    ))]
    pub email: String,

    #[validate(url(message = "Invalid URL, please provide a valid URL", code = "code_str"))]
    pub website: String,

    pub opening_hours: Option<chrono::DateTime<chrono::Utc>>,
    #[validate(length(min = 1, message = "Address must not be empty", code = "code_str"))]
    pub address: String,

    #[validate(length(min = 1, message = "City must not be empty", code = "code_str"))]
    pub city: String,

    #[validate(length(min = 1, message = "Country must not be empty", code = "code_str"))]
    pub country: String,

    #[validate(length(min = 1, message = "State must not be empty", code = "code_str"))]
    pub state: String,

    #[validate(length(min = 1, message = "Zipcode must not be empty", code = "code_str"))]
    pub zipcode: String,

    pub social_media: Vec<String>,
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if RE_PHONE_NUMBER.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid phone number format"))
    }
}
