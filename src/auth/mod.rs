//
// auth.rs
//

use db::{PooledConnection};
use juniper::{FieldError};
use diesel::prelude::*;
use web::ApiKey;
use models::{User,NewSession,Session};

pub fn new_session(conn: PooledConnection, user: &User) -> Result<Session, FieldError> {
    use schema::sessions::dsl::*;

    // Set old sessions as inactive
    // Handle case where no user session exists
    diesel::update(sessions)
        .filter(uid.eq(user.id))
        .set(active.eq(false))
        .execute(&conn)?;

    // Create a new session
    let hash_base = format!("{}{}{}", "session", user.id.to_string(), chrono::Utc::now());
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

pub fn user_from_key(conn: PooledConnection, key: ApiKey) -> Option<User> {
    use schema::users::dsl::*;
    use schema::users::dsl::id;
    use schema::sessions::dsl::*;

    // Load session and user
    let mut user = None;
    let session = sessions 
        .filter(token.eq(key.0))
        .first::<Session>(&conn).ok();
    if let Some(s) = session { 
        user = users 
            .filter(id.eq(s.uid))
            .first::<User>(&conn).ok();
    }

    user
}
