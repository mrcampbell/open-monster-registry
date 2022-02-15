use std::fs;

use crate::species_raw_types::SpeciesRawData;
use engine::types::monster::{Element, MoveLearnMethod, Species, SpeciesMoveLearns, StatGroup};
use regex::Regex;
use serde_json::{Result, Value};

mod species_raw_types;

const ENV_VERSION_GROUP: &str = "red-blue";
const ENV_LANGUAGE_ID: i32 = 1;

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
        stats: StatGroup {
            hp: 0,
            atk: 0,
            def: 0,
            spec_atk: 0,
            spec_def: 0,
            speed: 0,
        },
    }];

    for path in species_filenames {
        // read file contents
        let file_path = path.unwrap().path();
        println!("Reading: {}", file_path.display());
        let species_contents = read_file(file_path.as_os_str().to_str().unwrap());
        // println!("{}", species_contents);
        let deserialized_species: SpeciesRawData = serde_json::from_str(&species_contents).unwrap();
        // println!("Deserialized Monster: {:?}", deserialized_monster);

        // process species basic data
        let species_basic_data = process_species_basic_data(&deserialized_species);
        let move_learns = process_species_move_learns(deserialized_species);

        print!("{:?}", move_learns);

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

fn process_species_basic_data(raw_species: &SpeciesRawData) -> Species {
    let mut species = Species {
        id: raw_species.id,
        name: raw_species.name.to_string(),
        elements: raw_species
            .types
            .iter()
            .map(|x| x.type_field.name.parse().unwrap())
            .collect(),
        stats: StatGroup {
            // todo: search by stat.name, but this is okay for now
            hp: raw_species.stats[0].base_stat,
            atk: raw_species.stats[1].base_stat,
            def: raw_species.stats[2].base_stat,
            spec_atk: raw_species.stats[3].base_stat,
            spec_def: raw_species.stats[4].base_stat,
            speed: raw_species.stats[5].base_stat,
        },
    };

    if species.elements.len() == 1 {
        species.elements.push(Element::None);
    }

    species
}

fn process_species_move_learns(raw_species: SpeciesRawData) -> Vec<SpeciesMoveLearns> {
    // todo: how to move this outside of function?
    let move_url_regex: regex::Regex = Regex::new(r"move/(.*)/").unwrap();

    // todo: yo wtf is this?
    let caps = move_url_regex.captures(&raw_species.moves[0].move_field.url);
    let move_id = caps
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();

    raw_species
        .moves
        .iter()
        .map(|x| {
            match x
                .clone() // todo: hack to avoid shared reference
                .version_group_details
                .into_iter()
                .find(|x| x.version_group.name == ENV_VERSION_GROUP)
            {
                Some(vgDetail) => SpeciesMoveLearns {
                    species_id: raw_species.id,
                    move_id: move_id,
                    // todo: filter based off of version group
                    method: match vgDetail.move_learn_method.name.as_str() {
                        "level-up" => MoveLearnMethod::LevelUp,
                        "tutor" => MoveLearnMethod::Tutor,
                        "egg" => MoveLearnMethod::Egg,
                        "machine" => MoveLearnMethod::TM,
                        "special" => MoveLearnMethod::Special,
                        _ => panic!("Unknown move learn method"),
                    },
                    level: (vgDetail.level_learned_at).into(),
                },
                None => SpeciesMoveLearns {
                    species_id: 0,
                    move_id: -1,
                    method: MoveLearnMethod::LevelUp,
                    level: Some(-1),
                },
            }
        })
        .filter(|x| x.move_id > 0)
        .collect()
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
            match_statement,
            m.id,
            m.id,
            m.name,
            element_str,
            m.stats.hp,
            m.stats.atk,
            m.stats.def,
            m.stats.spec_atk,
            m.stats.spec_def,
            m.stats.speed,
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
