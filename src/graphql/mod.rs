//
// graphql.rs
//

use accounts::types::{User,LoginInput, AuthPayload, RegistrationInput};
use deals::types::{Deal};
use db::{ConnectionPool};
use juniper::{FieldResult};
use super::{accounts,deals};

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub struct Ctx {
    pub user: Option<User>,
    pub pool: ConnectionPool
}

pub struct Query;
pub struct Mutation;

graphql_object!(Query: Ctx |&self| {
    field all_users(&executor) -> FieldResult<Vec<User>> {
        accounts::resolvers::all_users(executor)
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field login(&executor, input: LoginInput) -> FieldResult<AuthPayload> {
        accounts::resolvers::login(executor, input)
    }

    field register(&executor, input: RegistrationInput) -> FieldResult<AuthPayload> {
        accounts::resolvers::register(executor, input)
    }

    field create_deal(&executor) -> FieldResult<Deal> {
        deals::resolvers::create_deal(executor)
    }
});
