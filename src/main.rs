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
extern crate futures;
extern crate hyper;
extern crate juniper_hyper;
extern crate pretty_env_logger;
use futures::future;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Method;
use hyper::{Body, Response, Server, StatusCode};
use juniper::tests::model::Database;
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

fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 3000).into();

    let root_node = Arc::new(Schema::new(Query, Mutation));
    let ctx = Arc::new(Ctx { db: db_pool() });

    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = ctx.clone();
        service_fn(move |req| -> Box<Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
                (&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                (&Method::POST, "/graphql") => {
                    Box::new(juniper_hyper::graphql(root_node, ctx, req))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}
