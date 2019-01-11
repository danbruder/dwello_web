#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate bcrypt;
extern crate serde;
extern crate serde_json;
extern crate dotenv;
extern crate juniper_rocket;
extern crate rocket_contrib;

#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod error;
mod db;
mod resolvers;
mod auth;
mod web;
mod graphql;

fn main() {
    web::launch();
}
