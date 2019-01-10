//
// auth.rs
//

use db::{PooledConnection};
use juniper::{FieldResult,FieldError};
use diesel::prelude::*;
use schema::{sessions};
use user::{User,NewUser};
use super::ApiKey;

#[derive(GraphQLObject, Clone, Queryable)]
struct Session {
    id: i32,
    uid: i32,
    created: chrono::NaiveDateTime,
    updated: chrono::NaiveDateTime,
    #[graphql(skip)]
    hash: String,
}

#[derive(Insertable)]
#[table_name = "sessions"]
struct NewSession {
    uid: i32,
    created: chrono::NaiveDateTime,
    updated: chrono::NaiveDateTime,
    hash: String,
}

#[derive(GraphQLInputObject, Clone)]
pub struct RegistrationInput {
    name: String,
    email: String,
    password: String,
}

#[derive(GraphQLObject, Clone)]
pub struct AuthPayload {
    token: String,
    user: User
}

#[derive(GraphQLInputObject, Clone)]
pub struct LoginInput {
    email: String,
    password: String,
}

pub struct Auth;

impl Auth { 
    pub fn login(
        conn: PooledConnection,
        current_user: Option<User>,
        input: LoginInput
        ) -> FieldResult<AuthPayload> {
        use schema::users::dsl::*;
        use schema::sessions::dsl::*;

        // Load user
        let user = users
            .filter(email.eq(input.email))
            .first::<User>(&conn)?;

        // Check password
        match bcrypt::verify(&input.password, &user.password_hash)  {
            Ok(true) => (),
            _ => return Err(FieldError::new("Invalid password", graphql_value!("")))
        }

        // Delete old sessions
        diesel::delete(sessions)
            .filter(uid.eq(user.id))
            .execute(&conn)?;

        // Create a new session
        let hash_bash = format!("{}{}{}", "session", user.id.to_string(), chrono::Utc::now());
        let new_session = NewSession{
            uid: user.id,
            created: chrono::Utc::now().naive_utc(),
            updated: chrono::Utc::now().naive_utc(),
            hash: bcrypt::hash(&hash_bash, bcrypt::DEFAULT_COST)?
        };
        diesel::insert_into(sessions)
            .values(&new_session)
            .execute(&conn)?;

        // Return the auth payload
        Ok(AuthPayload{
            token: new_session.hash,
            user: user
        })
    }

    pub fn register_user(
        conn: PooledConnection,
        current_user: Option<User>,
        input: RegistrationInput 
) -> FieldResult<AuthPayload> {
        use schema::users::dsl::*;
        use schema::sessions::dsl::*;

        // Create user
        let user = diesel::insert_into(users)
            .values(&NewUser{
                name: input.name,
                email: input.email,
                password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?
            })
            .get_result::<User>(&conn)?;

        // Create a new session
        let hash_bash = format!("{}{}{}", "session", user.id.to_string(), chrono::Utc::now());
        let new_session = NewSession{
            uid: user.id,
            created: chrono::Utc::now().naive_utc(),
            updated: chrono::Utc::now().naive_utc(),
            hash: bcrypt::hash(&hash_bash, bcrypt::DEFAULT_COST)?
        };

        let session = diesel::insert_into(sessions)
            .values(&new_session)
            .get_result::<Session>(&conn)?;

        Ok(AuthPayload{
            token: new_session.hash,
            user: user
        })
    }

    pub fn user_from_key(conn: PooledConnection, key: ApiKey) -> Option<User> {
        use schema::users::dsl::*;
        use schema::users::dsl::id;
        use schema::sessions::dsl::*;

        // Load session and user
        let mut user = None;
        let session = sessions 
            .filter(hash.eq(key.0))
            .first::<Session>(&conn).ok();
        if let Some(s) = session { 
            user = users 
                .filter(id.eq(s.uid))
                .first::<User>(&conn).ok();
        }

        user
    }

}
