use super::unit::UnitList;
use crate::base_types::UnitTrait;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::common::{Float, RoundTo};

/// Measure stores a measurable value.  
///
/// The unit type is determined by the supplied struct implementing UnitTrait.
///
/// TODO: LOTS MORE DOCUMENTATION ON THIS
pub struct Measure<'l, UnitVal: UnitTrait> {
    unit: &'l UnitVal,
    val: Float,
}
// Implement creation methods
impl<U: UnitTrait> Measure<'static, U> {
    /// Generates a new Measurement with the given value and unit
    pub fn new(val: Float, unit: &'static U) -> Measure<U> {
        Measure { unit, val }
    }
    /// Parses the literal string for a Measurement (ex: `12m`).
    ///
    /// The unit is checked against every unit found in the system provided.
    ///
    /// If there are multiple matches, the first one found is returned.
    /// The order of these should not be relied upon.
    ///
    /// Returns a Length struct if the value was parsed correctly
    pub fn from_literal(system: UnitList<'static, U>, val: &str) -> Option<Measure<'static, U>> {
        // Parse the string
        let res = system.parse_str(val)?;
        // If found, return a new Length
        Some(Measure {
            val: res.0,
            unit: res.1,
        })
    }
}
// Implement getters and add/subtract
impl<U: UnitTrait> Measure<'static, U> {
    /// Returns the value in the current unit
    pub fn get_val(&self) -> Float {
        self.val
    }
    /// Returns the currently stored unit
    pub fn get_unit(&self) -> &'static U {
        self.unit
    }
    /// Adds the other value to this one.  
    ///
    /// The conversion is done within this method.
    pub fn add_other(&mut self, other: &Self) {
        self.val += other.get_val_as(self.unit);
    }
    /// Subtracts the other value to this one.  
    ///
    /// The conversion is done within this method.
    pub fn subtract_other(&mut self, other: &Self) {
        self.val -= other.get_val_as(self.unit);
    }
}
// Implement conversion traits
impl<U: UnitTrait> Measure<'static, U> {
    /// Returns the value stored in the specified unit (without mutating)
    pub fn get_val_as(&self, unit: &U) -> Float {
        Self::convert(self.val, self.unit, unit)
    }
    /// Convert the value from and to the given units
    pub fn convert(val: Float, from: &U, to: &U) -> Float {
        val / from.in_base() * to.in_base()
    }
    /// Converts the value stored to the new unit and stores the unit
    pub fn convert_to(&mut self, new_unit: &'static U) {
        let val = self.val;
        let from = self.unit;
        // Convert the value over
        self.val = Self::convert(val, &from, new_unit);
        // Set the unit
        self.unit = new_unit;
    }
}
// Implement display traits
impl<U: UnitTrait> Measure<'static, U> {
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// value = 1.5, decimals = 2, unit = "METRE"; result = `1.50m`
    pub fn display_abbr(&self, decimals: usize) -> String {
        let val = self.val.round_to(decimals);
        format!("{:.*}{}", decimals, val, self.unit.get_abbr())
    }
    /// Displays the value with the appropriate singular or plural name after it
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// If plural: `1.5 metres` or `0.75 metres`
    /// If singular, `1 metre`
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
    /// EX: `1.0 metres`, `1.5 metres`
    pub fn display_plural(&self, decimals: usize) -> String {
        let val = self.val.round_to(decimals);
        format!("{:.*} {}", decimals, val, self.unit.get_plural())
    }
    /// Displays the value with the singular name after it (and a space between).
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// The plurality of the value is not considered (see Dim.display(decimals)).
    /// EX: `1.0 metre`, `1.5 metre`
    pub fn display_singular(&self, decimals: usize) -> String {
        let val = self.val.round_to(decimals);
        format!("{:.*} {}", decimals, val, self.unit.get_singular())
    }
}

// Add the various operators
impl<U: UnitTrait> Add<Self> for Measure<'static, U> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let val = self.val + other.get_val_as(self.unit);
        Measure {
            val,
            unit: self.unit,
        }
    }
}
impl<U: UnitTrait> AddAssign<Self> for Measure<'static, U> {
    fn add_assign(&mut self, other: Self) {
        self.add_other(&other);
    }
}
impl<U: UnitTrait> Sub<Self> for Measure<'static, U> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let val = self.val - other.get_val_as(self.unit);
        Measure {
            val,
            unit: self.unit,
        }
    }
}
impl<U: UnitTrait> SubAssign<Self> for Measure<'static, U> {
    fn sub_assign(&mut self, other: Self) {
        self.subtract_other(&other);
    }
}

// Implement the equality operator
impl<U: UnitTrait> std::cmp::PartialEq for Measure<'static, U> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.get_val_as(self.unit)
    }
}
// Implement clone and copy
impl<U: UnitTrait> Clone for Measure<'static, U> {
    fn clone(&self) -> Self {
        Self {
            val: self.val,
            unit: self.unit,
        }
    }
}
impl<U: UnitTrait> Copy for Measure<'static, U> {}
