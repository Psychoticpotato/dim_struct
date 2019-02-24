mod base_types;
pub mod common;
pub mod units;
#[macro_use]
extern crate lazy_static;
pub use base_types::Measure;
/// Import this to create a new Unit type
pub mod unit_creation {
    pub use super::base_types::{UnitListTrait, UnitTrait};
    pub use super::common::Float;
}
