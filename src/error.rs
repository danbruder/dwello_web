//
// error.rs
//

use std::fmt::{Display,Formatter,Result};

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
