#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate bcrypt;
extern crate serde;
extern crate serde_json;
extern crate dotenv;
extern crate juniper_rocket;
extern crate rocket_contrib;

#[macro_use] extern crate juniper;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use juniper::{ FieldResult };
use rocket::response::content;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

mod schema;
mod user;
mod error;
mod db;
mod auth;
use user::{User};
use db::{Db, ConnectionPool };
use auth::{Auth, LoginInput, AuthPayload, RegistrationInput};

struct Query;
struct Mutation;

/*
* Juniper stuff
*/
graphql_object!(Query: Ctx |&self| {
    field all_users(&executor) -> FieldResult<Vec<User>> {
        User::all_users(executor)
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field login(&executor, input: LoginInput) -> FieldResult<AuthPayload> {
        Auth::login(executor, input)
    }

    field register_user(&executor, input: RegistrationInput) -> FieldResult<AuthPayload> {
        Auth::register_user(executor, input)
    }
});

type Schema = juniper::RootNode<'static, Query, Mutation>;

pub struct Ctx {
    user: Option<User>,
    pool: ConnectionPool
}

/*
 * Rocket stuff
 */
pub struct ApiKey(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(_key: &str) -> bool {
    true
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}


#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    key: ApiKey,
    db: State<Db>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    let connection = db.pool.get().unwrap();
    let user = Auth::user_from_key(connection, key);

    // Create new context
    let context = Ctx{
        pool: db.pool.clone(),
        user: user,
    };

    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Db { pool: db::create_pool()})
        .manage(Schema::new(
                Query, 
                Mutation
        ))
        .mount(
            "/",
            routes![graphiql, post_graphql_handler],
        )
        .launch();
}

