#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::State;


mod schema;
use schema::{Schema,create_schema};

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
    request.execute(&schema, &())
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
