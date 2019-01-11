//
// resolvers/auth.rs
//

use auth;
use juniper::{FieldResult,FieldError,Executor};
use diesel::prelude::*;
use graphql::Ctx;
use models::{RegistrationInput,AuthPayload,LoginInput,User,NewUser };

pub fn login(
    executor: &Executor<Ctx>,
    input: LoginInput
) -> FieldResult<AuthPayload> {
    use schema::users::dsl::*;
    let conn = executor.context().pool.get().unwrap();

    // Load user
    let user = users
        .filter(email.eq(input.email))
        .first::<User>(&conn)?;

    // Check password
    // Handle case where user doesn't exist
    match bcrypt::verify(&input.password, &user.password_hash)  {
        Ok(true) => (),
        _ => return Err(FieldError::new("Invalid password", graphql_value!("")))
    }

    // Create a new session
    let session = auth::new_session(conn, &user)?;

    // Return the auth payload
    Ok(AuthPayload{
        token: session.token,
        user: user
    })
}

pub fn register_user(
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
    .get_result::<User>(&conn)?;


    let session = auth::new_session(conn, &user)?;

    Ok(AuthPayload{
        token: session.token,
        user: user
    })
}

