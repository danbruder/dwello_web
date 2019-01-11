//
// resolvers/user.rs
//

use error::ScoutError::{AccessDenied, DbError};
use juniper::{FieldResult,Executor, IntoFieldError};
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
        return Err(AccessDenied.into_field_error());
    }

    users
        .limit(10)
        .load::<User>(&connection)
        .map_err(|e| DbError(e).into_field_error())
}
