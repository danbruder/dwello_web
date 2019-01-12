//
// models/user.rs
//
use schema::{users};
use db::{PooledConnection};
use diesel::prelude::*;
use web::ApiKey;
use super::session::Session;

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
    pub fn from_key(conn: PooledConnection, key: ApiKey) -> Option<User> {
        use schema::users::dsl::*;
        use schema::users::dsl::id;
        use schema::sessions::dsl::*;

        // Load session and user
        let mut user = None;
        let session = sessions 
            .filter(token.eq(key.0))
            .first::<Session>(&conn).ok();
        if let Some(s) = session { 
            user = users 
                .filter(id.eq(s.uid))
                .first::<User>(&conn).ok();
        }

        user
    }
}
