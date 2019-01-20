//
// deal/resolvers.rs
//

use error::ScoutError;
use juniper::{FieldResult,FieldError,Executor};
use diesel::prelude::*;
use super::types::*;
use graphql::Ctx;

pub fn create_deal(
    executor: &Executor<Ctx>,
    ) -> FieldResult<Deal> {
    use schema::deals::dsl::*;
    let current_user = executor.context().user.clone();
    let connection = executor.context().pool.get().unwrap();

    if current_user.is_none() { 
        return Err(FieldError::from(ScoutError::AccessDenied));
    }

    Ok( Deal{
        id: 1,
        bid: None,
        hid: None,
        sid: None,
        access_code: "123".to_string(),
        status: "Something".to_string(),
        created: chrono::Utc::now().naive_utc(),
        updated: chrono::Utc::now().naive_utc(),
    }
    )

        //let house = House::new(conn, 
        //users
        //.limit(10)
        //.load::<User>(&connection)
        //.map_err(|e| FieldError::from(e))
}
