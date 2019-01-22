//
// graphql.rs
//

use accounts::types::{User,LoginInput, AuthPayload, RegistrationInput};
use deals::types::{HouseInput};
use deals::types::{Deal};
use db::{ConnectionPool};
use juniper::{FieldResult,FieldError};
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
        let conn = executor.context().pool.get().unwrap();
        let current_user = executor.context().user.clone();
        accounts::all_users(conn, current_user)
            .map_err(|e| FieldError::from(e))
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field login(&executor, input: LoginInput) -> FieldResult<AuthPayload> {
        let conn = executor.context().pool.get().unwrap();
        accounts::login(conn, input)
            .map_err(|e| FieldError::from(e))
    }

    field register(&executor, input: RegistrationInput) -> FieldResult<AuthPayload> {
        let conn = executor.context().pool.get().unwrap();
        accounts::register(conn, input)
            .map_err(|e| FieldError::from(e))
    }

    field create_deal(&executor, input: HouseInput) -> FieldResult<Deal> {
        let current_user = executor.context().user.clone();
        let conn = executor.context().pool.get().unwrap();
        deals::create_deal(conn, current_user, input)
            .map_err(|e| FieldError::from(e))
    }
});
