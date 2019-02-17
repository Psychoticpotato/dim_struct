use crate::common::serial::SERIAL_REGEX;
use crate::common::Float;
/// The trait from which all units must derive
pub trait UnitTrait: Sized {
    /// The abbreviation for this unit
    fn get_abbr(&self) -> &'static str;
    /// Full name of this unit (singular)
    fn get_singular(&self) -> &'static str;
    /// Plural name of this unit
    fn get_plural(&self) -> &'static str;
    /// How many of this unit does it take to make a base unit.
    ///
    /// Base Units:
    /// - length: metre
    /// - area: square metre
    /// - volume: litre
    /// - mass: gram
    /// - TODO: finish out these
    fn in_base(&self) -> Float;
}
/// A list of units for a particular system.
///
/// This is for something like SI length units, to avoid overlap of abbreviated units.
pub trait UnitListTrait<U: UnitTrait> {
    /// Return the title of this set of units
    fn get_title(&self) -> &str;
    /// Grab the full list of units for this set
    fn get_list(&self) -> &Vec<&'static U>;
    /// Parse the given string and returns:
    /// - The floating point value
    /// - The unit type for this value
    ///
    /// If not properly parsed, returns None
    fn parse_str(&self, test_val: &str) -> Option<(Float, &'static U)> {
        // Grab the capture group
        let captures = SERIAL_REGEX.captures(test_val)?;
        // The value from the unit
        let val = captures.get(1).unwrap().as_str().trim();
        // The unit string
        let unit_str = captures.get(2).unwrap().as_str();
        // The resulting unit (if found)
        let result = self.find_in_list(unit_str)?;
        // Parse the number
        let val = val.parse::<Float>().unwrap();
        // Return the result (if this point is ever reached)
        Some((val, result))
    }
    fn find_in_list(&self, unit_str: &str) -> Option<&'static U> {
        // Loop through each unit
        for unit in self.get_list() {
            // If any of these cases is true, we have found the unit
            if unit_str == unit.get_abbr()
                || unit_str == unit.get_singular()
                || unit_str == unit.get_plural()
            {
                return Some(&unit);
            }
            // Otherwise, keep looping
        }
        // If nothing is found, return that
        None
    }
    /// Add the given unit to the stored vec
    fn add_unit(&mut self, new_unit: &'static U);
}
