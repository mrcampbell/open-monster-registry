use engine::types::monster::{Species, StatGroup, Monster};
use serde_json::{Result, Value};
use std::{fmt::format, fs, path::Path};

fn main() -> Result<()> {
    println!("Reading config...");
    let config_contents = read_file("../config.json");

    let config: Value = str_to_json(&config_contents)?;

    println!("Collection Source: {}", config["collection_source"]);

    let base_path = config["collection_source"].as_str().unwrap();
    let species_path = format!("{}/species", base_path);
    print!("{}", base_path);

    let species_filenames = fs::read_dir(species_path).unwrap();

    let mut monsters = vec![Species {
        id: -1,
        name: "unknown".into(),
        elements: vec![],
        stats:  StatGroup { hp: 0, atk: 0, def: 0, spec_atk: 0, spec_def: 0, speed: 0 }
    }];

    for path in species_filenames {
        let file_path = path.unwrap().path();
        println!("Reading: {}", file_path.display());
        let species_contents = read_file(file_path.as_os_str().to_str().unwrap());
        println!("{}", species_contents);
        let deserialized_monster: Species = serde_json::from_str(&species_contents).unwrap();
        println!("Deserialized Monster: {:?}", deserialized_monster);
        monsters.push(deserialized_monster);
    }

    let monster_file_content = generate_monster_file_contents(monsters);

    write_file("../engine/src/generated/monster.rs", monster_file_content);

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

fn generate_monster_file_contents(monsters: Vec<Species>) -> String {
    let mut match_statement: String = "".into();

    for m in monsters {
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
        }}),
        "##,
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
