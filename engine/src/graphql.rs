use wasm_bindgen::prelude::*;
// Only needed due to 2018 edition because the macro is not accessible.
use juniper::{
  graphql_object, EmptyMutation, FieldResult, 
  GraphQLEnum, Variables, graphql_value,
  EmptySubscription
};

use crate::utils;

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

// Arbitrary context data.
struct Ctx(Episode);

impl juniper::Context for Ctx {}
extern crate web_sys;

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    fn favoriteEpisode(context: &Ctx) -> FieldResult<Episode> {
        Ok(context.0)
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

pub fn handle_request(query: &str) -> String {
    utils::set_panic_hook();

    // web_sys::console::log_1(&"Hello, world!".into());

      // Create a context object.
    let ctx = Ctx(Episode::NewHope);

    // Run the executor.
    let (res, errors) = juniper::execute_sync(
        query,
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();

    res.to_string()
}
