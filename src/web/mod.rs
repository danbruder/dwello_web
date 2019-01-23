//
// web.rs
//
pub mod cors;

use rocket::response::content;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use db::{Db,create_pool,PooledConnection};
use graphql::{Mutation,Query,Ctx,Schema};
use accounts::types::{CurrentUser,User};
use controllers::{viewer,deal};
use accounts::types::CurrentUser::*;
use error::ScoutError;
use error::ApiKeyError;


pub struct ApiKey(pub String);

fn is_valid(_key: &str) -> bool {
    true
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

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ScoutError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        let db = request.guard::<State<Db>>().unwrap();

        let conn = db.pool.get().unwrap();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ScoutError::ApiKeyError(ApiKeyError::Missing))),
            1 if is_valid(keys[0]) => Outcome::Success(user_from_key(conn, ApiKey(keys[0].to_string()))),
            1 => Outcome::Failure((Status::BadRequest, ScoutError::ApiKeyError(ApiKeyError::Invalid))),
            _ => Outcome::Failure((Status::BadRequest, ScoutError::ApiKeyError(ApiKeyError::BadCount))),
        }
    }
}

fn user_from_key(conn: PooledConnection, key: ApiKey) -> CurrentUser {
    User::from_key(conn, key)
        .map_or(Anonymous, |u| match u.is_admin() {
            true => Admin(u),
            false => Authenticated(u)
        })
}


//#[get("/graphql/explorer")]
//fn graphiql() -> content::Html<String> {
    //juniper_rocket::graphiql_source("/graphql")
//}

//#[options("/graphql")]
//fn post_graphql_cors_handler() -> content::Plain<String> { 
    //content::Plain("".to_string())
//}

//#[post("/graphql", data = "<request>")]
//fn post_graphql_handler(
    //key: ApiKey,
    //db: State<Db>,
    //request: juniper_rocket::GraphQLRequest,
    //schema: State<Schema>,
//) -> juniper_rocket::GraphQLResponse {
    //let connection = db.pool.get().unwrap();
    //let user = User::from_key(connection, key);

    //// Create new context
    //let context = Ctx{
        //pool: db.pool.clone(),
        //user: user,
    //};

    //request.execute(&schema, &context)
//}

pub fn launch() {
    rocket::ignite()
        .manage(Db { pool: create_pool()})
        .manage(Schema::new(
                Query, 
                Mutation
        ))
        .mount(
            "/",
            routes![
                graphiql, 
                post_graphql_handler, 
                post_graphql_cors_handler,
                viewer::user_with_deals,
                deal::create_deal,
            ],
        )
        .attach(cors::CORS())
        .launch();
}
