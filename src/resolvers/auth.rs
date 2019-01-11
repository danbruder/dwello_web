//
// resolvers/auth.rs
//

use auth;
use juniper::{FieldResult,Executor,IntoFieldError};
use diesel::prelude::*;
use graphql::Ctx;
use models::{RegistrationInput,AuthPayload,LoginInput,User,NewUser};
use error::ScoutError::{InvalidEmail,InvalidPassword, DbError};

/*
 * Login
 */
pub fn login(
    executor: &Executor<Ctx>,
    input: LoginInput
) -> FieldResult<AuthPayload> {
    use schema::users::dsl::*;
    let conn = executor.context().pool.get().unwrap();

    // Load user
    let user = users
        .filter(email.eq(&input.email))
        .first::<User>(&conn)
        .map_err(|_| {
            let _ = bcrypt::verify(&input.email, "hash the email to protect against timing attacks");
            InvalidEmail.into_field_error()
        })?;

    // Check password
    // Handle case where user doesn't exist
    match bcrypt::verify(&input.password, &user.password_hash)  {
        Ok(true) => (),
        _ => return Err(InvalidPassword.into_field_error())
    }

    // Create a new session
    let session = auth::new_session(conn, &user)?;

    // Return the auth payload
    Ok(AuthPayload{
        token: session.token,
        user: user
    })
}

/*
 * Register user
 */
pub fn register(
    executor: &Executor<Ctx>,
    input: RegistrationInput 
) -> FieldResult<AuthPayload> {
    use schema::users::dsl::*;
    let conn = executor.context().pool.get().unwrap();

    // Create user
    let user = diesel::insert_into(users)
        .values(&NewUser{
            name: input.name,
            email: input.email,
            password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?
        })
        .get_result::<User>(&conn)
        .map_err(|e| DbError(e).into_field_error())?;


    let session = auth::new_session(conn, &user)?;

    Ok(AuthPayload{
        token: session.token,
        user: user
    })
}

