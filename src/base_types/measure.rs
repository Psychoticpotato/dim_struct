use crate::base_types::UnitTrait;
use crate::common::{Float, RoundTo};
/// The trait from which all measurement units are derived
pub trait MeasureTrait {
    /// The stored unit is of this type
    type Unit: UnitTrait;
    /// Returns the value in the current unit
    fn get_val(&self) -> Float;
    /// Returns the currently stored unit
    fn get_unit(&self) -> &Self::Unit;
    /// Adds the other value to this one
    fn add_other(&mut self, other: &Self);
    /// Subtracts the other value from this one
    fn subtract_other(&mut self, other: &Self);
}
/// These methods are for INTERNAL use only (consider these private methods).
///
/// IT IS HIGHLY ADVISED **NOT** TO BRING THIS INTO SCOPE.
///
/// These directly set the internally stored values WITHOUT modifying anything else.
pub trait MeasureInternalSetters<T: MeasureTrait> {
    /// Sets the internal value directly
    ///
    /// **IMPORTANT**
    ///
    /// ONLY TO BE USED INTERNALLY
    ///
    /// This is only public so custom measurements can be created.
    fn set_val_internal(&mut self, val: Float);
    /// Sets the internal unit directly
    ///
    /// **IMPORTANT**
    ///
    /// ONLY TO BE USED INTERNALLY
    ///
    /// This is only public so custom measurements can be created.
    fn set_unit_internal(&mut self, val: &'static T::Unit);
}

pub trait MeasureConvert<T: MeasureTrait>
where
    Self: MeasureTrait,
    Self: MeasureInternalSetters<T>,
{
    /// Returns the value stored in the specified unit (without mutating)
    fn get_val_as(&self, unit: &'static T::Unit) -> Float {
        let val = self.get_val();
        let from = self.get_unit();
        Self::convert(val, &from, unit)
        // convert::<T>(self.get_val(), &self.get_unit(), &unit)
    }
    /// Convert the value from and to the given units
    fn convert(val: Float, from: &Self::Unit, to: &T::Unit) -> Float {
        val / from.in_base() * to.in_base()
    }
    /// Converts the value stored to the new unit and stores the unit
    fn convert_to(&mut self, new_unit: &'static T::Unit) {
        let val = self.get_val();
        let from = self.get_unit();
        let val = Self::convert(val, &from, new_unit);
        // Convert the value over
        self.set_val_internal(val);
        // Set the unit
        self.set_unit_internal(new_unit);
    }
}

pub trait MeasureDisplay {
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// value = 1.5, decimals = 2, unit = "METRE"; result = `1.50m`
    fn display_abbr(&self, decimals: usize) -> String;
    /// Displays the value with the appropriate singular or plural name after it
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// If plural: `1.5 metres` or `0.75 metres`
    /// If singular, `1 metre`
    fn display(&self, decimals: usize) -> String;
    /// Displays the value with the plural name after it (and a space between).
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// The plurality of the value is not considered (see Dim.display(decimals)).
    /// EX: `1.0 metres`, `1.5 metres`
    fn display_plural(&self, decimals: usize) -> String;
    /// Displays the value with the singular name after it (and a space between).
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// The plurality of the value is not considered (see Dim.display(decimals)).
    /// EX: `1.0 metre`, `1.5 metre`
    fn display_singular(&self, decimals: usize) -> String;
}
// Move on to default impl blocks
impl<T: MeasureTrait> MeasureDisplay for T {
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// value = 1.5, decimals = 2, unit = "METRE"; result = `1.50m`
    fn display_abbr(&self, decimals: usize) -> String {
        let val = self.get_val().round_to(decimals);
        format!("{:.*}{}", decimals, val, self.get_unit().get_abbr())
    }
    /// Displays the value with the appropriate singular or plural name after it
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// If plural: `1.5 metres` or `0.75 metres`
    /// If singular, `1 metre`
    fn display(&self, decimals: usize) -> String {
        let val = self.get_val().round_to(decimals);
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
    fn display_plural(&self, decimals: usize) -> String {
        let val = self.get_val().round_to(decimals);
        format!("{:.*} {}", decimals, val, self.get_unit().get_plural())
    }

    /// Displays the value with the singular name after it (and a space between).
    /// The value is rounded to the number of decimals (and will show trailing `0`)
    /// The plurality of the value is not considered (see Dim.display(decimals)).
    /// EX: `1.0 metre`, `1.5 metre`
    fn display_singular(&self, decimals: usize) -> String {
        let val = self.get_val().round_to(decimals);
        format!("{:.*} {}", decimals, val, self.get_unit().get_singular())
    }
}
