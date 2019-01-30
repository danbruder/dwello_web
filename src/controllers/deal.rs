use accounts::types::{CurrentUser, CurrentUser::*};
use db::Conn;
use deals::types::*;
use deals::types::{Deal, House};
use diesel::prelude::*;
use error::Error;
use rocket::request::Form;
use rocket_contrib::json::Json;
use std::borrow::Cow;
use validator::{Validate, ValidationError, ValidationErrors};
use web::ApiData;

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

/// Create deal and house input data
#[derive(Deserialize, Validate)]
pub struct CreateDealAndHouseInput {
    pub buyer_id: i32,
    #[validate(length(min = "1", max = "500", message = "Cannot be blank"))]
    pub address: String,
    #[validate(length(min = "1", max = "500", message = "Cannot be blank"))]
    pub lat: String,
    #[validate(length(min = "1", max = "500", message = "Cannot be blank"))]
    pub lon: String,
}

type Response<T> = Result<Json<ApiData<T>>, Error>;

#[get("/deals")]
pub fn get_deals(user: CurrentUser, conn: Conn) -> Response<Vec<Deal>> {
    // Currently only admins can create deals
    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;
    use schema::deals::dsl::*;

    let d = deals.filter(buyer_id.eq(&user.id)).load::<Deal>(&conn)?;

    Ok(Json(ApiData {
        data: d,
        success: true,
        ..Default::default()
    }))
}

#[post("/deals", format = "application/json", data = "<input>")]
pub fn create_deal(
    user: CurrentUser,
    conn: Conn,
    input: Json<CreateDealAndHouseInput>,
) -> Response<DealWithHouse> {
    use schema::deals::dsl::*;
    use schema::houses::dsl::id;
    use schema::houses::dsl::*;

    // Currently only admins can create deals
    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;
    let formatted_address = input.address.trim().to_uppercase();

    input.validate()?;

    // Look for a house with address
    let house = match houses
        .filter(address.eq(&formatted_address))
        .first::<House>(&conn)
    {
        Ok(house) => house,
        Err(diesel::NotFound) => diesel::insert_into(houses)
            .values(&NewHouse {
                address: formatted_address,
                lat: input.lat.clone(),
                lon: input.lon.clone(),
                created: chrono::Utc::now().naive_utc(),
                updated: chrono::Utc::now().naive_utc(),
            })
            .get_result::<House>(&conn)?,
        Err(e) => return Err(Error::from(e)),
    };

    // Create a deal and link it to the house and buyer
    // Make sure one doesn't exist already
    let deal = match deals
        .filter(house_id.eq(&house.id))
        .filter(buyer_id.eq(&input.buyer_id))
        .first::<Deal>(&conn)
    {
        Ok(_) => {
            let mut errors = ValidationErrors::new();
            let mut error = ValidationError::new("deal_exists");
            error.message = Some(Cow::from("Existing deal for address"));
            errors.add("address", error);
            return Err(Error::InvalidInput(errors));
        }
        Err(diesel::NotFound) => diesel::insert_into(deals)
            .values(&NewDeal {
                buyer_id: Some(input.buyer_id),
                seller_id: None,
                house_id: Some(house.id),
                access_code: "CODE".to_string(),
                status: DealStatus::Initialized,
                created: chrono::Utc::now().naive_utc(),
                updated: chrono::Utc::now().naive_utc(),
            })
            .get_result::<Deal>(&conn)?,
        Err(e) => return Err(Error::from(e)),
    };

    Ok(Json(ApiData {
        data: DealWithHouse {
            id: deal.id,
            buyer_id: deal.buyer_id,
            seller_id: deal.seller_id,
            house_id: deal.house_id,
            access_code: deal.access_code,
            status: deal.status,
            address: house.address,
            lat: house.lat,
            lon: house.lon,
        },
        success: true,
        error_message: None,
        validation_errors: None,
        page_info: None,
    }))
}

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
) -> Response<DealWithHouse> {
    use schema::deals::dsl::*;
    use schema::deals::dsl::{id, updated};
    use schema::houses::dsl::*;

    // Currently only admins can create deals
    let _ = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;

    let deal = deals.filter(id.eq(deal_id)).first::<Deal>(&conn)?;

    // If the field is set, use the value
    // If it is not set, ignore.
    let deal = diesel::update(&deal)
        .set((
            status.eq(input.status.unwrap_or(deal.status)),
            updated.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&conn)?;

    let deal = deals
        .inner_join(houses)
        .select((
            id,
            buyer_id,
            seller_id,
            house_id,
            access_code,
            status,
            address,
            lat,
            lon,
        ))
        .filter(id.eq(deal_id))
        .first::<DealWithHouse>(&conn)?;

    Ok(Json(ApiData {
        data: deal,
        success: true,
        error_message: None,
        validation_errors: None,
        page_info: None,
    }))
}

#[derive(FromForm)]
pub struct ViewDealsWithHousesQuery {
    buyer_id: Option<i32>,
}

#[get("/views/deals-with-houses?<query..>")]
pub fn deals_with_houses(
    query: Option<Form<ViewDealsWithHousesQuery>>,
    user: CurrentUser,
    conn: Conn,
) -> Response<Vec<DealWithHouse>> {
    use schema::deals;
    use schema::houses;

    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;

    let bid = match query {
        Some(q) => match q.buyer_id {
            Some(b) => b,
            None => user.id,
        },
        None => user.id,
    };

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
        .filter(deals::dsl::buyer_id.eq(bid))
        .limit(10)
        .order_by(deals::created.desc())
        .load::<DealWithHouse>(&conn)?;

    Ok(Json(ApiData {
        data: d,
        success: true,
        error_message: None,
        validation_errors: None,
        page_info: None,
    }))
}
