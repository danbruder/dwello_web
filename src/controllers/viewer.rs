use rocket_contrib::json::Json;
use rocket::State;
use db::{Db};
use accounts::types::CurrentUser;

#[derive(Serialize)]
pub struct ViewUserWithDealsResponse { 
    //pub user: Option<User>,
    pub count: i32
    //pub deals: Vec<Deal>
}

#[get("/views/user-with-deals")]
pub fn user_with_deals(
    user: CurrentUser,
    db: State<Db>,
    ) -> Json<ViewUserWithDealsResponse> { 
    //let connection = db.pool.get().unwrap();
    //let user = User::from_key(connection, key);

    Json(ViewUserWithDealsResponse{
        count: 0
    })
}
