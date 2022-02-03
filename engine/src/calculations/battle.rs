use crate::types::monster::Element;
use crate::types::monster::Element::{
    Bug, Dark, Dragon, Electric, Fairy, Fighting, Fire, Flying, Ghost, Grass, Ground, Ice, Normal,
    Poison, Psychic, Rock, Steel, Water,
};

pub fn elemental_advantage(atk_type: Element, def_type: Element) -> f32 {
    match atk_type {
        Element::Normal => match def_type {
            Rock | Steel => 0.5,
            Ghost => 0.0,
            _ => 1.0,
        },
        Element::Fire => match def_type {
            Fire | Water | Rock | Dragon => 0.5,
            Grass | Ice | Bug | Steel => 2.0,
            _ => 1.0,
        },
        Element::Water => match def_type {
            Water | Grass | Dragon => 0.5,
            Fire | Ground | Rock => 2.0,
            _ => 1.0,
        },
        Element::Electric => match def_type {
            Electric | Grass | Dragon => 0.5,
            Water | Flying => 2.0,
            Ground => 0.0,
            _ => 1.0,
        },
        Element::Grass => match def_type {
            Fire | Grass | Poison | Flying | Bug | Dragon | Steel => 0.5,
            Water | Ground | Rock => 2.0,
            _ => 1.0,
        },
        Element::Ice => match def_type {
            Fire | Water | Ice | Steel => 0.5,
            Grass | Ground | Flying | Dragon => 2.0,
            _ => 1.0,
        },
        Element::Fighting => match def_type {
            Poison | Flying | Psychic | Bug | Fairy => 0.5,
            Normal | Ice | Rock | Dark | Steel => 2.0,
            _ => 1.0,
        },
        Element::Poison => match def_type {
            Poison | Ground | Rock | Ghost => 0.5,
            Grass | Fairy => 2.0,
            Steel => 0.0,
            _ => 1.0,
        },
        Element::Ground => match def_type {
            Grass | Bug => 0.5,
            Fire | Electric | Poison | Rock | Steel => 2.0,
            Flying => 0.0,
            _ => 1.0,
        },
        Element::Flying => match def_type {
            Electric | Rock | Steel => 0.5,
            Grass | Fighting | Bug => 2.0,
            _ => 1.0,
        },
        Element::Psychic => match def_type {
            Psychic | Steel => 0.5,
            Fighting | Poison => 0.5,
            Dark => 0.0,
            _ => 1.0,
        },
        Element::Bug => match def_type {
            Fire | Fighting | Poison | Flying | Ghost | Steel | Fairy => 0.5,
            Grass | Psychic | Dark => 2.0,
            _ => 1.0,
        },
        Element::Rock => match def_type {
            Fighting | Ground | Steel => 0.5,
            Fire | Ice | Flying | Bug => 2.0,
            _ => 1.0,
        },
        Element::Ghost => match def_type {
            Dark => 0.5,
            Psychic | Ghost => 2.0,
            Normal => 0.0,
            _ => 1.0,
        },
        Element::Dragon => match def_type {
            Steel => 0.5,
            Dragon => 2.0,
            Fairy => 0.0,
            _ => 1.0,
        },
        Element::Dark => match def_type {
            Fighting | Dark | Fairy => 0.5,
            Psychic | Ghost => 2.0,
            _ => 1.0,
        },
        Element::Steel => match def_type {
            Fire | Water | Electric | Steel  => 0.5,
            Ice | Rock | Fairy => 2.0,
            _ => 1.0,
        },
        Element::Fairy => match def_type {
            Fire | Poison | Steel => 0.5,
            Fighting | Dragon | Dark => 2.0,
            _ => 1.0,
        },
    }
}
