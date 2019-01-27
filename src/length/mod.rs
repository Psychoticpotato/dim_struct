use crate::defaults::Float;
use std::ops::{Add, AddAssign, Sub, SubAssign};
pub mod units;
use units::si::*;
use units::Unit;
#[derive(Clone, Copy)]
/// A struct to store a floating point number, and a unit
/// The benefit from this is conversion safety and various methods
pub struct Dim {
    val: Float,
    unit: &'static Unit,
}
/// Begin with static functions
impl Dim {
    /// Create a new Dimension with the given value and unit
    pub fn new(val: Float, unit: &'static Unit) -> Dim {
        Dim { val, unit }
    }
    /// Convert the value from and to the given units
    pub fn convert(val: Float, from: &'static Unit, to: &'static Unit) -> Float {
        val / from.in_meter * to.in_meter
    }
}

// Getters in this one
impl Dim {
    /// Returns the value in the current unit
    pub fn get_val(&self) -> Float {
        self.val
    }
    /// Returns the value stored in the specified unit (without mutating)
    pub fn get_val_as(&self, unit: &'static Unit) -> Float {
        Dim::convert(self.val, self.unit, unit)
    }
    /// Returns the current value, formatted in the following manner:
    /// If plural: `1.5 meters`
    /// If singular, `1 meter`
    /// TODO: specify decimal points
    pub fn display_abbr(&self, precision: u8) -> String {
        format!("{}{}", self.get_val(), self.unit.abbr)
    }
    /// If plural: `1.5 meters`
    /// If singular, `1 meter`
    /// TODO: specify decimal points
    pub fn display_full(&self, precision: u8) -> String {
        if self.val == 1.0 {
            format!("{} {}", self.get_val(), self.unit.singular)
        } else {
            format!("{} {}", self.get_val(), self.unit.plural)
        }
    }
}
// Setters here
impl Dim {
    /// Converts the value stored to the new unit and stores the unit
    pub fn convert_to(&mut self, new_unit: &'static Unit) {
        // Convert the value over
        self.val = Dim::convert(self.val, self.unit, new_unit);
        // Set the unit
        self.unit = new_unit;
    }
}
// Operators here
impl Dim {
    /// Adds the other value to this one
    pub fn add_other(&mut self, other: Dim) {
        self.val += other.get_val_as(self.unit);
    }
    /// Subtracts the other value from this one
    pub fn subtract_other(&mut self, other: Dim) {
        self.val -= other.get_val_as(self.unit);
    }
}
// Implement the various operators
impl Add for Dim {
    type Output = Dim;
    fn add(self, other: Dim) -> Dim {
        // Get these values as meter
        let val = self.get_val_as(&METER) + other.get_val_as(&METER);
        // Adjust them to the first value's unit
        let val = Dim::convert(val, &METER, self.unit);
        Dim {
            val,
            unit: self.unit,
        }
    }
}
impl Sub for Dim {
    type Output = Dim;
    fn sub(self, other: Dim) -> Dim {
        // Get these values as meter
        let val = self.get_val_as(&METER) - other.get_val_as(&METER);
        // Adjust them to the first value's unit
        let val = Dim::convert(val, &METER, self.unit);
        Dim {
            val,
            unit: self.unit,
        }
    }
}
impl AddAssign for Dim {
    fn add_assign(&mut self, other: Dim) {
        // Simply use add_other
        self.add_other(other);
    }
}
impl SubAssign for Dim {
    fn sub_assign(&mut self, other: Dim) {
        // Simply use add_other
        self.subtract_other(other);
    }
}
// Move on to the tests
#[cfg(test)]
mod test {
    use super::Dim;
    use super::*;
    #[test]
    fn test_metric() {
        // Start out with a basic number
        let mut first = Dim::new(0.5, &METER);
        // Ensure the most basic thing; that it actually is the same
        assert_eq!(first.get_val(), 0.5);
        // Create some kilometers
        let second = Dim::new(0.75, &KILOMETER);
        assert_eq!(second.get_val_as(&METER), 750.0);
        assert_eq!(second.get_val_as(&CENTIMETER), 75000.0);
        assert_eq!(second.get_val_as(&MILLIMETER), 750000.0);

        // Add some kilometers to it
        first += second;
        assert_eq!(first.get_val(), 750.50);
        // subtract it back out
        first -= second;
        assert_eq!(first.get_val(), 0.5);
        // Create one from millimeter
        let third = Dim::new(17.3, &MILLIMETER);
        assert_eq!(third.get_val_as(&METER), 0.0173);
        assert_eq!(third.get_val_as(&CENTIMETER), 1.73);
        assert_eq!(third.get_val_as(&MILLIMETER), 17.3);

        // Add to get a new value
        let dim = second + third;
        assert_eq!(dim.get_val(), 0.7500173);
        let dim = second - third;
        // Subtract, as well
        assert_eq!(dim.get_val(), 0.7499827);
    }
}
