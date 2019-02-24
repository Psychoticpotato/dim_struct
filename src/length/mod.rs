use crate::unit_creation::*;
/// Module with SI units (Metre and such)
pub mod si;
/// Module with American spellings of SI units (Meter and such)
pub mod si_us;
#[cfg(test)]
mod test_measure;
#[cfg(test)]
mod test_unit;
/// Module with the United States customary units (Inch and such)
pub mod us;

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

pub struct LengthUnitList {
    /// Title of these units
    title: &'static str,
    /// The list of stored units
    units: Vec<&'static LengthUnit>,
}

impl UnitListTrait<LengthUnit> for LengthUnitList {
    fn get_title(&self) -> &str {
        self.title
    }
    fn get_list(&self) -> &Vec<&'static LengthUnit> {
        &self.units
    }
    fn get_list_mut(&mut self) -> &mut Vec<&'static LengthUnit> {
        &mut self.units
    }
}
