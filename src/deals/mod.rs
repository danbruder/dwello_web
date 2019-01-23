//
// deal/mod.rs
//
pub mod types;

use diesel::prelude::*;
use self::types::*;
use accounts::types::User;
use db::PooledConnection;
use error::ScoutError;


//pub fn create_deal(
    //conn: PooledConnection,
    //current_user: Option<User>,
    //input: HouseInput
    //) -> Result<Deal, ScoutError> {
    //use schema::deals::dsl::*;
    //use schema::houses::dsl::*;
    //use schema::houses::dsl::id;

    //if current_user.is_none() { 
        //return Err(ScoutError::AccessDenied);
    //}

    //let user = current_user.unwrap();

    //let formatted_address = input.address.trim().to_uppercase();

    //// Look for a house with address
    //let house = match houses
        //.filter(address.eq(&formatted_address))
        //.first::<House>(&conn) {
            //Ok(house) => house,
            //Err(diesel::NotFound) => {
                //diesel::insert_into(houses) 
                    //.values(&NewHouse{
                        //address: formatted_address,
                        //lat: input.lat,
                        //lon: input.lon,
                        //created: chrono::Utc::now().naive_utc(),
                        //updated: chrono::Utc::now().naive_utc(),
                    //}).get_result::<House>(&conn)?
            //},
            //Err(e) => return Err(ScoutError::from(e))
        //};

    //// Create a deal and link it to the house and buyer
    //// Make sure one doesn't exist already
    //let deal = match deals
        //.filter(house_id.eq(&house.id))
        //.filter(buyer_id.eq(&user.id))
        //.first::<Deal>(&conn) {
            //Ok(_) => return Err(ScoutError::DealExists),
            //Err(diesel::NotFound) => {
                //diesel::insert_into(deals) 
                    //.values(&NewDeal{
                        //buyer_id: Some(user.id),
                        //seller_id: None,
                        //house_id: Some(house.id),
                        //access_code: "CODE".to_string(),
                        //status: DealStatus::Initialized,
                        //created: chrono::Utc::now().naive_utc(),
                        //updated: chrono::Utc::now().naive_utc(),
                    //}).get_result::<Deal>(&conn)?
            //},
            //Err(e) => return Err(ScoutError::from(e))
        //};

    //Ok(deal)
//}
