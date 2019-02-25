//
// accounts/mod.rs
//
pub mod types;

use self::types::*;
use accounts::types::CurrentUser::*;
use db::{Conn, PooledConnection};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use result::Error;
use result::{Payload, Response};
use validator::Validate;

///
/// Helpers
///

/// Create a new session
pub fn create_session(conn: PooledConnection, user: &User) -> Result<Session, Error> {
    use schema::sessions::dsl::*;

    // Set old sessions as inactive
    let _ = diesel::update(sessions)
        .filter(uid.eq(user.id))
        .set(active.eq(false))
        .execute(&conn);

    // Create a new session
    let hash_base = format!(
        "{}{}{}",
        chrono::Utc::now(),
        user.id.to_string(),
        "8h9gfds98f9g9f8dgs98gf98d$5$$%",
    );
    let new_session = NewSession {
        uid: user.id,
        token: bcrypt::hash(&hash_base, bcrypt::DEFAULT_COST)?,
        active: true,
        created: chrono::Utc::now().naive_utc(),
        updated: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(sessions)
        .values(&new_session)
        .get_result(&conn)
        .map_err(|e| Error::from(e))
}

/// Get user from key
pub fn user_from_key(conn: PooledConnection, key: String) -> Option<User> {
    use schema::sessions::dsl::*;
    use schema::users::dsl::id;
    use schema::users::dsl::*;

    // Load session and user
    let mut user = None;
    let session = sessions.filter(token.eq(key)).first::<Session>(&conn).ok();
    if let Some(s) = session {
        user = users.filter(id.eq(s.uid)).first::<User>(&conn).ok();
    }

    user
}

/// Check if the user is an admin
pub fn user_is_admin(user: &User) -> bool {
    user.roles.iter().any(|r| match r {
        Role::Admin => true,
        _ => false,
    })
}

///
/// Public API
///

/// Get all users
pub fn all_users(user: CurrentUser, conn: Conn) -> Response<Vec<User>> {
    use schema::users::dsl::*;

    let _ = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;

    let u = users.limit(10).load::<User>(&conn)?;

    Ok(Payload {
        data: u,
        success: true,
        ..Default::default()
    })
}

/// User by id
pub fn user_by_id(user_id: i32, user: CurrentUser, conn: Conn) -> Response<User> {
    use schema::users::dsl::*;

    let _ = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };
    let Conn(conn) = conn;

    let u = users.find(user_id).first::<User>(&conn)?;

    Ok(Payload {
        data: u,
        success: true,
        ..Default::default()
    })
}

/// Login
pub fn login(conn: Conn, input: LoginInput) -> Response<AuthPayload> {
    use schema::users::dsl::*;

    input.validate()?;
    let Conn(conn) = conn;

    // Load user
    let user = match users.filter(email.eq(&input.email)).first::<User>(&conn) {
        Ok(user) => user,
        Err(_) => {
            // Take hash time to return results
            let _ = bcrypt::verify(&input.email, &input.email);
            return Err(Error::from_custom_validation(
                "email_doesnt_exist",
                "email",
                "Email doesn't exist",
            ));
        }
    };

    // Check password
    // Handle case where user doesn't exist
    let pw_result = bcrypt::verify(&input.password, &user.password_hash).map_err(|_| {
        return Error::from_custom_validation("password_invalid", "password", "Password is invalid");
    })?;

    // If password doesn't match, return error
    if !pw_result {
        return Err(Error::from_custom_validation(
            "password_invalid",
            "password",
            "Password is invalid",
        ));
    }

    // Create a new session
    let session = self::create_session(conn, &user)?;

    // Return the auth payload
    Ok(Payload {
        data: AuthPayload {
            token: Some(session.token),
            user: Some(user),
        },
        success: true,
        ..Default::default()
    })
}
/// Register
pub fn register(conn: Conn, input: RegistrationInput) -> Response<AuthPayload> {
    use schema::users::dsl::*;

    input.validate()?;

    let input = input.clone();

    let Conn(conn) = conn;

    // Create user
    let user = diesel::insert_into(users)
        .values(&NewUser {
            name: input.name,
            email: input.email,
            password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?,
            roles: vec![Role::Authenticated],
        })
        .get_result::<User>(&conn)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => {
                Error::from_custom_validation("email_taken", "email", "Email is taken")
            }
            _ => Error::from(e),
        })?;

    let session = self::create_session(conn, &user)?;

    Ok(Payload {
        data: AuthPayload {
            token: Some(session.token),
            user: Some(user),
        },
        success: true,
        ..Default::default()
    })
}

/// Create user
///
/// Used by admins to create a user
pub fn create_user(user: CurrentUser, conn: Conn, input: CreateUserInput) -> Response<User> {
    use schema::users::dsl::*;

    let _ = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };

    input.validate()?;

    // Create user
    let user = diesel::insert_into(users)
        .values(&NewUser {
            name: input.name,
            email: input.email,
            password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?,
            roles: input.roles,
        })
        .get_result::<User>(&conn.0)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => {
                Error::from_custom_validation("email_taken", "email", "Email is taken")
            }
            _ => Error::from(e),
        })?;

    Ok(Payload {
        data: user,
        success: true,
        ..Default::default()
    })
}

//
// Profiles
//
/// Create a new session
pub fn create_profile(
    conn: Conn,
    user: &CurrentUser,
    profile_user_id: i32,
    input: &ProfileInput,
) -> Response<Profile> {
    use schema::profiles::dsl::*;

    let _ = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };

    input.validate()?;

    // Create user
    let profile = diesel::insert_into(profiles)
        .values(&NewProfile {
            uid: profile_user_id,
            title: input.title.clone(),
            intro: input.intro.clone(),
            body: input.body.clone(),
        })
        .get_result::<Profile>(&conn.0)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => {
                Error::from_custom_validation("profile_exists", "profile", "Profile exists")
            }
            _ => Error::from(e),
        })?;

    Ok(Payload {
        data: profile,
        success: true,
        ..Default::default()
    })
}

/// Update Profile
pub fn update_profile(
    conn: Conn,
    user: &CurrentUser,
    profile_user_id: i32,
    input: &ProfileInput,
) -> Response<Profile> {
    use schema::profiles::dsl::*;

    let _ = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };

    input.validate()?;

    let profile = profiles
        .filter(uid.eq(profile_user_id))
        .first::<Profile>(&conn.0)?;

    let profile = diesel::update(&profile).set(input).get_result(&conn.0)?;

    Ok(Payload {
        data: profile,
        success: true,
        ..Default::default()
    })
}

/// Get profile
pub fn get_profile(user_id: i32, user: CurrentUser, conn: Conn) -> Response<Profile> {
    use schema::profiles::dsl::*;

    let _ = match user {
        Admin(user) => user,
        _ => return Err(Error::AccessDenied),
    };

    let profile = profiles.filter(uid.eq(user_id)).first::<Profile>(&conn.0)?;

    Ok(Payload {
        data: profile,
        success: true,
        ..Default::default()
    })
}
