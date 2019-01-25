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
            InputError(validation_errors) => {
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
            AccessDenied => (
                Status::Forbidden,
                Json(json!({
                    "status": "error",
                    "reason": "Forbidden"
                })),
            ),
            ApiKeyError => (
                Status::UnprocessableEntity,
                Json(json!({
                    "status": "error",
                    "reason": "Api key error"
                })),
            ),
            PasswordNoMatch => {
                let mut errors = HashMap::new();
                errors.insert("password", "Password does not match");
                (
                    Status::UnprocessableEntity,
                    Json(json!({ "errors": errors })),
                )
            }
            EmailTaken => {
                let mut errors = HashMap::new();
                errors.insert("email", "Email is taken");
                (
                    Status::UnprocessableEntity,
                    Json(json!({ "errors": errors })),
                )
            }
            EmailDoesntExist => {
                let mut errors = HashMap::new();
                errors.insert("email", "Email doesn't exist");
                (
                    Status::UnprocessableEntity,
                    Json(json!({ "errors": errors })),
                )
            }
            DealExists => {
                let mut errors = HashMap::new();
                errors.insert("deal", "Deal exists");
                (
                    Status::UnprocessableEntity,
                    Json(json!({ "errors": errors })),
                )
            }
            _ => (
                Status::ServiceUnavailable,
                Json(json!({
                    "status": "error",
                    "reason": "Service unavailable"
                })),
            ),
        };

        status::Custom(res_status, payload).respond_to(req)
    }
}
