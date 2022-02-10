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
    pub speed: i32,
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
    pub stats: StatGroup,
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
    pub level: i32,
    pub iv_stats: Option<StatGroup>,
    pub ev_stats: Option<StatGroup>,
    pub move_1_id: i32,
    pub move_2_id: Option<i32>,
    pub move_3_id: Option<i32>,
    pub move_4_id: Option<i32>,
}

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug)]
pub struct CalculateOtherStatInput {
  pub level: i32, 
  pub base: i32, 
  pub iv: i32, 
  pub ev: i32
}

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug)]
pub struct CalculateHPStatInput {
  pub level: i32, 
  pub base: i32, 
  pub iv: i32, 
  pub ev: i32
}
