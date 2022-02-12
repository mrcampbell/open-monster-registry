use crate::types::monster::{Element, Species, StatGroup};

pub fn species_by_id(id: i32) -> Result<Species, &'static str> {
    match id {
        -1 => Ok(Species {
            id: -1,
            name: "unknown".into(),
            elements: vec![],
            stats: StatGroup {
                hp: 0,
                atk: 0,
                def: 0,
                spec_atk: 0,
                spec_def: 0,
                speed: 0,
            },
        }),
        1 => Ok(Species {
            id: 1,
            name: "bulbasaur".into(),
            elements: vec![Element::Grass, Element::Poison],
            stats: StatGroup {
                hp: 45,
                atk: 49,
                def: 49,
                spec_atk: 65,
                spec_def: 65,
                speed: 45,
            },
        }),
        2 => Ok(Species {
            id: 2,
            name: "ivysaur".into(),
            elements: vec![Element::Grass, Element::Poison],
            stats: StatGroup {
                hp: 60,
                atk: 62,
                def: 63,
                spec_atk: 80,
                spec_def: 80,
                speed: 60,
            },
        }),
        4 => Ok(Species {
            id: 4,
            name: "charmander".into(),
            elements: vec![Element::Fire, Element::None],
            stats: StatGroup {
                hp: 39,
                atk: 52,
                def: 43,
                spec_atk: 60,
                spec_def: 50,
                speed: 65,
            },
        }),
        _ => Err("No species with id provided"),
    }
}
  
  