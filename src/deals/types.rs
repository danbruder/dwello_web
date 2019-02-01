//
// deal/types.rs
//
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Varchar;
use schema::{deals, houses};
use std::io::Write;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Varchar"]
pub enum DealStatus {
    Initialized,
    MailerSent,
}

impl ToSql<Varchar, Pg> for DealStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            DealStatus::Initialized => out.write_all(b"initialized")?,
            DealStatus::MailerSent => out.write_all(b"mailer_sent")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Pg> for DealStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"initialized" => Ok(DealStatus::Initialized),
            b"mailer_sent" => Ok(DealStatus::MailerSent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl Default for DealStatus {
    fn default() -> Self {
        DealStatus::Initialized
    }
}

#[derive(Deserialize, Serialize, Identifiable, Associations, Clone, Queryable, AsChangeset)]
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
    pub title: String,
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
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateDeal {
    pub status: Option<DealStatus>,
}

#[derive(Serialize, Identifiable, Clone, Queryable)]
#[table_name = "houses"]
pub struct House {
    pub id: i32,
    pub address: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "houses"]
pub struct NewHouse {
    pub address: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct HouseInput {
    pub address: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateDealAndHouseInput {
    pub buyer_id: i32,
    #[validate(length(min = "1", max = "500", message = "Cannot be blank"))]
    pub address: String,
}

#[derive(FromForm, Deserialize)]
pub struct DealsQuery {
    pub buyer_id: Option<i32>,
}
