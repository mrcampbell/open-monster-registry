use engine::types::monster::{Species, StatGroup, Monster, Element};
use serde_json::{Result, Value};
use std::{fmt::format, fs, path::Path};
use crate::species_raw_types::{SpeciesRawData};

mod species_raw_types;

fn main() -> Result<()> {
    println!("Reading config...");
    let config_contents = read_file("../config.json");

    let config: Value = str_to_json(&config_contents)?;

    println!("Collection Source: {}", config["collection_source"]);

    let base_path = config["collection_source"].as_str().unwrap();
    let species_path = format!("{}/species", base_path);
    // print!("{}", base_path);

    let species_filenames = fs::read_dir(species_path).unwrap();

    let mut species = vec![Species {
        id: -1,
        name: "unknown".into(),
        elements: vec![],
        stats:  StatGroup { hp: 0, atk: 0, def: 0, spec_atk: 0, spec_def: 0, speed: 0 }
    }];

    for path in species_filenames {
        // read file contents
        let file_path = path.unwrap().path();
        println!("Reading: {}", file_path.display());
        let species_contents = read_file(file_path.as_os_str().to_str().unwrap());
        // println!("{}", species_contents);
        let deserialized_monster: SpeciesRawData = serde_json::from_str(&species_contents).unwrap();
        // println!("Deserialized Monster: {:?}", deserialized_monster);

        // process species basic data
        let species_basic_data = process_species_basic_data(deserialized_monster);
        species.push(species_basic_data);

        // process move learns



    }

    let species_file_content = generate_species_file_contents(species);

    write_file("../engine/src/generated/species.rs", species_file_content);

    Ok(())
}

// todo: error handle
fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

// todo: error handle
fn write_file(filename: &str, content: String) {
    fs::write(filename, content).expect("Something went wrong reading the file")
}

fn str_to_json(str: &str) -> Result<Value> {
    serde_json::from_str(&str)
}

fn process_species_basic_data(raw_species: SpeciesRawData) -> Species {
    let mut species = Species {
        id: raw_species.id,
        name: raw_species.name,
        elements: raw_species.types.iter().map(|x| x.type_field.name.parse().unwrap()).collect(),
        stats: StatGroup { // todo: search by stat.name, but this is okay for now
            hp: raw_species.stats[0].base_stat,
            atk: raw_species.stats[1].base_stat,
            def: raw_species.stats[2].base_stat,
            spec_atk: raw_species.stats[3].base_stat,
            spec_def: raw_species.stats[4].base_stat,
            speed: raw_species.stats[5].base_stat,
        }
    };

    if species.elements.len() == 1 {
        species.elements.push(Element::None);
    }

    species
}

fn generate_species_file_contents(species: Vec<Species>) -> String {
    let mut match_statement: String = "".into();

    for m in species {
        let element_str = m
            .elements
            .iter()
            .map(|m| format!("Element::{:?}", m))
            .collect::<Vec<String>>()
            .join(", ");

        match_statement = format!(
            r##"{}
        {} => Ok(Species {{
            id: {},
            name: "{}".into(),
            elements: vec![{}],
            stats: StatGroup {{
                hp: {},
                atk: {},
                def: {},
                spec_atk: {},
                spec_def: {},
                speed: {},
            }},
        }}),"##,
            match_statement, m.id, m.id, m.name, element_str, m.stats.hp, m.stats.atk, m.stats.def, m.stats.spec_atk, m.stats.spec_def, m.stats.speed,
        );
    }

    format!(
        r##"use crate::types::monster::{{Element, Species, StatGroup}};

pub fn species_by_id(id: i32) -> Result<Species, &'static str> {{
    match id {{{}
        _ => Err("No species with id provided"),
    }}
}}
  
  "##,
        match_statement
    )
    .into()
}
