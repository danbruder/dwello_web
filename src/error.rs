//
// error.rs
//
use rocket::response::status;
use std::collections::HashMap;
use serde_json::json;
use std::fmt::{Display,Formatter,Result};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response;
use validator::ValidationErrors;
use self::Error::*;
use rocket_contrib::json::Json;
use rocket::request::{self, Request, FromRequest};

#[derive(Debug)]
pub enum Error { 
    BcryptError(bcrypt::BcryptError),
    DieselError(diesel::result::Error),
    InputError(ValidationErrors), 
    ServiceUnavailable,
    ApiKeyError,
    AccessDenied,
    PasswordNoMatch,
    EmailTaken
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
        let (res_status, payload) = match self { 
            InputError(validation_errors) =>  {
                let mut errors = HashMap::new();
                for (field, ers) in validation_errors.field_errors() {
                    errors.insert(
                        field,
                        ers.into_iter()
                        .map(|err| err.message.to_owned())
                        .collect::<Vec<_>>(),
                        );
                }
                (Status::UnprocessableEntity, Json(json!({ 
                    "errors": errors 
                })))
            }, 
            AccessDenied => (Status::Forbidden, Json(json!({
                "status": "error",
                "reason": "Forbidden"
            }))),
            ApiKeyError => (Status::UnprocessableEntity, Json(json!({
                "status": "error",
                "reason": "Api key error"
            }))),
            PasswordNoMatch => {
                let mut errors = HashMap::new();
                errors.insert("password", "Password does not match");
                (Status::UnprocessableEntity, Json(json!({
                    "errors": errors
                })))
            },
            EmailTaken => {
                let mut errors = HashMap::new();
                errors.insert("email", "Email is taken");
                (Status::UnprocessableEntity, Json(json!({
                    "errors": errors
                })))
            },
            _ => (Status::ServiceUnavailable, Json(json!({
                "status": "error",
                "reason": "Service unavailable"
            }))),
        };

        status::Custom(res_status, payload).respond_to(req)
    }
}
