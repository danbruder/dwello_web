use accounts::types::*;
use accounts::types::CurrentUser::*;
use db::Conn;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use error::Error;
use rocket_contrib::json::Json;
use validator::Validate;

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Validate)]
pub struct RegistrationInput {
    #[validate(length(min = "1", max = "256", message = "Cannot be blank"))]
    pub name: String,
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(
        min = "6",
        max = "30",
        message = "Password length must be between 6 and 30"
    ))]
    pub password: String,
}

#[derive(Serialize, Clone)]
pub struct AuthPayload {
    pub token: Option<String>,
    pub user: Option<User>,
}


#[get("/users")]
pub fn all_users(user: CurrentUser, conn: Conn) -> Result<Json<Vec<User>>, Error> {
    use schema::users::dsl::*;

    let user = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;

    let u = users
        .limit(10)
        .load::<User>(&conn)?;

    Ok(Json(u))
}

/// Login
#[post("/login", format = "application/json", data = "<input>")]
pub fn login(conn: Conn, input: Json<LoginInput>) -> Result<Json<AuthPayload>, Error> {
    use schema::users::dsl::*;

    let Conn(conn) = conn;

    // Load user
    let user = match users.filter(email.eq(&input.email)).first::<User>(&conn) {
        Ok(user) => user,
        Err(_) => {
            // Make sure it costs something if there is no user to
            // prevent timing attacks
            let _ = bcrypt::verify(&input.email, "hash the email");
            return Err(Error::EmailDoesntExist);
        }
    };

    // Check password
    // Handle case where user doesn't exist
    bcrypt::verify(&input.password, &user.password_hash)
        .map_err(|_| return Error::PasswordNoMatch)?;

    // Create a new session
    let session = Session::new(conn, &user)?;

    // Return the auth payload
    Ok(Json(AuthPayload {
        token: Some(session.token),
        user: Some(user),
    }))
}

/// Login
#[post("/register", format = "application/json", data = "<input>")]
pub fn register(conn: Conn, input: Json<RegistrationInput>) -> Result<Json<AuthPayload>, Error> {
    use schema::users::dsl::*;

    input.validate().map_err(|e| Error::InputError(e))?;

    let input = input.clone();

    let Conn(conn) = conn;

    // Create user
    let user = diesel::insert_into(users)
        .values(&NewUser {
            name: input.name,
            email: input.email,
            password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?,
            roles: vec![Role::Admin],
        })
        .get_result::<User>(&conn)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => Error::EmailTaken,
            _ => Error::from(e),
        })?;

    let session = Session::new(conn, &user)?;

    Ok(Json(AuthPayload {
        token: Some(session.token),
        user: Some(user),
    }))
}
