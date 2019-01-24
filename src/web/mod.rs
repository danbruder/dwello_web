//
// web.rs
//
pub mod cors;

use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use db::{Pool,create_pool,PooledConnection};
use accounts::types::{CurrentUser,User};
use controllers::{viewer};
use accounts::types::CurrentUser::*;
use error::ScoutError;

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ScoutError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        let pool_guard = request.guard::<State<Pool>>();

        let pool = pool_guard.success_or_else(|| return Outcome::Failure((Status::BadRequest, ScoutError::ApiKeyError)));

        let conn = pool.0.get().unwrap();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ScoutError::ApiKeyError)),
            1 if is_valid(keys[0]) => Outcome::Success(user_from_key(conn, keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ScoutError::ApiKeyError)),
            _ => Outcome::Failure((Status::BadRequest, ScoutError::ApiKeyError)),
        }
    }
}

fn is_valid(key: &str) -> bool{
    true
}

fn user_from_key(conn: PooledConnection, key: String) -> CurrentUser {
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
        .manage(Pool(create_pool()))
        .mount(
            "/",
            routes![
                //graphiql, 
                //post_graphql_handler, 
                //post_graphql_cors_handler,
                viewer::user_with_deals,
                //deal::create_deal,
            ],
        )
        .attach(cors::CORS())
        .launch();
}
