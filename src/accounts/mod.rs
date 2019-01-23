pub mod types;

//use diesel::prelude::*;
//use self::types::*;
//use db::PooledConnection;
//use error::ScoutError;
//use validator::Validate;
//use diesel::result::Error::DatabaseError;
//use diesel::result::DatabaseErrorKind;

//pub fn login(
    //conn: PooledConnection,
    //input: LoginInput
//) -> Result<AuthPayload, ScoutError> {
    //use schema::users::dsl::*;

    //// Load user
    //let user = match users
        //.filter(email.eq(&input.email))
        //.first::<User>(&conn) {
        //Ok(user) => user,
        //Err(_) => {
            //// Make sure it costs something if there is no user to 
            //// prevent timing attacks
            //let _ = bcrypt::verify(&input.email, "hash the email");
            //return Ok(AuthPayload{
                //token: None,
                //user: None,
                //valid: false,
                //validation_errors: Some(vec![ValidationError{
                    //field: "email".to_string(),
                    //message: "Invalid email".to_string()
                //}])
            //})
        //}
    //};

    //// Check password
    //// Handle case where user doesn't exist
    //match bcrypt::verify(&input.password, &user.password_hash)  {
        //Ok(true) => (),
        //_ => return Ok(AuthPayload{
            //token: None,
            //user: None,
            //valid: false,
            //validation_errors: Some(vec![ValidationError{
                //field: "password".to_string(),
                //message: "Password does not match".to_string()
            //}])
        //})
    //}

    //// Create a new session
    //let session = Session::new(conn, &user)?;

    //// Return the auth payload
    //Ok(AuthPayload{
        //token: Some(session.token),
        //user: Some(user),
        //valid: true,
        //validation_errors: None
    //})
//}

//pub fn register(
    //conn: PooledConnection,
    //input: RegistrationInput 
//) -> Result<AuthPayload, ScoutError> {
    //use schema::users::dsl::*;

    //match input.validate() {
        //Err(e) => {
            //return Ok(AuthPayload::from_validation_errors(e))
        //},
        //Ok(_) => ()
    //}

    //// Create user
    //let user = match diesel::insert_into(users) 
        //.values(&NewUser{
            //name: input.name,
            //email: input.email,
            //password_hash: bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)?,
            //roles: vec![Role::Admin]
        //}).get_result::<User>(&conn) {
        //Ok(user) => user,
        //Err(err) => match err {
            //DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => return Ok(AuthPayload::from_simple_error("email", "Email is taken")),
            //_ => return Err(ScoutError::from(err))
        //}
    //};

    //let session = Session::new(conn, &user)?;

    //Ok(AuthPayload{
        //token: Some(session.token),
        //user: Some(user),
        //valid: true,
        //validation_errors: None
    //})
//}

//pub fn all_users(
    //conn: PooledConnection,
    //current_user: Option<User>
    //) -> Result<Vec<User>, ScoutError> {
    //use schema::users::dsl::*;

    //if current_user.is_none() { 
        //return Err(ScoutError::AccessDenied);
    //}

    //users
        //.limit(10)
        //.load::<User>(&conn)
        //.map_err(|e| ScoutError::from(e))
//}

//impl Session {
    //pub fn new(conn: PooledConnection, user: &User) -> Result<Session, ScoutError> {
        //use schema::sessions::dsl::*;

        //// Set old sessions as inactive
        //let _ = diesel::update(sessions)
            //.filter(uid.eq(user.id))
            //.set(active.eq(false))
            //.execute(&conn);

        //// Create a new session
        //let hash_base = format!("{}{}{}", "8h9gfds98f9g9f8dgs98gf98d$5$$%", user.id.to_string(), chrono::Utc::now());
        //let new_session = NewSession{
            //uid: user.id,
            //token: bcrypt::hash(&hash_base, bcrypt::DEFAULT_COST)?,
            //active: true,
            //created: chrono::Utc::now().naive_utc(),
            //updated: chrono::Utc::now().naive_utc(),
        //};

        //diesel::insert_into(sessions)
            //.values(&new_session)
            //.get_result(&conn)
            //.map_err(|e| ScoutError::from(e))
    //}
//}

