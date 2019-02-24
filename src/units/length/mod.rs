use crate::unit_creation::*;
/// The different pre-made unit systems for Length
pub mod systems;
#[cfg(test)]
mod test_measure;
#[cfg(test)]
mod test_unit;
/// The struct that specifies the unit for a Length entry.
///
/// The base unit for Length is `Metre`.
pub struct LengthUnit {
    /// The abbreviation for this unit
    abbr: &'static str,
    /// Full name of this unit (singular)
    singular: &'static str,
    /// Plural name of this unit
    plural: &'static str,
    /// How many of this unit does it take to make a metre
    in_metre: Float,
}
// Implement the UnitTrait
impl UnitTrait for LengthUnit {
    fn get_abbr(&self) -> &'static str {
        self.abbr
    }
    fn get_singular(&self) -> &'static str {
        self.singular
    }
    fn get_plural(&self) -> &'static str {
        self.plural
    }
    fn in_base(&self) -> Float {
        self.in_metre
    }
}
// Implement the equality operator
impl std::cmp::PartialEq for LengthUnit {
    fn eq(&self, other: &LengthUnit) -> bool {
        // TODO: What about spellings?
        self.in_metre == other.in_metre
    }
}
