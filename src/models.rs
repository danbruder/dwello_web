//
// user.rs
//
use schema::{sessions,users};

//
// Users
//
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

//
// Authentication
//
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

#[derive(GraphQLInputObject, Clone)]
pub struct RegistrationInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(GraphQLObject, Clone)]
pub struct AuthPayload {
    pub token: String,
    pub user: User
}

#[derive(GraphQLInputObject, Clone)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
