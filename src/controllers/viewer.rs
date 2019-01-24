use rocket_contrib::json::Json;
use rocket::State;
use db::{Db};
use accounts::types::CurrentUser;
//use deals::types::Deal;

#[derive(Serialize)]
pub struct UserWithDeals { 
    pub user: CurrentUser,
    //pub deals: Vec<Deal>
}

#[get("/views/user-with-deals")]
pub fn user_with_deals(
    user: CurrentUser,
    db: State<Db>,
    ) -> Json<UserWithDeals> { 


    Json(UserWithDeals{
        user: user,
        //deals: vec![]
    })
}
