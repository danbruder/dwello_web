use result::{Error, Payload};
use rocket_contrib::json::Json;

pub type ApiResponse<T> = Result<Json<Payload<T>>, Error>;
