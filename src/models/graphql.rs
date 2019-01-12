//
// models/graphql.rs
//
use models::user::User;

// A trait that the Validate derive will impl
use validator::{Validate,ValidationErrors};
use error;

#[derive(GraphQLInputObject, Clone, Validate)]
pub struct RegistrationInput {
    #[validate(length(min = "1", max = "256"))]
    pub name: String,
    #[validate(email(message="Email %s is not valid"))]
    pub email: String,
    #[validate(length(min = "6", max = "30", message="Password length must be between 6 and 30"))]
    pub password: String,
}

#[derive(GraphQLObject, Clone)]
pub struct AuthPayload {
    pub token: Option<String>,
    pub user: Option<User>,
    pub valid: bool,
    pub validation_errors: Option<Vec<ValidationError>>
}

impl AuthPayload { 
    pub fn from_validation_errors(e: ValidationErrors) -> AuthPayload { 
        let errors = error::from_validation_errors(e);
        AuthPayload{
            user: None,
            token: None,
            valid: false,
            validation_errors: Some(errors)
        }
    }
    pub fn from_simple_error(key: &'static str, value: &'static str) -> AuthPayload { 
        AuthPayload{
            user: None,
            token: None,
            valid: false,
            validation_errors: Some(vec![ValidationError{
                field: key.to_string(),
                message: value.to_string()
            }])
        }
    }
}

#[derive(GraphQLInputObject, Clone)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(GraphQLObject, Clone)]
pub struct ValidationError { 
    pub field: String,
    pub message: String
}
