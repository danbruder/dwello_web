//
// auth.rs
//

use db::{PooledConnection};
use juniper::{FieldResult,FieldError,Executor};
use diesel::prelude::*;
use schema::{sessions};
use user::{User,NewUser};
use super::ApiKey;
use super::Ctx;

#[derive(GraphQLObject, Clone, Queryable)]
pub struct Session {
    id: i32,
    uid: i32,
    #[graphql(skip)]
    token: String,
    active: bool,
    created: chrono::NaiveDateTime,
    updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "sessions"]
struct NewSession {
    uid: i32,
    token: String,
    active: bool,
    created: chrono::NaiveDateTime,
    updated: chrono::NaiveDateTime,
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
        executor: &Executor<Ctx>,
        input: LoginInput
        ) -> FieldResult<AuthPayload> {
        use schema::users::dsl::*;
        let conn = executor.context().pool.get().unwrap();

        // Load user
        let user = users
            .filter(email.eq(input.email))
            .first::<User>(&conn)?;

        // Check password
        // Handle case where user doesn't exist
        match bcrypt::verify(&input.password, &user.password_hash)  {
            Ok(true) => (),
            _ => return Err(FieldError::new("Invalid password", graphql_value!("")))
        }

        // Create a new session
        let session = Auth::new_session(conn, &user)?;

        // Return the auth payload
        Ok(AuthPayload{
            token: session.token,
            user: user
        })
    }

    pub fn register_user(
        executor: &Executor<Ctx>,
        input: RegistrationInput 
) -> FieldResult<AuthPayload> {
        use schema::users::dsl::*;
        let conn = executor.context().pool.get().unwrap();

        // Create user
        let user = diesel::insert_into(users)
            .values(&NewUser{
                name: input.name,
                email: input.email,
                password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?
            })
            .get_result::<User>(&conn)?;


        let session = Auth::new_session(conn, &user)?;

        Ok(AuthPayload{
            token: session.token,
            user: user
        })
    }

    pub fn new_session(conn: PooledConnection, user: &User) -> Result<Session, FieldError> {
        use schema::sessions::dsl::*;

        // Set old sessions as inactive
        // Handle case where no user session exists
        diesel::update(sessions)
            .filter(uid.eq(user.id))
            .set(active.eq(false))
            .execute(&conn)?;

        // Create a new session
        let hash_base = format!("{}{}{}", "session", user.id.to_string(), chrono::Utc::now());
        let new_session = NewSession{
            uid: user.id,
            token: bcrypt::hash(&hash_base, bcrypt::DEFAULT_COST)?,
            active: true,
            created: chrono::Utc::now().naive_utc(),
            updated: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(sessions)
            .values(&new_session)
            .get_result(&conn)
            .map_err(|e| FieldError::from(e))
    }

    pub fn user_from_key(conn: PooledConnection, key: ApiKey) -> Option<User> {
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
