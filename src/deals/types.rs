//
// deal/types.rs
//
use schema::{deals,houses};
use diesel::prelude::*;
use accounts::types::User;

#[derive(Identifiable, Associations, Clone, Queryable)]
//#[belongs_to(User, foreign_key="bid")]
//#[belongs_to(User, foreign_key="sid")]
//#[belongs_to(House, foreign_key="hid")]
#[table_name = "deals"]
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

graphql_object!(Deal: () |&self| {
    field id() -> i32 { self.id }
    field bid() -> Option<i32> { None }
    field hid() -> Option<i32> { None }
    field sid() -> Option<i32> { None }
    field access_code() -> String { "".to_string() }
    field status() -> String { "".to_string() }
    field created() -> chrono::NaiveDateTime { chrono::Utc::now().naive_utc() }
    field updated() -> chrono::NaiveDateTime { chrono::Utc::now().naive_utc() }
});

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
