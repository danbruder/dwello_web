//
// models/session.rs
//
use db::{PooledConnection};
use juniper::{FieldError,FieldResult};
use diesel::prelude::*;
use models::user::{User};
use schema::{sessions};

#[derive(GraphQLObject, Clone, Queryable)]
pub struct Session {
    pub id: i32,
    pub uid: i32,
    #[graphql(skip)]
    pub token: String,
    pub active: bool,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub uid: i32,
    pub token: String,
    pub active: bool,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

impl Session {
    pub fn new(conn: PooledConnection, user: &User) -> FieldResult<Session> {
        use schema::sessions::dsl::*;

        // Set old sessions as inactive
        let _ = diesel::update(sessions)
            .filter(uid.eq(user.id))
            .set(active.eq(false))
            .execute(&conn);

        // Create a new session
        let hash_base = format!("{}{}{}", "8h9gfds98f9g9f8dgs98gf98d$5$$%", user.id.to_string(), chrono::Utc::now());
        let new_session = NewSession{
            uid: user.id,
            token: bcrypt::hash(&hash_base, bcrypt::DEFAULT_COST)?,
            active: true,
            created: chrono::Utc::now().naive_utc(),
            updated: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(sessions)
            .values(&new_session)
            .get_result(&conn)
            .map_err(|e| FieldError::from(e))
    }
}
