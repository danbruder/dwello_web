//
// user.rs
//

use schema::users;
use super::PooledConnection;
use error::ScoutError;
use juniper::{FieldResult,FieldError};
use diesel::prelude::*;

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
     pub fn all_users(connection: PooledConnection, current_user: Option<User>) -> FieldResult<Vec<User>> {
        use schema::users::dsl::*;

        if current_user.is_none() { 
            return Err(FieldError::from(ScoutError::AccessDenied));
        }

        users
            .limit(10)
            .load::<User>(&connection)
            .or(Ok(vec![]))
    }
}

