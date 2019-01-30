use accounts::types::CurrentUser;
use db::Conn;
use deals;
use deals::types::*;
use rocket::request::Form;
use rocket_contrib::json::Json;
use web::ApiResponse;

/// Get all deals
#[get("/deals")]
pub fn get_deals(user: CurrentUser, conn: Conn) -> ApiResponse<Vec<Deal>> {
    deals::get_deals(user, conn).map(|r| Json(r))
}

/// Create deal
#[post("/deals", format = "application/json", data = "<input>")]
pub fn create_deal(
    user: CurrentUser,
    conn: Conn,
    input: Json<CreateDealAndHouseInput>,
) -> ApiResponse<DealWithHouse> {
    deals::create_deal(user, conn, input.into_inner()).map(|r| Json(r))
}

/// Update deal
#[post(
    "/deals/<deal_id>/update",
    format = "application/json",
    data = "<input>"
)]
pub fn update_deal(
    deal_id: i32,
    user: CurrentUser,
    conn: Conn,
    input: Json<UpdateDeal>,
) -> ApiResponse<DealWithHouse> {
    deals::update_deal(deal_id, user, conn, input.into_inner()).map(|r| Json(r))
}

/// View deals with houses
#[get("/views/deals-with-houses?<query..>")]
pub fn deals_with_houses(
    query: Option<Form<ViewDealsWithHousesQuery>>,
    user: CurrentUser,
    conn: Conn,
) -> ApiResponse<Vec<DealWithHouse>> {
    deals::deals_with_houses(query.map(|r| r.into_inner()), user, conn).map(|r| Json(r))
}
