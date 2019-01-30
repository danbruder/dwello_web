#![feature(decl_macro, proc_macro_hygiene, custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate bcrypt;
extern crate dotenv;
extern crate juniper_rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate validator;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate serde_derive;

mod accounts;
mod db;
mod deals;
mod result;
mod schema;
mod web;

fn main() {
    web::launch();
}
