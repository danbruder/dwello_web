#[derive(Serialize, Clone)]
pub struct ValidationError { 
    pub field: String,
    pub message: String
}

