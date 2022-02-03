extern crate engine;

use engine::types::monster::Element::{Normal, Water, Rock, Ghost};
use engine::calculations::battle::elemental_advantage;

#[test]
fn normal_vs_works() {
    assert_eq!(elemental_advantage(Normal, Rock), 0.5);
    assert_eq!(elemental_advantage(Normal, Ghost), 0.0);
    assert_eq!(elemental_advantage(Normal, Water), 1.0);
}
