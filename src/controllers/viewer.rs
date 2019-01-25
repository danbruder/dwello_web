use rocket_contrib::json::Json;
use db::Conn;
use error::Error;
use accounts::types::{CurrentUser,User};
use deals::types::Deal;
use diesel::prelude::*;
use accounts::types::CurrentUser::*;

#[derive(Serialize)]
pub struct UserWithDeals {
    pub user: User,
    pub deals: Vec<Deal>
}

#[get("/views/user-with-deals")]
pub fn user_with_deals(
    user: CurrentUser,
    conn: Conn
    ) -> Result<Json<UserWithDeals>, Error> {
    use schema::deals::dsl::*;

    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied)
    };
    let Conn(conn) = conn;

    let d = deals
        .filter(buyer_id.eq(user.id))
        .limit(10)
        .load::<Deal>(&conn)?;

    Ok(Json(UserWithDeals{
        user: user,
        deals: d
    }))
}
