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

graphql_object!(Viewer: () |&self| {
    field deals(&executor, filter: Option<DealFilter>) -> &DealConnection { &self.deals }
});

graphql_object!(DealConnection: () |&self| {
    //field page_info() -> FieldResult<PageInfo> { Ok(self.page_info) }
    field total_count() -> &i32 { &self.total_count }
    field edges() -> &Vec<DealEdge> { &self.edges }
});

