//
// models/Deal.rs
//
use schema::{deals};
use db::{PooledConnection};
use diesel::prelude::*;

#[derive(Identifiable, GraphQLObject, Clone, Queryable)]
#[table_name = "deals"]
#[belongs_to(User, foreign_key="bid")]
#[belongs_to(User, foreign_key="sid")]
#[belongs_to(House, foreign_key="hid")]
pub struct Deal {
    pub id: i32,
    pub bid: Option<i32>,
    pub hid: Option<i32>,
    pub sid: Option<i32>,
    pub access_code: String,
    pub status: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "deals"]
pub struct NewDeal {
    pub bid: Option<i32>,
    pub hid: Option<i32>,
    pub sid: Option<i32>,
    pub access_code: String,
    pub status: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}
