//
// db.rs
//
use rocket::{Outcome, State};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use diesel::pg::PgConnection;
use diesel::r2d2;
use dotenv::dotenv;
use diesel::prelude::*;
use std::env;

pub type ConnectionManager = r2d2::ConnectionManager<PgConnection>;
pub type ConnectionPool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

/// State container for connection pool
pub struct Pool(pub ConnectionPool);
/// Request guard for db connection
pub struct Conn(pub PooledConnection);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.0.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub fn create_pool() -> ConnectionPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

/// Get single connection
pub fn single_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
