use crate::types::monsters::{Element, InputMonster, InputStatGroup, Monster, Species, StatGroup};
use crate::types::monsters::Element::{Dark, Dragon, Fairy, Ghost, Steel, Rock};

pub fn elemental_advantage(atk_type: Element, def_type: Element) -> f32 {
    match atk_type {
        Normal => match def_type {
          Rock => 0.5,
          Steel => 0.5,
          Ghost => 0.0,
          _ => 1.0,
        },
        Fire => match def_type {_ => 1.0,},
        Water => match def_type {_ => 1.0,},
        Grass => match def_type {_ => 1.0,},
        Electric => match def_type {_ => 1.0,},
        Ice => match def_type {_ => 1.0,},
        Fighting => match def_type {_ => 1.0,},
        Poison => match def_type {_ => 1.0,},
        Ground => match def_type {_ => 1.0,},
        Flying => match def_type {_ => 1.0,},
        Psychic => match def_type {_ => 1.0,},
        Bug => match def_type {_ => 1.0,},
        Rock => match def_type {_ => 1.0,},
        Ghost => match def_type {_ => 1.0,},
        Dragon => match def_type {_ => 1.0,},
        Dark => match def_type {_ => 1.0,},
        Steel => match def_type {_ => 1.0,},
        Fairy => match def_type {_ => 1.0,},
        _ => 1.0,
    }
}