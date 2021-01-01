pub mod controls;
#[macro_use]
pub mod macros;
pub mod segment;
pub mod segment_list;
pub mod utils;

pub use macros::*;
pub use utils::*;

const ID_SEPARATOR: &str = "-";
const COMPOUND_ID_SEPARATOR: &str = "+";
const CATEGORY_TYPE_SEPARATOR: &str = ":";
const COMPOUND_TYPE_SEPARATOR: &str = "+";
