//
// error.rs
//
use std::fmt::{Display,Formatter,Result};
use validator::ValidationErrors;
use models::graphql::ValidationError;

#[derive(Debug)]
pub enum ScoutError { 
    AccessDenied
}


impl Display for ScoutError { 
    fn fmt(&self, f: &mut Formatter) -> Result{
        match self { 
            ScoutError::AccessDenied => write!(f, "Access denied")
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
