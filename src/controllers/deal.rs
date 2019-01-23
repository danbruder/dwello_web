use rocket_contrib::json::Json;
use accounts::types::{CurrentUser, CurrentUser::*  };
use rocket::http::Status;
use rocket::State;
use deals::types::*;
use db::{Db};
use diesel::prelude::*;
use error::ScoutError;
use validator::Validate;
use validation::ValidationError;
use deals::types::{House,Deal};

#[derive(Deserialize, Validate)]
pub struct CreateDealAndHouseData {
    #[validate(length(min = "1", max = "500", message="Cannot be blank"))]
    pub address: String,
    #[validate(length(min = "1", max = "500", message="Cannot be blank"))]
    pub lat: String,
    #[validate(length(min = "1", max = "500", message="Cannot be blank"))]
    pub lon: String,
}

#[derive(Serialize,Default)]
pub struct CreateDealAndHousePayload {
    pub house: Option<House>,
    pub deal: Option<Deal>,
    pub valid: bool,
    pub validation_errors: Option<Vec<ValidationError>>
}

#[post("/deals", format = "application/json", data = "<input>")]
pub fn create_deal(
    user: CurrentUser,
    db: State<Db>,
    input: Json<CreateDealAndHouseData>
    ) -> Result<Json<CreateDealAndHousePayload>, ScoutError> { 
    let conn = db.pool.get().unwrap();

    use schema::deals::dsl::*;
    use schema::houses::dsl::*;
    use schema::houses::dsl::id;

    // Currently only admins can create deals
    let user = match user { 
        Admin(user) => user,
        _ => return Err(ScoutError::AccessDenied)
    };

    let formatted_address = input.address.trim().to_uppercase();

    match input.validate() {
        Err(e) => return Ok(Json(CreateDealAndHousePayload{
            ..Default::default(),
            validation_errors: Some(error::from_validation_errors(e))
        })),
        Ok(_) => ()
    }

    // Look for a house with address
    let house = match houses
        .filter(address.eq(&formatted_address))
        .first::<House>(&conn) {
            Ok(house) => house,
            Err(diesel::NotFound) => {
                diesel::insert_into(houses) 
                    .values(&NewHouse{
                        address: formatted_address,
                        lat: input.lat.clone(),
                        lon: input.lon.clone(),
                        created: chrono::Utc::now().naive_utc(),
                        updated: chrono::Utc::now().naive_utc(),
                    }).get_result::<House>(&conn)?
            },
            Err(e) => return Err(ScoutError::from(e))
        };

    // Create a deal and link it to the house and buyer
    // Make sure one doesn't exist already
    let deal = match deals
        .filter(house_id.eq(&house.id))
        .filter(buyer_id.eq(&user.id))
        .first::<Deal>(&conn) {
            Ok(_) => return Err(ScoutError::DealExists),
            Err(diesel::NotFound) => {
                diesel::insert_into(deals) 
                    .values(&NewDeal{
                        buyer_id: Some(user.id),
                        seller_id: None,
                        house_id: Some(house.id),
                        access_code: "CODE".to_string(),
                        status: DealStatus::Initialized,
                        created: chrono::Utc::now().naive_utc(),
                        updated: chrono::Utc::now().naive_utc(),
                    }).get_result::<Deal>(&conn)?
            },
            Err(e) => return Err(ScoutError::from(e))
        };

    Ok(Json(CreateDealAndHousePayload{
        house: Some(house),
        deal: Some(deal),
        valid: true,
        validation_errors: None
    }))
}
