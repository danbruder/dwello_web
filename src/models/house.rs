//
// models/house.rs
//
use schema::{houses};
use db::{PooledConnection};
use diesel::prelude::*;

#[derive(Identifiable, GraphQLObject, Clone, Queryable)]
#[table_name = "houses"]
pub struct House {
    pub id: i32,
    pub address: String,
    pub lat: String,
    pub lon: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "houses"]
pub struct NewHouse {
    pub address: String,
    pub lat: String,
    pub lon: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(GraphQLObject, Clone)]
pub struct HouseInput { 
    pub address: String,
    pub lat: String,
    pub lon: String,
}
