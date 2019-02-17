use crate::unit_creation::*;
#[cfg(test)]
mod test;

/// Module with SI units (Metre and such)
pub mod si;
/// Module with American spellings of SI units (Meter and such)
pub mod si_us;
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
    fn add_unit(&mut self, new_unit: &'static LengthUnit) {
        self.units.push(new_unit)
    }
}
