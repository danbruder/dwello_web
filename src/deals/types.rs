//
// deal/types.rs
//
use schema::{deals,houses};
use diesel::prelude::*;
use diesel::pg::Pg;
//use accounts::types::User;
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Varchar;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;


#[derive(Serialize, Debug, Copy, Clone, GraphQLEnum, AsExpression, FromSqlRow)]
#[sql_type = "Varchar"]
pub enum DealStatus {
    Initialized
}

impl ToSql<Varchar, Pg> for DealStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self { 
            DealStatus::Initialized => out.write_all(b"initialized")?
        }

        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Pg> for DealStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"initialized" => Ok(DealStatus::Initialized),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Serialize, Identifiable, GraphQLObject, Associations, Clone, Queryable)]
//#[belongs_to(User, foreign_key="bid")]
//#[belongs_to(User, foreign_key="sid")]
//#[belongs_to(House, foreign_key="hid")]
#[table_name = "deals"]
pub struct Deal {
    pub id: i32,
    pub buyer_id: Option<i32>,
    pub seller_id: Option<i32>,
    pub house_id: Option<i32>,
    pub access_code: String,
    pub status: DealStatus,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "deals"]
pub struct NewDeal {
    pub buyer_id: Option<i32>,
    pub seller_id: Option<i32>,
    pub house_id: Option<i32>,
    pub access_code: String,
    pub status: DealStatus,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Serialize, Identifiable, GraphQLObject, Clone, Queryable)]
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

#[derive(GraphQLInputObject, Clone)]
pub struct HouseInput { 
    pub address: String,
    pub lat: String,
    pub lon: String,
}
