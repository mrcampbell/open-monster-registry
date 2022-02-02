use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

#[derive(GraphQLEnum, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Element {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct StatGroup {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spec_atk: i32,
    pub spec_def: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug)]
pub struct InputStatGroup {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spec_atk: i32,
    pub spec_def: i32,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct Species {
    pub id: i32,
    pub name: String,
    pub elements: Vec<Element>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct Monster {
    pub species_id: i32,
    pub species: Species,
    pub level: i32,
    pub iv_stats: StatGroup,
    pub ev_stats: StatGroup,
    pub stats: StatGroup,
    pub move_1_id: i32,
    pub move_2_id: i32,
    pub move_3_id: i32,
    pub move_4_id: i32,
}

pub struct InputMonster {
    pub species_id: i32,
    pub species: Species,
    pub level: i32,
    pub iv_stats: StatGroup,
    pub ev_stats: StatGroup,
    pub stats: StatGroup,
    pub move_1_id: i32,
    pub move_2_id: i32,
    pub move_3_id: i32,
    pub move_4_id: i32,
}

// impl Monster {
//     fn from_input_value(v: &InputMonster) -> Option<Self> {
//       // TODO
//       Some(Monster{ species_id: todo!(), species: todo!(), level: todo!(), iv_stats: todo!(), ev_stats: todo!(), stats: todo!(), move_1_id: todo!(), move_2_id: todo!(), move_3_id: todo!(), move_4_id: todo!() })
//     }
// }