#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate juniper;

use juniper::{EmptyMutation, FieldError, FieldResult, Variables};

/*
* Diesel stuff
*/
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;
use std::env;
pub mod schema;
use self::diesel::prelude::*;
use schema::users;

// Hyper stuff
extern crate juniper_rocket;
#[macro_use] extern crate rocket;

use rocket::response::content;
use rocket::State;

use juniper::RootNode;
use std::sync::Arc;

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

/*
* Juniper stuff
*/

#[derive(GraphQLObject, Clone, Queryable)]
struct User {
    id: i32,
    name: String,
}

struct Query;
struct Mutation;

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

#[derive(GraphQLInputObject, Clone, Insertable)]
#[table_name = "users"]
struct UserInput {
    name: String,
}

graphql_object!(Mutation: Ctx |&self| {
    field create_user(&executor, input: UserInput) -> FieldResult<User> {
        use schema::users::dsl::*;

        let connection = executor.context().db.get().unwrap();
        diesel::insert_into(users)
        .values(&input)
        .get_result(&connection)
            .map_err(|e| {
                FieldError::new("Error inserting user", graphql_value!({"one": "two"}))
            })
    }
});

// Arbitrary context data.

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation>;

struct Ctx {
    db: ConnectionPool,
}


/*
 * Rocket stuff
 */
#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
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
            routes![graphiql, post_graphql_handler],
        )
        .launch();
}
