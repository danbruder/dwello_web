//
// graphql.rs
//

use models::user::{User};
use models::graphql::{LoginInput, AuthPayload, RegistrationInput};
use db::{ConnectionPool};
use juniper::{ FieldResult };
use resolvers::{user,auth};

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub struct Ctx {
    pub user: Option<User>,
    pub pool: ConnectionPool
}

pub struct Query;
pub struct Mutation;

graphql_object!(Query: Ctx |&self| {
    field all_users(&executor) -> FieldResult<Vec<User>> {
        user::all_users(executor)
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field login(&executor, input: LoginInput) -> FieldResult<AuthPayload> {
        auth::login(executor, input)
    }

    field register(&executor, input: RegistrationInput) -> FieldResult<AuthPayload> {
        auth::register(executor, input)
    }
});
