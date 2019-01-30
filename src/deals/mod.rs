//
// deal/mod.rs
//
pub mod types;

use accounts::types::{CurrentUser, CurrentUser::*};
use db::Conn;
use deals::types::*;
use deals::types::{Deal, House};
use diesel::prelude::*;
use result::{Error, Payload, Response};
use validator::Validate;

/// Get deals
pub fn get_deals(user: CurrentUser, conn: Conn) -> Response<Vec<Deal>> {
    // Currently only admins can create deals
    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;
    use schema::deals::dsl::*;

    let d = deals.filter(buyer_id.eq(&user.id)).load::<Deal>(&conn)?;

    Ok(Payload {
        data: d,
        success: true,
        ..Default::default()
    })
}

/// Create deal
pub fn create_deal(
    user: CurrentUser,
    conn: Conn,
    input: CreateDealAndHouseInput,
) -> Response<DealWithHouse> {
    use schema::deals::dsl::*;
    //use schema::houses::dsl::id;
    use schema::houses::dsl::*;

    // Currently only admins can create deals
    let _ = match user {
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
            return Err(Error::from_custom_validation(
                "deal_exists",
                "address",
                "Existing deal for address",
            ));
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

    Ok(Payload {
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
    })
}

/// Update Deal
pub fn update_deal(
    deal_id: i32,
    user: CurrentUser,
    conn: Conn,
    input: UpdateDeal,
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
    let _ = diesel::update(&deal)
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

    Ok(Payload {
        data: deal,
        success: true,
        error_message: None,
        validation_errors: None,
        page_info: None,
    })
}

/// Deals with houses
pub fn deals_with_houses(
    query: Option<ViewDealsWithHousesQuery>,
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

    Ok(Payload {
        data: d,
        success: true,
        error_message: None,
        validation_errors: None,
        page_info: None,
    })
}
