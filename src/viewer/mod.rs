pub mod types;
           use juniper::LookAheadMethods;

use self::types::*;
use db::PooledConnection;
use error::ScoutError;
use accounts::types::User;
use juniper::LookAheadSelection;
use diesel::prelude::*;
use deals::types::Deal;
use validator::Validate;
use diesel::result::Error::DatabaseError;
use diesel::result::DatabaseErrorKind;

pub fn current(
    conn: PooledConnection,
    current_user: Option<User>,
    look_ahead: LookAheadSelection<juniper::DefaultScalarValue>
) -> Result<Viewer, ScoutError> {
    use schema::deals::dsl::*;
    use schema::houses::dsl::*;
    use schema::houses::dsl::id;
    
    if current_user.is_none() {
        return Err(ScoutError::AccessDenied);
    }

    let current_user = current_user.unwrap();
     
    let deal_selection = look_ahead.select_child("deals");
    let deal_args = deal_selection.map(|d| d.arguments());
    let limit = deal_args
        .map(|a| a
             .iter()
             .find(|a| a.item.field_name() == "limit")
             .map(|a| a.value())
         );

    println!("{:?}", limit);


    let mut deal_results = vec![];
    match deal_selection { 
        Some(s) => {
            deal_results = deals
                .filter(buyer_id.eq(&current_user.id))
                .limit(10)
                .load::<Deal>(&conn)
                .unwrap_or(vec![])
                .iter()
                .map(|e| DealEdge{
                    node: e.clone(), 
                    cursor: "1".to_string()
                })
            .collect(); 

        }
        None => ()
    }


            

    Ok(Viewer{
        user: Some(current_user),
        deals: DealConnection{
            //page_info: PageInfo{
                //end_cursor: "123".to_string(),
                //has_next_page: false
            //},
            total_count: 0,
            edges: deal_results
        }
    })
}

