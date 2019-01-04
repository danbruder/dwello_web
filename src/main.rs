#[macro_use]
extern crate juniper;

use juniper::{EmptyMutation, FieldResult, Variables};

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

struct Query;

graphql_object!(Query: Ctx |&self| {
    field favoriteEpisode(&executor) -> FieldResult<Episode> {
        // Use the special &executor argument to fetch our fav episode.
        Ok(executor.context().0)
    }
});

// Arbitrary context data.
struct Ctx(Episode);

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>>;

fn main() {
    // Create a context object.
    let ctx = Ctx(Episode::NewHope);

    // Run the executor.
    let (res, _errors) = juniper::execute(
        "query { favoriteEpisode }",
        None,
        &Schema::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    let thing = res
        .as_object_value()
        .unwrap()
        .get_field_value("favoriteEpisode")
        .unwrap()
        .as_string_value()
        .unwrap();

    println!("{}", thing);
    // Ensure the value matches.
    assert_eq!(thing, "NEW_HOPE");
}
