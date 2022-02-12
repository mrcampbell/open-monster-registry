use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesRawData {
    pub abilities: Vec<Ability>,
    #[serde(rename = "base_experience")]
    pub base_experience: i32,
    pub forms: Vec<Form>,
    #[serde(rename = "game_indices")]
    pub game_indices: Vec<Index>,
    pub height: i32,
    #[serde(rename = "held_items")]
    pub held_items: Vec<Value>,
    pub id: i32,
    #[serde(rename = "is_default")]
    pub is_default: bool,
    #[serde(rename = "location_area_encounters")]
    pub location_area_encounters: String,
    pub moves: Vec<Mfe>,
    pub name: String,
    pub order: i32,
    #[serde(rename = "past_types")]
    pub past_types: Vec<Value>,
    pub species: Species,
    pub sprites: Sprites,
    pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub ability: Ability2,
    #[serde(rename = "is_hidden")]
    pub is_hidden: bool,
    pub slot: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    #[serde(rename = "game_index")]
    pub game_index: i32,
    pub version: Version,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mfe {
    #[serde(rename = "move")]
    pub move_field: Move,
    #[serde(rename = "version_group_details")]
    pub version_group_details: Vec<VersionGroupDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGroupDetail {
    #[serde(rename = "level_learned_at")]
    pub level_learned_at: i32,
    #[serde(rename = "move_learn_method")]
    pub move_learn_method: MoveLearnMethod,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroup,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveLearnMethod {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGroup {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprites {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_female")]
    pub back_female: Value,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Value,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
    pub other: Other,
    pub versions: Versions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Other {
    #[serde(rename = "dream_world")]
    pub dream_world: DreamWorld,
    pub home: Home,
    #[serde(rename = "official-artwork")]
    pub official_artwork: OfficialArtwork,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DreamWorld {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Home {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficialArtwork {
    #[serde(rename = "front_default")]
    pub front_default: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    #[serde(rename = "generation-i")]
    pub generation_i: GenerationI,
    #[serde(rename = "generation-ii")]
    pub generation_ii: GenerationIi,
    #[serde(rename = "generation-iii")]
    pub generation_iii: GenerationIii,
    #[serde(rename = "generation-iv")]
    pub generation_iv: GenerationIv,
    #[serde(rename = "generation-v")]
    pub generation_v: GenerationV,
    #[serde(rename = "generation-vi")]
    pub generation_vi: GenerationVi,
    #[serde(rename = "generation-vii")]
    pub generation_vii: GenerationVii,
    #[serde(rename = "generation-viii")]
    pub generation_viii: GenerationViii,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationI {
    #[serde(rename = "red-blue")]
    pub red_blue: RedBlue,
    pub yellow: Yellow,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedBlue {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_gray")]
    pub back_gray: String,
    #[serde(rename = "back_transparent")]
    pub back_transparent: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_gray")]
    pub front_gray: String,
    #[serde(rename = "front_transparent")]
    pub front_transparent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yellow {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_gray")]
    pub back_gray: String,
    #[serde(rename = "back_transparent")]
    pub back_transparent: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_gray")]
    pub front_gray: String,
    #[serde(rename = "front_transparent")]
    pub front_transparent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationIi {
    pub crystal: Crystal,
    pub gold: Gold,
    pub silver: Silver,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crystal {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_transparent")]
    pub back_shiny_transparent: String,
    #[serde(rename = "back_transparent")]
    pub back_transparent: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_transparent")]
    pub front_shiny_transparent: String,
    #[serde(rename = "front_transparent")]
    pub front_transparent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gold {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_transparent")]
    pub front_transparent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Silver {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_transparent")]
    pub front_transparent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationIii {
    pub emerald: Emerald,
    #[serde(rename = "firered-leafgreen")]
    pub firered_leafgreen: FireredLeafgreen,
    #[serde(rename = "ruby-sapphire")]
    pub ruby_sapphire: RubySapphire,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emerald {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FireredLeafgreen {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RubySapphire {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationIv {
    #[serde(rename = "diamond-pearl")]
    pub diamond_pearl: DiamondPearl,
    #[serde(rename = "heartgold-soulsilver")]
    pub heartgold_soulsilver: HeartgoldSoulsilver,
    pub platinum: Platinum,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiamondPearl {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_female")]
    pub back_female: Value,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Value,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartgoldSoulsilver {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_female")]
    pub back_female: Value,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Value,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platinum {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_female")]
    pub back_female: Value,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Value,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationV {
    #[serde(rename = "black-white")]
    pub black_white: BlackWhite,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlackWhite {
    pub animated: Animated,
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_female")]
    pub back_female: Value,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Value,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animated {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_female")]
    pub back_female: Value,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Value,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationVi {
    #[serde(rename = "omegaruby-alphasapphire")]
    pub omegaruby_alphasapphire: OmegarubyAlphasapphire,
    #[serde(rename = "x-y")]
    pub x_y: XY,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OmegarubyAlphasapphire {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XY {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationVii {
    pub icons: Icons,
    #[serde(rename = "ultra-sun-ultra-moon")]
    pub ultra_sun_ultra_moon: UltraSunUltraMoon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icons {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltraSunUltraMoon {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationViii {
    pub icons: Icons2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icons2 {
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    #[serde(rename = "base_stat")]
    pub base_stat: i32,
    pub effort: i32,
    pub stat: Stat2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat2 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub slot: i32,
    #[serde(rename = "type")]
    pub type_field: Type2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type2 {
    pub name: String,
    pub url: String,
}
