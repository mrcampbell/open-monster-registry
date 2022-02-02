
  use crate::types::monsters::{Element, Species};

  pub fn monster_by_id(id: i32) -> Result<Species, &'static str> {
      match id {
				-1 => Species { id: -1, name: "unknown", elements: [] },
				1 => Species { id: 1, name: "Bulbasaur", elements: [Poison, Grass] },
          _ => Err("No species with id provided"),
      }
  }
  
  