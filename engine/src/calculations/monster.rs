use crate::generated::monster::species_by_id;
use crate::types::monster::{Monster, StatGroup};

pub fn create_randomized_monster(species_id: i32, level: i32) -> Result<Monster, String> {
  // todo: find learnable moves
  inflate_monster(species_id, level, 1, None, None, None, None, None)
}

pub fn inflate_monster(
    species_id: i32,
    level: i32,
    move_1_id: i32,
    move_2_id: Option<i32>,
    move_3_id: Option<i32>,
    move_4_id: Option<i32>,
    input_iv: Option<StatGroup>,
    input_ev: Option<StatGroup>,
) -> Result<Monster, String> {
    let species = match species_by_id(species_id) {
        Ok(s) => s,
        Err(msg) => return Err(msg.to_string()),
    };

    let iv: StatGroup = input_iv.unwrap_or(StatGroup {
        hp: 0,
        atk: 0,
        def: 0,
        spec_atk: 0,
        spec_def: 0,
        speed: 0,
    });
    let ev: StatGroup = input_ev.unwrap_or(StatGroup {
        hp: 0,
        atk: 0,
        def: 0,
        spec_atk: 0,
        spec_def: 0,
        speed: 0,
    });

    let stats: StatGroup = StatGroup {
        hp: calculate_hp(level, species.stats.hp, iv.hp, ev.hp),
        atk: calculate_stat(level, species.stats.atk, iv.atk, ev.atk),
        def: calculate_stat(level, species.stats.def, iv.def, ev.def),
        spec_atk: calculate_stat(level, species.stats.spec_atk, iv.spec_atk, ev.spec_atk),
        spec_def: calculate_stat(level, species.stats.spec_def, iv.spec_def, ev.spec_def),
        speed: calculate_stat(level, species.stats.speed, iv.speed, ev.speed),
    };

    return Ok(Monster {
        species_id,
        species,
        level,
        iv_stats: iv,
        ev_stats: ev,
        stats,
        move_1_id,
        move_2_id: move_2_id.unwrap_or(-1),
        move_3_id: move_3_id.unwrap_or(-1),
        move_4_id: move_4_id.unwrap_or(-1),
    });
}

// calculated using gen III and onwards
pub fn calculate_stat(level: i32, base: i32, iv: i32, ev: i32) -> i32 {
    let mut stat: i32 = (((2 * base + iv + ev / 4) * level) / 100) + 5;
    if stat > 255 {
        stat = 255;
    }
    return stat;
}

pub fn calculate_hp(level: i32, base: i32, iv: i32, ev: i32) -> i32 {
    calculate_stat(level, base, iv, ev) + 5 + level
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

    let monster = inflate_monster(
        species_id, level, move_1_id, move_2_id, move_3_id, move_4_id, iv, ev,
    );

    assert!(monster.is_err());
}

#[test]
fn test_def_stat_calculation() {
    let level = 78;
    let base = 95;
    let iv = 30;
    let ev = 91;

    let stat = calculate_stat(level, base, iv, ev);

    assert_eq!(stat, 193);
}

#[test]
fn test_hp_stat_calculation() {
    let level = 78;
    let base = 108;
    let iv = 24;
    let ev = 74;

    let stat = calculate_hp(level, base, iv, ev);

    assert_eq!(stat, 289);
}
