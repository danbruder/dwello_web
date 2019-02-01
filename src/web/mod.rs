//
// web.rs
//
pub mod controllers;
pub mod cors;
pub mod error;
pub mod guards;
pub mod types;

use self::controllers::*;
use db::{create_pool, Pool};
use rocket::Rocket;

pub fn build() -> Rocket {
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
                // deal::deals_with_houses,
            ],
        )
        .attach(cors::CORS())
}

pub fn launch() {
    build().launch();
}
