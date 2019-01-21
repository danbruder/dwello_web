//
// models/graphql.rs
//

// A trait that the Validate derive will impl
use validator::{Validate,ValidationErrors};
use error;
use schema::{users,sessions};
use db::{PooledConnection};
use diesel::prelude::*;
use web::ApiKey;
use juniper::{FieldError,FieldResult};



#[derive(GraphQLInputObject, Clone, Validate)]
pub struct RegistrationInput {
    #[validate(length(min = "1", max = "256", message="Cannot be blank"))]
    pub name: String,
    #[validate(email(message="Email %s is not valid"))]
    pub email: String,
    #[validate(length(min = "6", max = "30", message="Password length must be between 6 and 30"))]
    pub password: String,
}

#[derive(GraphQLObject, Clone)]
pub struct AuthPayload {
    pub token: Option<String>,
    pub user: Option<User>,
    pub valid: bool,
    pub validation_errors: Option<Vec<ValidationError>>
}

impl AuthPayload { 
    pub fn from_validation_errors(e: ValidationErrors) -> AuthPayload { 
        let errors = error::from_validation_errors(e);
        AuthPayload{
            user: None,
            token: None,
            valid: false,
            validation_errors: Some(errors)
        }
    }
    pub fn from_simple_error(key: &'static str, value: &'static str) -> AuthPayload { 
        AuthPayload{
            user: None,
            token: None,
            valid: false,
            validation_errors: Some(vec![ValidationError{
                field: key.to_string(),
                message: value.to_string()
            }])
        }
    }
}

#[derive(GraphQLInputObject, Clone)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(GraphQLObject, Clone)]
pub struct ValidationError { 
    pub field: String,
    pub message: String
}

#[derive(GraphQLObject, Clone, Queryable)]
pub struct Session {
    pub id: i32,
    pub uid: i32,
    #[graphql(skip)]
    pub token: String,
    pub active: bool,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub uid: i32,
    pub token: String,
    pub active: bool,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

impl Session {
    pub fn new(conn: PooledConnection, user: &User) -> FieldResult<Session> {
        use schema::sessions::dsl::*;

        // Set old sessions as inactive
        let _ = diesel::update(sessions)
            .filter(uid.eq(user.id))
            .set(active.eq(false))
            .execute(&conn);

        // Create a new session
        let hash_base = format!("{}{}{}", "8h9gfds98f9g9f8dgs98gf98d$5$$%", user.id.to_string(), chrono::Utc::now());
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
}

#[derive(Identifiable,GraphQLObject, Clone, Queryable)]
#[table_name = "users"]
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
