use rocket_contrib::json::Json;
use accounts::types::User;
use rocket::State;
use deals::types::Deal;
use db::{Db};
use web::ApiKey;

#[derive(Serialize)]
pub struct ViewUserWithDealsResponse { 
    pub user: Option<User>,
    pub deals: Vec<Deal>
}

#[get("/views/user-with-deals")]
pub fn user_with_deals(
    key: ApiKey,
    db: State<Db>,
    ) -> Json<ViewUserWithDealsResponse> { 
    let connection = db.pool.get().unwrap();
    let user = User::from_key(connection, key);

    Json(ViewUserWithDealsResponse{
        user: user,
        deals: vec![]
    })
}
