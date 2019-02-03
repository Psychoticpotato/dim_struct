use crate::defaults::{Float, RoundTo};
use std::ops::{Add, AddAssign, Sub, SubAssign};
#[cfg(test)]
mod test;
pub mod units;

use units::si::*;
use units::Unit;
#[derive(Clone, Copy)]
/// A struct to store a floating point number, and a unit
/// The benefit from this is conversion safety and various methods
///
/// **IMPORTANT**:
///
/// This is `Clone` and `Copy`.  
/// Be careful, as after a move, the two instances are no longer tied.
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
}

// Various display functions here
impl Dim {
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// value = 1.5, decimals = 2, unit = "METER"; result = `1.50m`
    pub fn display_abbr(&self, decimals: usize) -> String {
        let val = self.val.round_to(decimals);
        format!("{:.*}{}", decimals, val, self.unit.abbr)
    }
    /// Displays the value with the appropriate singular or plural name after it
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// If plural: `1.5 meters` or `0.75 meters`
    /// If singular, `1 meter`
    pub fn display(&self, decimals: usize) -> String {
        let val = self.val.round_to(decimals);
        if val == 1.0 {
            self.display_singular(decimals)
        } else {
            self.display_plural(decimals)
        }
    }

    /// Displays the value with the plural name after it (and a space between).
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// The plurality of the value is not considered (see Dim.display(decimals)).
    /// EX: `1.0 meters`, `1.5 meters`
    pub fn display_plural(&self, decimals: usize) -> String {
        let val = self.val.round_to(decimals);
        format!("{:.*} {}", decimals, val, self.unit.plural)
    }

    /// Displays the value with the singular name after it (and a space between).
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// The plurality of the value is not considered (see Dim.display(decimals)).
    /// EX: `1.0 meter`, `1.5 meter`
    pub fn display_singular(&self, decimals: usize) -> String {
        let val = self.val.round_to(decimals);
        format!("{:.*} {}", decimals, val, self.unit.singular)
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
    pub fn add_other(&mut self, other: &Dim) {
        self.val += other.get_val_as(self.unit);
    }
    /// Subtracts the other value from this one
    pub fn subtract_other(&mut self, other: &Dim) {
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
        self.add_other(&other);
    }
}

impl SubAssign for Dim {
    fn sub_assign(&mut self, other: Dim) {
        // Simply use add_other
        self.subtract_other(&other);
    }
}
