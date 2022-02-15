use std::str::FromStr;

use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

use crate::generated::move_learns::move_learns_by_species_id;

#[derive(GraphQLEnum, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Element {
    None,
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

impl FromStr for Element {
    type Err = ();

    fn from_str(input: &str) -> Result<Element, Self::Err> {
        match input.to_lowercase().as_str() {
            "none" => Ok(Element::None),
            "normal" => Ok(Element::Normal),
            "fire" => Ok(Element::Fire),
            "water" => Ok(Element::Water),
            "electric" => Ok(Element::Electric),
            "grass" => Ok(Element::Grass),
            "ice" => Ok(Element::Ice),
            "fighting" => Ok(Element::Fighting),
            "poison" => Ok(Element::Poison),
            "ground" => Ok(Element::Ground),
            "flying" => Ok(Element::Flying),
            "psychic" => Ok(Element::Psychic),
            "bug" => Ok(Element::Bug),
            "rock" => Ok(Element::Rock),
            "ghost" => Ok(Element::Ghost),
            "dragon" => Ok(Element::Dragon),
            "dark" => Ok(Element::Dark),
            "steel" => Ok(Element::Steel),
            "fairy" => Ok(Element::Fairy),
            _ => Err(()),
        }
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Species {
    pub id: i32,
    pub name: String,
    pub elements: Vec<Element>,
    pub stats: StatGroup,
}

#[graphql_object]
impl Species {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    fn stats(&self) -> &StatGroup {
        &self.stats
    }

    fn moves(&self) -> Vec<SpeciesMoveLearn> {
        move_learns_by_species_id(self.id)
    }
}

#[derive(GraphQLEnum, Serialize, Deserialize, Debug)]
pub enum MoveLearnMethod {
    LevelUp,
    TM,
    HM,
    Tutor,
    Egg,
    Special,
    Other,
}
#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct SpeciesMoveLearn {
    // #[graphql(ignore)]
    pub species_id: i32,
    pub move_id: i32,
    pub method: MoveLearnMethod,
    pub level: Option<i32>,
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
    pub ev: i32,
}

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug)]
pub struct CalculateHPStatInput {
    pub level: i32,
    pub base: i32,
    pub iv: i32,
    pub ev: i32,
}
