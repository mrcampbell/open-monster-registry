extern crate engine;

#[test]
pub fn test_simple_query() {
  let res = engine::execute("query { favoriteEpisode }".to_string());

  println!("RESULT: {}", res);
}