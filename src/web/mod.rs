//
// web.rs
//
pub mod cors;

use accounts::types::CurrentUser::*;
use accounts::types::{CurrentUser, User};
use controllers::{accounts, deal};
use db::{create_pool, Pool, PooledConnection};
use error::Error;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;

#[derive(Serialize)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub total_count: u32,
}

#[derive(Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Serialize, Default)]
pub struct ApiData<T> {
    pub data: T,
    pub success: bool,
    pub error_message: Option<String>,
    pub validation_errors: Option<Vec<ValidationError>>,
    pub page_info: Option<PageInfo>,
}

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();

        let pool = match request.guard::<State<Pool>>() {
            Outcome::Success(s) => s,
            _ => return Outcome::Failure((Status::BadRequest, Error::ServiceUnavailable)),
        };

        let conn = pool.0.get().unwrap();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, Error::ApiKeyError)),
            1 if is_valid(keys[0]) => Outcome::Success(user_from_key(conn, keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, Error::ApiKeyError)),
            _ => Outcome::Failure((Status::BadRequest, Error::ApiKeyError)),
        }
    }
}

fn is_valid(_key: &str) -> bool {
    true
}

fn user_from_key(conn: PooledConnection, key: String) -> CurrentUser {
    println!("In user frm key!, {}", key);
    User::from_key(conn, key).map_or(Anonymous, |u| match u.is_admin() {
        true => Admin(u),
        false => Authenticated(u),
    })
}

pub fn launch() {
    rocket::ignite()
        .manage(Pool(create_pool()))
        .mount(
            "/",
            routes![
                cors::cors,
                accounts::login,
                accounts::register,
                accounts::all_users,
                accounts::user_by_id,
                deal::create_deal,
                deal::get_deals,
                deal::update_deal,
                deal::deals_with_houses,
            ],
        )
        .attach(cors::CORS())
        .launch();
}
