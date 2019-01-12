//
// web.rs
//
use rocket::response::content;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use db::{Db,create_pool};
use graphql::{Mutation,Query,Ctx,Schema};
use models::user::User;

pub struct ApiKey(pub String);

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
    let user = User::from_key(connection, key);

    // Create new context
    let context = Ctx{
        pool: db.pool.clone(),
        user: user,
    };

    request.execute(&schema, &context)
}

pub fn launch() {
    rocket::ignite()
        .manage(Db { pool: create_pool()})
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

