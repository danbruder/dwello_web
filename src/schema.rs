#![allow(proc_macro_derive_resolution_fallback)]

use juniper::{FieldResult, RootNode};

use super::establish_connection;
use diesel::prelude::*;

#[derive(Queryable)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User")]
struct NewUser {
    first_name: String,
    last_name: String,
    password: String,
    email: String,
    phone_number: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field all_users(&executor) -> FieldResult<Vec<u32>> {
        use ::db::users::dsl::*;

        let conn = establish_connection();
        users.limit(5)
            .load::<User>(&conn);

        Ok(vec![])
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createUser(&executor, input: NewUser) -> FieldResult<Vec<u32>> {
        Ok(vec![0])

        //Ok(User{
            //id: 123,
            //first_name: input.first_name,
            //last_name: input.last_name,
            //phone_number: input.phone_number,
            //email: input.email,
        //})
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
