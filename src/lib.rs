mod base_types;
pub mod common;
pub mod length;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
/// Import this to bring into scope the traits common to each Measure
pub mod preamble {
    pub use super::base_types::{MeasureConvert, MeasureDisplay, MeasureTrait};
}
/// Import this to create a new Measure type
pub mod measure_creation {
    pub use super::base_types::{
        MeasureConvert, MeasureDisplay, MeasureInternalSetters, MeasureTrait, UnitListTrait,
    };
    pub use super::common::{Float, RoundTo};
}
/// Import this to create a new Unit type
pub mod unit_creation {
    pub use super::base_types::{UnitListTrait, UnitTrait};
    pub use super::common::Float;

}
