#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate juniper;
#[macro_use] extern crate rocket;

extern crate juniper_rocket;

use rocket::State;
use schema::{Schema,create_schema};
use diesel::pg::PgConnection;
use rocket::response::content;

pub mod schema;
pub mod db;

extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/*
 * Rocket routes
 */
#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &())
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema ,&())
}

fn main() {
    rocket::ignite()
        .manage(create_schema())
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
