use std::{fs, path::Path, fmt::format};
use serde_json::{Result, Value};

fn main() -> Result<()> {
  println!("Reading config...");
  let config_contents = fs::read_to_string("../config.json")
    .expect("Something went wrong reading the file");

    let config: Value = serde_json::from_str(&config_contents)?;

    println!("Collection Source: {}", config["collection_source"]);

    let base_path = config["collection_source"].as_str().unwrap();
    let species_path = format!("{}/species", base_path);
    print!("{}", base_path);

    let species_filenames = fs::read_dir(species_path).unwrap();

    for path in species_filenames {
        println!("Name: {}", path.unwrap().path().display())
    }

    Ok(())
}