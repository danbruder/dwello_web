//
// resolvers/deal.rs
//

use error::ScoutError;
use juniper::{FieldResult,FieldError,Executor};
use diesel::prelude::*;
use models::deal::*;
use graphql::Ctx;

pub fn create_deal(
    executor: &Executor<Ctx>,
    input: HouseInput
    ) -> FieldResult<Deal> {
    use schema::deals::dsl::*;
    let current_user = executor.context().user.clone();
    let connection = executor.context().pool.get().unwrap();

    if current_user.is_none() { 
        return Err(FieldError::from(ScoutError::AccessDenied));
    }

    //let house = House::new(conn, 
    //users
        //.limit(10)
        //.load::<User>(&connection)
        //.map_err(|e| FieldError::from(e))
}
