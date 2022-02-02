use crate::generated::monsters::monster_by_id;
use crate::types::monsters::{Element, Species, Monster, StatGroup, InputMonster, InputStatGroup};
use crate::utils;
// use wasm_bindgen::prelude::*;

// Only needed due to 2018 edition because the macro is not accessible.
use juniper::{graphql_object, EmptyMutation, EmptySubscription, Variables, FieldError};

struct Context {
    // Use your real database pool here.
// pool: DatabasePool,
}

impl juniper::Context for Context {}
extern crate web_sys;

struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn speciesByID(_context: &Context, id: i32) -> Result<Species, FieldError> {
      match monster_by_id(id) {
        Ok(m) => Ok(m),
        Err(msg) => panic!("{}", msg)
      }
    }
}

struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    fn generate_monster(_context: &Context, species_id: i32, level: i32) -> Result<Monster, FieldError> {
     Ok(Monster{ species_id, species: todo!(), level, iv_stats: todo!(), ev_stats: todo!(), stats: todo!(), move_1_id: todo!(), move_2_id: todo!(), move_3_id: todo!(), move_4_id: todo!() })
    }

    // for chaining to customize freshly minted monster
    // fn set_stats(_context: &Context, monster: InputMonster, iv: InputStatGroup, ev: InputStatGroup) -> Result<Monster, FieldError> {
    //   todo!()
    // }

    // for chaining to customize freshly minted monster
    // fn set_moves(_context: &Context, monster: InputMonster) -> Result<Monster, FieldError> {
    //   todo!()
    // }
}


// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

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
        &Schema::new(Query, Mutation, EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    res.to_string()
}
