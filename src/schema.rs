#![feature(decl_macro, proc_macro_hygiene)]

use juniper::{FieldResult, LookAheadSelection, RootNode};

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLEnum, Clone, Copy)]
enum Role {
    Admin,
    Anonymous,
    Editor,
    Writer,
}

#[derive(GraphQLObject, Clone)]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
    papers: Vec<Paper>,
    role: Role,
}

impl Human {
    fn from_lookahead(selection: LookAheadSelection<Human>) -> Self {
        Human {
            id: "1234".to_owned(),
            name: "Dan".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Earth".to_owned(),
            role: Role::Admin,
            papers: vec![Paper {
                title: "New York Times".to_string(),
                id: "1".to_string(),
                readers: vec![],
            }],
        }
    }
}

#[derive(GraphQLObject, Clone)]
#[graphql(description = "A medium of words")]
struct Paper {
    id: String,
    title: String,
    readers: Vec<Human>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    role: Role,
    home_planet: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String, name: Option<String>) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Dan".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Earth".to_owned(),
            role: Role::Admin,
            papers: vec![Paper{
                title: "New York Times".to_string(), 
                id: "1".to_string(),
                readers: vec![]
            }]
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            role: new_human.role,
            home_planet: new_human.home_planet,
            papers: vec![]
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
