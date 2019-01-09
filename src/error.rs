//
// error.rs
//

use juniper::{FieldError,IntoFieldError};

#[derive(Debug)]
pub enum ScoutError { 
    AccessDeined
}

impl<ScoutError> IntoFieldError<ScoutError> for FieldError<ScoutError> {
    fn into_field_error(self) -> FieldError {
        match self { 
            ScoutError::AccessDeined => FieldError::new("Access denied", graphql_value!(""))
        }
    }
}
