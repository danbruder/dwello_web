//
// resolvers/auth.rs
//
use validator::Validate;
use juniper::{FieldResult,FieldError,Executor};
use diesel::prelude::*;
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind;
use graphql::Ctx;
use models::graphql::{RegistrationInput,AuthPayload,LoginInput,ValidationError};
use models::user::{User,NewUser};
use models::session::Session;


pub fn login(
    executor: &Executor<Ctx>,
    input: LoginInput
) -> FieldResult<AuthPayload> {
    use schema::users::dsl::*;
    let conn = executor.context().pool.get().unwrap();

    // Load user
    let user = match users
        .filter(email.eq(&input.email))
        .first::<User>(&conn) {
        Ok(user) => user,
        Err(_) => {
            // Make sure it costs something if there is no user to 
            // prevent timing attacks
            let _ = bcrypt::verify(&input.email, "hash the email");
            return Ok(AuthPayload{
                token: None,
                user: None,
                valid: false,
                validation_errors: Some(vec![ValidationError{
                    field: "email".to_string(),
                    message: "Invalid email".to_string()
                }])
            })
        }
    };

    // Check password
    // Handle case where user doesn't exist
    match bcrypt::verify(&input.password, &user.password_hash)  {
        Ok(true) => (),
        _ => return Ok(AuthPayload{
            token: None,
            user: None,
            valid: false,
            validation_errors: Some(vec![ValidationError{
                field: "password".to_string(),
                message: "Password does not match".to_string()
            }])
        })
    }

    // Create a new session
    let session = Session::new(conn, &user)?;

    // Return the auth payload
    Ok(AuthPayload{
        token: Some(session.token),
        user: Some(user),
        valid: true,
        validation_errors: None
    })
}

pub fn register(
    executor: &Executor<Ctx>,
    input: RegistrationInput 
) -> FieldResult<AuthPayload> {
    use schema::users::dsl::*;
    let conn = executor.context().pool.get().unwrap();

    match input.validate() {
        Err(e) => {
            return Ok(AuthPayload::from_validation_errors(e))
        },
        Ok(_) => ()
    }

    // Create user
    let user = match diesel::insert_into(users) 
        .values(&NewUser{
            name: input.name,
            email: input.email,
            password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?
        }).get_result::<User>(&conn) {
        Ok(user) => user,
        Err(err) => match err {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => return Ok(AuthPayload::from_simple_error("email", "Email is taken")),
            _ => return Err(FieldError::from(err))
        }
    };


    let session = Session::new(conn, &user)?;

    Ok(AuthPayload{
        token: Some(session.token),
        user: Some(user),
        valid: true,
        validation_errors: None
    })
}

