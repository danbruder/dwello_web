//
// user.rs
//

use schema::users;
use error::ScoutError;
use juniper::{FieldResult,FieldError,Executor};
use diesel::prelude::*;
use super::Ctx;

#[derive(GraphQLObject, Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[graphql(skip)]
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
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
}

