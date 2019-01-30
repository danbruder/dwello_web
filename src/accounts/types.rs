use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use schema::{sessions, users};
use std::io::Write;
use validator::Validate;

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

#[derive(Serialize, Debug, Default, Identifiable, GraphQLObject, Clone, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[graphql(skip)]
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Debug)]
pub enum CurrentUser {
    Anonymous,
    Authenticated(User),
    Admin(User),
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub roles: Vec<Role>,
}

/*
* Deal status
*/
#[derive(Serialize, Debug, Copy, Clone, GraphQLEnum, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum Role {
    Anonymous,
    Admin,
}

impl ToSql<Text, Pg> for Role {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Role::Anonymous => out.write_all(b"anonymous")?,
            Role::Admin => out.write_all(b"admin")?,
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

#[derive(Deserialize, Validate)]
pub struct LoginInput {
    #[validate(length(min = "1", max = "256", message = "Cannot be blank"))]
    pub email: String,
    #[validate(length(min = "1", max = "256", message = "Cannot be blank"))]
    pub password: String,
}

/// Input used for registratoin
#[derive(Deserialize, Clone, Validate)]
pub struct RegistrationInput {
    #[validate(length(min = "1", max = "256", message = "Cannot be blank"))]
    pub name: String,
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(
        min = "6",
        max = "30",
        message = "Password length must be between 6 and 30"
    ))]
    pub password: String,
}

#[derive(Serialize, Clone, Default)]
pub struct AuthPayload {
    pub token: Option<String>,
    pub user: Option<User>,
}
