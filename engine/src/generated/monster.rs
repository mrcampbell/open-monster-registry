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
            name: "Bulbasaur".into(),
            elements: vec![Element::Poison, Element::Grass],
            stats: StatGroup {
                hp: 45,
                atk: 49,
                def: 49,
                spec_atk: 65,
                spec_def: 65,
                speed: 45,
            },
        }),
        
        _ => Err("No species with id provided"),
    }
}
  
  