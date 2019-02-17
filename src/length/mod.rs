use crate::length::units::{LengthUnit, LengthUnitList};
use crate::measure_creation::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(test)]
mod test;
pub mod units;
#[derive(Copy, Clone)]
/// Length handles, well, length.
///
/// Feet, meters, lightyears, etc.
pub struct Length {
    /// The unit of this value
    unit: &'static LengthUnit,
    /// The value stored as a float
    val: Float,
}
impl Length {
    pub fn new(val: Float, unit: &'static LengthUnit) -> Length {
        Length { val, unit }
    }
    /// Parses the literal string for a length (ex: `12m`).
    ///
    /// Returns a Length struct if the value was parsed correctly
    pub fn from_literal(system: LengthUnitList, val: &str) -> Option<Length> {
        // Parse the string
        let res = system.parse_str(val)?;
        // If found, return a new Length
        Some(Length {
            val: res.0,
            unit: res.1,
        })
    }
}
// Implement the measurement traits
impl MeasureTrait for Length {
    type Unit = LengthUnit;
    fn get_val(&self) -> Float {
        self.val
    }
    fn get_unit(&self) -> &'static LengthUnit {
        self.unit
    }
    fn add_other(&mut self, other: &Length) {
        self.val += other.get_val_as(self.unit);
    }
    fn subtract_other(&mut self, other: &Length) {
        self.val -= other.get_val_as(self.unit);
    }
}
// Implement the internal setters
impl MeasureInternalSetters<Length> for Length {
    fn set_unit_internal(&mut self, unit: &'static LengthUnit) {
        self.unit = unit;
    }
    fn set_val_internal(&mut self, val: Float) {
        self.val = val;
    }
}
// Implement the conversion traits
impl MeasureConvert<Length> for Length {}

// Add the operators
impl Add<Length> for Length {
    type Output = Length;
    fn add(self, other: Length) -> Length {
        let val = self.val + other.get_val_as(self.unit);
        Length {
            val,
            unit: self.unit,
        }
    }
}
impl AddAssign<Length> for Length {
    fn add_assign(&mut self, other: Length) {
        self.add_other(&other);
    }
}
impl Sub<Length> for Length {
    type Output = Length;
    fn sub(self, other: Length) -> Length {
        let val = self.val - other.get_val_as(self.unit);
        Length {
            val,
            unit: self.unit,
        }
    }
}
impl SubAssign<Length> for Length {
    fn sub_assign(&mut self, other: Length) {
        self.subtract_other(&other);
    }
}
