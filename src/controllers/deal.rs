use accounts::types::{CurrentUser, CurrentUser::*};
use db::Conn;
use deals::types::*;
use deals::types::{Deal, House};
use diesel::prelude::*;
use error::Error;
use rocket_contrib::json::Json;
use validator::Validate;

#[get("/deals")]
pub fn get_deals(user: CurrentUser, conn: Conn) -> Result<Json<Vec<Deal>>, Error> {
    // Currently only admins can create deals
    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;
    use schema::deals::dsl::*;

    let d = deals.filter(buyer_id.eq(&user.id)).load::<Deal>(&conn)?;

    Ok(Json(d))
}

/// Create deal and house input data
#[derive(Deserialize, Validate)]
pub struct CreateDealAndHouseInput {
    #[validate(length(min = "1", max = "500", message = "Cannot be blank"))]
    pub address: String,
    #[validate(length(min = "1", max = "500", message = "Cannot be blank"))]
    pub lat: String,
    #[validate(length(min = "1", max = "500", message = "Cannot be blank"))]
    pub lon: String,
}

#[derive(Serialize, Default)]
pub struct CreateDealAndHousePayload {
    pub house: Option<House>,
    pub deal: Option<Deal>,
}

#[post("/deals", format = "application/json", data = "<input>")]
pub fn create_deal(
    user: CurrentUser,
    conn: Conn,
    input: Json<CreateDealAndHouseInput>,
) -> Result<Json<CreateDealAndHousePayload>, Error> {
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

    input.validate().map_err(|e| Error::InputError(e))?;

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
        .filter(buyer_id.eq(&user.id))
        .first::<Deal>(&conn)
    {
        Ok(_) => return Err(Error::DealExists),
        Err(diesel::NotFound) => diesel::insert_into(deals)
            .values(&NewDeal {
                buyer_id: Some(user.id),
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

    Ok(Json(CreateDealAndHousePayload {
        house: Some(house),
        deal: Some(deal),
    }))
}
