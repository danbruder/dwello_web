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
use std::collections::HashMap;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum Error {
    BcryptError(bcrypt::BcryptError),
    DieselError(diesel::result::Error),
    InputError(ValidationErrors),
    ServiceUnavailable,
    ApiKeyError,
    AccessDenied,
    PasswordNoMatch,
    EmailDoesntExist,
    EmailTaken,
    DealExists,
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

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        // Log the error
        println!("{:?}", self);

        let (res_status, payload) = match self {
            InputError(validation_errors) => format_validation_errors(validation_errors),
            AccessDenied => access_denied(),
            ApiKeyError => validation_error("api_key", "Api key is invalid"),
            PasswordNoMatch => validation_error("password", "Password does not match"),
            EmailTaken => validation_error("email", "Email is taken"),
            EmailDoesntExist => validation_error("email", "Email doesn't exist"),
            DealExists => validation_error("deal", "Deal exists"),
            _ => unavailable(),
        };

        status::Custom(res_status, payload).respond_to(req)
    }
}

fn access_denied() -> (Status, Json<serde_json::Value>) {
    (
        Status::Forbidden,
        Json(json!({
            "status": "error",
            "reason": "Forbidden"
        })),
    )
}

fn validation_error(key: &str, val: &str) -> (Status, Json<serde_json::Value>) {
    let mut errors = HashMap::new();
    errors.insert(key, val);
    (
        Status::UnprocessableEntity,
        Json(json!({ "errors": errors })),
    )
}

// Format multiple errors
fn error(key: &str, val: &str) -> (Status, Json<serde_json::Value>) {
    let mut errors = HashMap::new();
    errors.insert(key, val);
    (
        Status::UnprocessableEntity,
        Json(json!({ "errors": errors })),
    )
}

// Format multiple errors
fn unavailable() -> (Status, Json<serde_json::Value>) {
    (
        Status::ServiceUnavailable,
        Json(json!({
            "status": "error",
            "reason": "Service unavailable"
        })),
    )
}

// Format multiple errors
fn format_validation_errors(
    validation_errors: ValidationErrors,
) -> (Status, Json<serde_json::Value>) {
    let mut errors = HashMap::new();
    for (field, ers) in validation_errors.field_errors() {
        errors.insert(
            field,
            ers.into_iter()
                .map(|err| err.message.to_owned())
                .collect::<Vec<_>>(),
        );
    }
    (
        Status::UnprocessableEntity,
        Json(json!({ "errors": errors })),
    )
}
