#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate bcrypt;
extern crate serde;
extern crate serde_json;
extern crate dotenv;
extern crate juniper_rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate juniper;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use juniper::{ FieldError, FieldResult };
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::result::{DatabaseErrorKind};
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use rocket::response::content;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

mod schema;
use schema::users;

pub type ConnectionManager = r2d2::ConnectionManager<PgConnection>;
pub type ConnectionPool = r2d2::Pool<ConnectionManager>;

pub fn db_pool() -> ConnectionPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

struct Query;
struct Mutation;

/*
* Juniper stuff
*/
#[derive(GraphQLObject, Clone, Queryable)]
struct User {
    id: i32,
    name: String,
    email: String,
    #[graphql(skip)]
    password_hash: String,
}

#[derive(GraphQLInputObject, Clone)]
struct RegistrationInput {
    name: String,
    email: String,
    password: String,
}

#[derive(GraphQLInputObject, Clone)]
struct LoginInput {
    email: String,
    password: String,
}

#[derive(GraphQLObject, Clone)]
struct AuthPayload {
    token: String,
    user: User
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    name: String,
    email: String,
    password_hash: String,
}

graphql_object!(Query: Ctx |&self| {
    field all_users(&executor) -> FieldResult<Vec<User>> {
        use schema::users::dsl::*;

        let connection = executor.context().db.get().unwrap();
        users
            .limit(10)
            .load::<User>(&connection)
            .map_err(|e| {
                FieldError::new("No users", graphql_value!({"one": "two"}))
            })
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field login(&executor, input: LoginInput) -> FieldResult<AuthPayload> {
        use schema::users::dsl::*;
        let connection = executor.context().db.get().unwrap();

        // Load user
        let user = users
            .filter(email.eq(input.email))
            .first::<User>(&connection)?;

        // Create a session
        // Create a token
        Ok(AuthPayload{
            token: "valid_api_key".to_string(),
            user: user
        })
    }

    field register_user(&executor, input: RegistrationInput) -> FieldResult<User> {
        use schema::users::dsl::*;

        let hash = bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?;
        let new_user = NewUser{
            name: input.name,
            email: input.email,
            password_hash: hash
        };

        let connection = executor.context().db.get().unwrap();
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(&connection)
            .map_err(|e| {
                match e {
                    diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => FieldError::new("address is already taken", graphql_value!({"email": "address is already taken"})),
                    _ => FieldError::new("Registration error", graphql_value!(""))

                }
            })
    }
});

type Schema = juniper::RootNode<'static, Query, Mutation>;

struct Ctx {
    db: ConnectionPool,
}


/*
 * Rocket stuff
 */


struct ApiKey(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == "valid_api_key"
}

#[derive(Debug)]
enum ApiKeyError {
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
    context: State<Ctx>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Ctx{ db: db_pool()})
        .manage(Schema::new(
                Query, 
                Mutation
        ))
        .mount(
            "/",
            routes![graphiql, post_graphql_handler ],
        )
        .launch();
}
