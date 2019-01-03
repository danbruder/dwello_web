#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate juniper;
#[macro_use] extern crate rocket;

extern crate juniper_rocket;

use rocket::State;
use schema::{Schema,create_schema};
use diesel::pg::PgConnection;
use rocket::response::content;

pub mod schema;
pub mod db;
pub mod models;

#[database("scoutql")]
pub struct DbConn(PgConnection);

pub struct Ctx { 
    why: u32,
    conn: DbConn
}

impl juniper::Context for Ctx {}

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    conn: DbConn
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Ctx{ why: 1, conn: conn})
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    conn: DbConn
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Ctx{ why: 1, conn: conn})
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .manage(create_schema())
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
