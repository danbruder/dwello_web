//
// error.rs
//
use std::fmt::{Display,Formatter,Result};
use validator::ValidationErrors;
use accounts::types::ValidationError;
use std::error::Error;
use self::ScoutError::*;

#[derive(Debug)]
pub enum ScoutError { 
    BcryptError(bcrypt::BcryptError),
    DieselError(diesel::result::Error),

    // Access
    AccessDenied,

    // Deals
    DealExists
}


impl Display for ScoutError { 
    fn fmt(&self, f: &mut Formatter) -> Result{
        match self { 
            AccessDenied => write!(f, "Access denied"),
            DealExists => write!(f, "Deal exists"),
            BcryptError(ref e) => write!(f, "{}", e.description()),
            DieselError(ref e) => write!(f, "{}", e.description()),
        }
    }
}

pub fn from_validation_errors(e: ValidationErrors) -> Vec<ValidationError> { 
    let field_errors = e.field_errors();
    field_errors.iter().map(|(k, v)| {
        let messages = v
            .into_iter()
            .filter(|f| f.message.is_some())
            .map(|f| f.clone().message.unwrap().to_string())
            .collect::<Vec<String>>().join(", ");
        ValidationError{
            field: k.to_string(),
            message: messages
        }
    }).collect::<Vec<ValidationError>>()
}

impl From<bcrypt::BcryptError> for ScoutError { 
    fn from(error: bcrypt::BcryptError) -> Self {
        BcryptError(error)
    }
}

impl From<diesel::result::Error> for ScoutError { 
    fn from(error: diesel::result::Error) -> Self {
        DieselError(error)
    }
}
