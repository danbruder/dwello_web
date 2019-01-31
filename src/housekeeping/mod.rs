use accounts::types::*;
use db::single_connection;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use result::Error;
use schema::users::dsl::*;

pub fn run() -> Result<(), Error> {
    println!("Running housekeeping job");
    let conn = single_connection();

    // TODO: Remove this in favor of a more secure approach.
    // Create user if not created already
    let user = diesel::insert_into(users)
        .values(&NewUser {
            name: "admin".to_owned(),
            email: "admin@dwelloapp.com".to_owned(),
            password_hash: bcrypt::hash("Ix3aNGjUlv71xI4", bcrypt::DEFAULT_COST)?,
            roles: vec![Role::Admin],
        })
        .get_result::<User>(&conn)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => {
                Error::from_custom_validation("email_taken", "email", "Email is taken")
            }
            _ => Error::from(e),
        })?;

    println!("{} created.", user.name);
    Ok(())
}
