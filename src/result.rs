//
// error.rs
//
use self::Error::*;
use std::borrow::Cow;
use validator::{ValidationError as ExtValidationError, ValidationErrors};

#[derive(Serialize)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub total_count: u32,
}

#[derive(Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Serialize, Default)]
pub struct Payload<T> {
    pub data: T,
    pub success: bool,
    pub error_message: Option<String>,
    pub validation_errors: Option<Vec<ValidationError>>,
    pub page_info: Option<PageInfo>,
}

pub type Response<T> = Result<Payload<T>, Error>;
pub type ErrorPayload = Payload<Option<String>>;

#[derive(Debug)]
pub enum Error {
    BcryptError(bcrypt::BcryptError),
    DieselError(diesel::result::Error),
    InvalidInput(validator::ValidationErrors),
    JsonError(serde_json::Error),
    ServiceUnavailable,
    ApiKeyError,
    AccessDenied,
}

impl From<bcrypt::BcryptError> for Error {
    fn from(error: bcrypt::BcryptError) -> Self {
        BcryptError(error)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        DieselError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        JsonError(error)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(error: validator::ValidationErrors) -> Self {
        InvalidInput(error)
    }
}

impl Error {
    pub fn from_custom_validation(code: &'static str, field: &'static str, message: &str) -> Self {
        let mut errors = ValidationErrors::new();
        let mut error = ExtValidationError::new(code.clone());
        error.message = Some(Cow::from(message.to_owned()));
        errors.add(field.clone(), error);
        Error::InvalidInput(errors)
    }
}
