use crate::types::monsters::{Element, Species};

pub fn monster_by_id(id: i32) -> Result<Species, &'static str> {
    match id {
        -1 => Ok(Species {
            id: -1,
            name: "unknown".into(),
            elements: vec![],
        }),
        
        1 => Ok(Species {
            id: 1,
            name: "Bulbasaur".into(),
            elements: vec![Element::Poison, Element::Grass],
        }),
        
        _ => Err("No species with id provided"),
    }
}
  
  