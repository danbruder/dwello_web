pub mod types;

use self::types::*;
use db::PooledConnection;
use error::ScoutError;
use accounts::types::User;
//use diesel::prelude::*;
//use validator::Validate;
//use diesel::result::Error::DatabaseError;
//use diesel::result::DatabaseErrorKind;

pub fn current(
    conn: PooledConnection,
    current_user: Option<User>,
) -> Result<Viewer, ScoutError> {

    Ok(Viewer{
        user: current_user,
        deals: DealConnection{
            //page_info: PageInfo{
                //end_cursor: "123".to_string(),
                //has_next_page: false
            //},
            total_count: 0,
            edges: vec![]
        }
    })
}

