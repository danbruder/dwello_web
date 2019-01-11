//
// error.rs
//

use std::error::Error as StdError;
use std::fmt::{Display,Formatter,Result};
use juniper::{IntoFieldError, FieldError};
use diesel::result::Error::{DatabaseError,NotFound,RollbackTransaction,AlreadyInTransaction};
use diesel::result::{DatabaseErrorKind};

#[derive(Debug)]
pub enum ScoutError { 
    AccessDenied,
    InvalidEmail,
    InvalidPassword,
    HashError(bcrypt::BcryptError),
    DbError(diesel::result::Error)
}


impl StdError for ScoutError {
    fn description(&self) -> &str {
        match *self {
            ScoutError::AccessDenied => "Access denied",
            ScoutError::InvalidEmail => "Invalid email",
            ScoutError::InvalidPassword => "Invalid password",
            ScoutError::DbError(e) => {
                match e {
                    DatabaseError(k, _info) => {
                        match k {
                            DatabaseErrorKind::UniqueViolation => "Already exists",
                            DatabaseErrorKind::ForeignKeyViolation => "Foreign key violation",
                            _ => "Unknown database kind error",
                        }
                    }, 
                    NotFound => "Not found",
                    RollbackTransaction => "Transaction error",
                    AlreadyInTransaction => "Transaction error",
                    _ => "Unknown database error",
                }
            },
            ScoutError::HashError(e) => "Internal server error"
        }
    }
}

impl Display for ScoutError { 
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self { 
            ScoutError::AccessDenied => write!(f, "Access denied"),
            ScoutError::InvalidEmail => write!(f, "Invalid email"),
            ScoutError::InvalidPassword => write!(f, "Invalid password"),
            ScoutError::DbError(e) => {
                match e {
                    DatabaseError(k, _info) => {
                        match k {
                            DatabaseErrorKind::UniqueViolation => write!(f, "Already exists"),
                            DatabaseErrorKind::ForeignKeyViolation => write!(f, "Foreign key violation"),
                            _ => write!(f, "Unknown database kind error"),
                        }
                    }, 
                    NotFound => write!(f, "Not found"),
                    RollbackTransaction => write!(f, "Transaction error"),
                    AlreadyInTransaction => write!(f, "Transaction error"),
                    _ => write!(f, "Unknown database error"),
                }
            },
            ScoutError::HashError(e) => write!(f, "Internal server error")
        }
    }
}

impl IntoFieldError for ScoutError {
    fn into_field_error(self) -> FieldError {
        FieldError::new(self, graphql_value!(""))
    }
}

impl From<diesel::result::Error> for ScoutError {
    fn from(error: diesel::result::Error) -> Self {
        ScoutError::DbError(error)
    }
}

impl From<bcrypt::BcryptError> for ScoutError {
    fn from(error: bcrypt::BcryptError) -> Self {
        ScoutError::HashError(error)
    }
}
