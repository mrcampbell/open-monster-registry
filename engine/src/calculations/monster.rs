use crate::types::monster::{StatGroup, Monster};
use crate::generated::monster::species_by_id;

pub fn inflate_monster(species_id: i32, level: i32, move_1_id: i32, move_2_id: Option<i32>, move_3_id: Option<i32>, move_4_id: Option<i32>, input_iv: Option<StatGroup>, input_ev: Option<StatGroup>) -> Result<Monster, String> {
  let species = match species_by_id(species_id) {
    Ok(s) => s,
    Err(msg) => return Err(msg.to_string()),
  };

  let iv: StatGroup = input_iv.unwrap_or(StatGroup { hp: 0, atk: 0, def: 0, spec_atk: 0, spec_def: 0, speed: 0 });
  let ev: StatGroup = input_ev.unwrap_or(StatGroup { hp: 0, atk: 0, def: 0, spec_atk: 0, spec_def: 0, speed: 0 });
  // let stats: StatGroup = StatGroup {
  //   hp: calculate_hp(level, species.hp, iv.hp, ev.hp),
  // }

  return Ok(Monster { species_id, species, level, iv_stats: iv, ev_stats: ev, stats: todo!(), move_1_id, move_2_id: todo!(), move_3_id: todo!(), move_4_id: todo!() })
}

fn calculate_stat(level: i32, base: i32, iv: i32, ev: i32) -> i32 {
  return 0
}

fn calculate_hp(level: i32, base: i32, iv: i32, ev: i32) -> i32 {
  return 0
}

#[test]
// fn test_inflate_with_valid_empty_data() {
//   let species_id = 1;
//   let level = 5;
//   let move_1_id = 1;
//   let move_2_id = None;
//   let move_3_id = None;
//   let move_4_id = None;
//   let iv = None;
//   let ev = None;

//   let monster = inflate_monster(species_id, level, move_1_id, move_2_id, move_3_id, move_4_id, iv, ev);

//   assert!(monster.is_ok());
// }

#[test]
fn test_inflate_with_invalid_species_id() {
  let species_id = 0;
  let level = 5;
  let move_1_id = 1;
  let move_2_id = None;
  let move_3_id = None;
  let move_4_id = None;
  let iv = None;
  let ev = None;

  let monster = inflate_monster(species_id, level, move_1_id, move_2_id, move_3_id, move_4_id, iv, ev);

  assert!(monster.is_err());
}