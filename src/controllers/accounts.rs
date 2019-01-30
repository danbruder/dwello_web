use accounts;
use accounts::types::*;
use db::Conn;
use rocket_contrib::json::Json;
use web::ApiResponse;

/// Get all users
#[get("/users")]
pub fn all_users(user: CurrentUser, conn: Conn) -> ApiResponse<Vec<User>> {
    accounts::all_users(user, conn).map(|r| Json(r))
}

/// Get user by id
#[get("/users/<user_id>")]
pub fn user_by_id(user_id: i32, user: CurrentUser, conn: Conn) -> ApiResponse<User> {
    accounts::user_by_id(user_id, user, conn).map(|r| Json(r))
}

/// Login
#[post("/login", format = "application/json", data = "<input>")]
pub fn login(conn: Conn, input: Json<LoginInput>) -> ApiResponse<AuthPayload> {
    accounts::login(conn, input.into_inner()).map(|r| Json(r))
}

/// Register route
#[post("/register", format = "application/json", data = "<input>")]
pub fn register(conn: Conn, input: Json<RegistrationInput>) -> ApiResponse<AuthPayload> {
    accounts::register(conn, input.into_inner()).map(|r| Json(r))
}
