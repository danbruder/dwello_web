//
// resolvers/user.rs
//

use error::ScoutError;
use juniper::{FieldResult,FieldError,Executor};
use diesel::prelude::*;
use ::models::User;
use graphql::Ctx;

pub fn all_users(
    executor: &Executor<Ctx>,
    ) -> FieldResult<Vec<User>> {
    use schema::users::dsl::*;
    let current_user = executor.context().user.clone();
    let connection = executor.context().pool.get().unwrap();

    if current_user.is_none() { 
        return Err(FieldError::from(ScoutError::AccessDenied));
    }

    users
        .limit(10)
        .load::<User>(&connection)
        .map_err(|e| FieldError::from(e))
}
