use std::fs;
use serde_json::{Result, Value};

fn main() -> Result<()> {
  println!("Reading config...");
  let config_contents = fs::read_to_string("../config.json")
    .expect("Something went wrong reading the file");

    let config: Value = serde_json::from_str(&config_contents)?;

    println!("Collection Source: {}", config["collection_source"]);

    Ok(())
}