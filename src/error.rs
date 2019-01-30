//
// error.rs
//
use self::Error::*;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::status;
use rocket::response::Responder;
use rocket_contrib::json::Json;
use serde_json::json;
use std::borrow::Cow;
use validator::{ValidationError as ExtValidationError, ValidationErrors};
use web::{Payload, ValidationError};

#[derive(Debug)]
pub enum Error {
    BcryptError(bcrypt::BcryptError),
    DieselError(diesel::result::Error),
    InvalidInput(validator::ValidationErrors),
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

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        // Log the error
        println!("{:?}", self);

        let (res_status, payload) = match self {
            AccessDenied => access_denied(),
            ApiKeyError => validation_error("api_key", "Api key is invalid"),
            DieselError(e) => match e {
                diesel::result::Error::NotFound => not_found(),
                _ => unavailable(),
            },

            // Validation errors
            InvalidInput(validation_errors) => format_validation_errors(validation_errors),
            _ => unavailable(),
        };

        status::Custom(res_status, payload).respond_to(req)
    }
}

type ErrorPayload = Payload<Option<String>>;

fn access_denied() -> (Status, Json<serde_json::Value>) {
    (
        Status::Forbidden,
        Json(json!(ErrorPayload {
            error_message: Some("Access deined".to_string()),
            ..Default::default()
        })),
    )
}

fn validation_error(field: &str, message: &str) -> (Status, Json<serde_json::Value>) {
    (
        Status::PartialContent,
        Json(json!(ErrorPayload {
            validation_errors: Some(vec![ValidationError {
                field: field.to_string(),
                message: message.to_string(),
            }]),
            ..Default::default()
        })),
    )
}

// Format multiple errors
fn not_found() -> (Status, Json<serde_json::Value>) {
    (
        Status::NotFound,
        Json(json!(ErrorPayload {
            error_message: Some("Not found".to_string()),
            ..Default::default()
        })),
    )
}

// Format multiple errors
fn unavailable() -> (Status, Json<serde_json::Value>) {
    (
        Status::ServiceUnavailable,
        Json(json!(ErrorPayload {
            error_message: Some("Service unavailable".to_string()),
            ..Default::default()
        })),
    )
}

// Format multiple errors
fn format_validation_errors(e: validator::ValidationErrors) -> (Status, Json<serde_json::Value>) {
    let errors = e
        .field_errors()
        .iter()
        .map(|(k, v)| {
            let messages = v
                .into_iter()
                .filter(|f| f.message.is_some())
                .map(|f| f.clone().message.unwrap().to_string())
                .collect::<Vec<String>>()
                .join(", ");
            ValidationError {
                field: k.to_string(),
                message: messages,
            }
        })
        .collect::<Vec<ValidationError>>();

    (
        Status::PartialContent,
        Json(json!(ErrorPayload {
            validation_errors: Some(errors),
            ..Default::default()
        })),
    )
}
