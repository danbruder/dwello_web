//
// models/graphql.rs
//

use validator::{Validate,ValidationErrors};
use error;
use schema::{users,sessions};
use db::{PooledConnection};
use diesel::prelude::*;
use diesel::pg::Pg;
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use validation::ValidationError;


//#[derive(GraphQLInputObject, Clone, Validate)]
//pub struct RegistrationInput {
    //#[validate(length(min = "1", max = "256", message="Cannot be blank"))]
    //pub name: String,
    //#[validate(email(message="Email is not valid"))]
    //pub email: String,
    //#[validate(length(min = "6", max = "30", message="Password length must be between 6 and 30"))]
    //pub password: String,
//}

//#[derive(GraphQLObject, Clone)]
//pub struct AuthPayload {
    //pub token: Option<String>,
    //pub user: Option<User>,
    //pub valid: bool,
    //pub validation_errors: Option<Vec<ValidationError>>
//}

//impl AuthPayload { 
    //pub fn from_validation_errors(e: ValidationErrors) -> AuthPayload { 
        //let errors = error::from_validation_errors(e);
        //AuthPayload{
            //user: None,
            //token: None,
            //valid: false,
            //validation_errors: Some(errors)
        //}
    //}
    //pub fn from_simple_error(key: &'static str, value: &'static str) -> AuthPayload { 
        //AuthPayload{
            //user: None,
            //token: None,
            //valid: false,
            //validation_errors: Some(vec![ValidationError{
                //field: key.to_string(),
                //message: value.to_string()
            //}])
        //}
    //}
//}

//#[derive(GraphQLInputObject, Clone)]
//pub struct LoginInput {
    //pub email: String,
    //pub password: String,
//}

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

#[derive(Serialize, Identifiable,GraphQLObject, Clone, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub roles: Vec<Role>
}

#[derive(Serialize)] 
pub enum CurrentUser {
    Anonymous,
    Authenticated(User),
    Admin(User),
}

impl CurrentUser { 
    pub fn is_admin(&self) -> bool {
        match self { 
            CurrentUser::Admin(_) => true,
            _ => false
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub roles: Vec<Role>
}

/* 
 * Deal status
 */
#[derive(Serialize, Debug, Copy, Clone, GraphQLEnum, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum Role {
    Anonymous,
    Admin
}

impl ToSql<Text, Pg> for Role {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self { 
            Role::Anonymous => out.write_all(b"anonymous")?,
            Role::Admin => out.write_all(b"admin")?
        }

        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Role {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"anonymous" => Ok(Role::Anonymous),
            b"admin" => Ok(Role::Admin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


impl User { 
    pub fn from_key(conn: PooledConnection, key: String) -> Option<User> {
        use schema::users::dsl::*;
        use schema::users::dsl::id;
        use schema::sessions::dsl::*;

        // Load session and user
        let mut user = None;
        let session = sessions 
            .filter(token.eq(key))
            .first::<Session>(&conn).ok();
        if let Some(s) = session { 
            user = users 
                .filter(id.eq(s.uid))
                .first::<User>(&conn).ok();
        }

        user
    }

    /// Check if the user is an admin
    pub fn is_admin(&self) -> bool {
        self.roles
            .iter()
            .any(|r| match r { 
            Role::Admin => true,
            _ => false
        })
    }
}
