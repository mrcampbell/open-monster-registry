use crate::utils;
// use wasm_bindgen::prelude::*;

// Only needed due to 2018 edition because the macro is not accessible.
use juniper::{
  graphql_object, EmptyMutation, FieldResult, 
  GraphQLEnum, Variables,
  EmptySubscription
};

#[derive(GraphQLEnum, Clone, Copy)]
enum Element {
    Fire,
    Water,
    Grass,
    Poison,
}

#[derive(GraphQLObject)]
struct Species {
  id: i32,
  name: String,
  elements: Vec<Element>
}

struct Context {
  // Use your real database pool here.
  // pool: DatabasePool,
}

impl juniper::Context for Context {}
extern crate web_sys;

struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn speciesByID(_context: &Context, id: i32) -> FieldResult<Species> {
        match id {
          1 => Ok(Species { id: 1, name: "Bulbasaur".into(), elements: vec![Element::Grass, Element::Poison]}),
          _ => panic!("no species with id provided")
        }
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn handle_request(query: &str) -> String {
    utils::set_panic_hook();

    web_sys::console::log_1(&"Handling Request: ".into());
    web_sys::console::log_1(&query.into());

      // Create a context object.
    let ctx = Context {};

    // Run the executor.
    let (res, _errors) = juniper::execute_sync(
        query,
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();

    res.to_string()
}
