mod controls;
#[macro_use]
mod macros;
mod segment;
mod segment_list;
mod utils;

pub use utils::*;

const ID_SEPARATOR: &str = "-";
const COMPOUND_ID_SEPARATOR: &str = "+";
const CATEGORY_TYPE_SEPARATOR: &str = ":";
const COMPOUND_TYPE_SEPARATOR: &str = "+";
