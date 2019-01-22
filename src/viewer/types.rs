//
// models/graphql.rs
//

use juniper::FieldResult;
use graphql::Ctx;
use diesel::prelude::*;

use accounts::types::User;
use deals::types::Deal;

#[derive(Clone)]
pub struct DealConnection { 
    //pub page_info: PageInfo, 
    pub total_count: i32,
    pub edges: Vec<DealEdge>
}

#[derive(GraphQLObject, Clone)]
pub struct DealEdge {
    pub node: Deal,
    pub cursor: String
}

#[derive(GraphQLObject, Clone)]
pub struct PageInfo {
    pub end_cursor: String,
    pub has_next_page: bool
}

#[derive(Clone)]
pub struct Viewer {
    pub user: Option<User>,
    pub deals: DealConnection
}

#[derive(GraphQLInputObject, Clone)]
pub struct DealFilter { 
    limit: i32
}

graphql_object!(Viewer: Ctx |&self| {
    field deals(&executor, filter: DealFilter) -> &DealConnection { 
        // Get diesel query and join the deals
        let conn = executor.context().pool.get().unwrap();
        println!("{}", "in deals query".to_string());
        use schema::deals::dsl::*;

        let deal_results = deals
            .filter(buyer_id.eq(&self.user.unwrap().id))
            .load::<Vec<Deal>>(&conn).unwrap_or(vec![]);

        &DealConnection{
            total_count: 0,
            edges: vec![]
        }
    }
});

graphql_object!(DealConnection: () |&self| {
    //field page_info() -> FieldResult<PageInfo> { Ok(self.page_info) }
    field total_count() -> &i32 { &self.total_count }
    field edges() -> &Vec<DealEdge> { &self.edges }
});

