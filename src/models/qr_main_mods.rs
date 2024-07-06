use core::fmt;

use serde::{Deserialize, Serialize};

use sqlx::FromRow;

use uuid::Uuid;

use serde_json::Value;

use super::qr_model::QRProfileCard;

use super::qr_business::QRBusinessPage;

use validator::{Validate, ValidationErrors};

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct QrDataModel {
    pub id: Uuid,
    pub data_json: Value,
    pub data_type: DataType,
    pub views: Option<i32>,
    #[serde(default)]
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum QRDataType {
    ProfileCard(QRProfileCard),
    BusinessPage(QRBusinessPage),
}

impl QRDataType {
    pub fn into_value_inner(&self) -> Result<Value, serde_json::Error> {
        match self {
            Self::ProfileCard(v) => serde_json::to_value(v),
            Self::BusinessPage(v) => serde_json::to_value(v),
        }
    }

    pub fn data_type(&self) -> &str {
        match self {
            Self::ProfileCard(_) => "ProfileCard",
            Self::BusinessPage(_) => "BusinessPage",
        }
    }
    pub fn validate_inner(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::ProfileCard(profile) => profile.validate(),
            Self::BusinessPage(business) => business.validate(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DataType {
    ProfileCard,
    BusinessPage,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_type = match self {
            DataType::BusinessPage => "BusinessPage",
            DataType::ProfileCard => "ProfileCard",
        };

        write!(f, "{}", data_type)
    }
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BusinessPage" => DataType::BusinessPage,
            "ProfileCard" => DataType::ProfileCard,
            _ => panic!("Invalid data_type provided"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QrQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub data_type: Option<DataType>,
}

#[derive(Debug, Deserialize)]
pub struct QrDataPath {
    pub qr_id: Uuid,
}
