use accounts::types::CurrentUser;
use accounts::types::CurrentUser::*;
use db::Conn;
use deals::types::DealStatus;
use diesel::prelude::*;
use error::Error;
use rocket_contrib::json::Json;

#[derive(Serialize, Queryable)]
pub struct DealWithHouse {
    pub id: i32,
    pub buyer_id: Option<i32>,
    pub seller_id: Option<i32>,
    pub house_id: Option<i32>,
    pub access_code: String,
    pub status: DealStatus,
    pub address: String,
    pub lat: String,
    pub lon: String,
}

#[get("/views/deals-with-houses")]
pub fn deals_with_houses(user: CurrentUser, conn: Conn) -> Result<Json<Vec<DealWithHouse>>, Error> {
    use schema::deals;
    use schema::houses;

    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;

    let d = deals::table
        .inner_join(houses::table)
        .select((
            deals::id,
            deals::buyer_id,
            deals::seller_id,
            deals::house_id,
            deals::access_code,
            deals::status,
            houses::address,
            houses::lat,
            houses::lon,
        ))
        .filter(deals::dsl::buyer_id.eq(user.id))
        .limit(10)
        .load::<DealWithHouse>(&conn)?;

    Ok(Json(d))
}
