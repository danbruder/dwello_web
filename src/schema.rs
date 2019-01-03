#![allow(proc_macro_derive_resolution_fallback)]

use juniper::{FieldResult, RootNode};
use super::Ctx;
use super::models::*;

use diesel::prelude::*;

#[derive(GraphQLObject)]
#[graphql(description = "User")]
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

graphql_object!(QueryRoot: Ctx |&self| {
    field all_users(&executor) -> FieldResult<Vec<User>> {
        let one = executor.context().why;
        use ::db::users::dsl::*;

        users.limit(5)
            .load::<User>(&executor.context().conn)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Ctx |&self| {
    field createUser(&executor, input: NewUser) -> FieldResult<User> {
        Ok(User{
            id: 123,
            first_name: input.first_name,
            last_name: input.last_name,
            phone_number: input.phone_number,
            email: input.email,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
