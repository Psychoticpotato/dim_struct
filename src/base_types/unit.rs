use crate::common::serial::SERIAL_REGEX;
use crate::common::Float;
/// The trait from which all units must derive
pub trait UnitTrait: Sized + PartialEq {
    /// Generate a new UnitTrait
    fn new(abbr: &'static str,singular: &'static str,plural: &'static str, in_base: Float) -> Self;
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
pub struct UnitList<'l, U: UnitTrait> {
    /// Title of these units
    title: &'static str,
    /// The list of stored units
    units: Vec<&'l U>,
}
impl<U: UnitTrait> UnitList<'static, U> {
    /// Generates a new UnitList with the given info
    pub fn new(title: &'static str, units: Vec<&'static U>) -> Self {
        Self { title, units }
    }
    /// Return the title of this set of units
    pub fn get_title(&self) -> &str {
        self.title
    }
    /// Grab the full list of units for this set
    pub fn get_list(&self) -> &Vec<&'static U> {
        &self.units
    }
    /// Grab the full list of units for this set
    pub fn get_list_mut(&mut self) -> &mut Vec<&'static U> {
        &mut self.units
    }
    /// Parse the given string and returns:
    /// - The floating point value
    /// - The unit type for this value
    ///
    /// If not properly parsed, returns None
    pub fn parse_str(&self, test_val: &str) -> Option<(Float, &'static U)> {
        // Grab the capture group
        let captures = SERIAL_REGEX.captures(test_val)?;
        // The value from the unit
        let val = captures.get(1).unwrap().as_str().trim();
        // The unit string
        let unit_str = captures.get(2).unwrap().as_str();
        // The resulting unit (if found)
        let result = self.find_in_list(unit_str)?;
        // Parse the number
        let val = val
            .replace(" ", "") // Remove the whitespace
            .parse::<Float>() // Parse into a float
            .expect(format!("Matched Str:{}", val).as_str()); // Yell if something bad happens

        // Return the result (if this point is ever reached)
        Some((val, result))
    }
    /// Attempts to find the given string in the list.
    ///
    /// This can be:
    /// - Abbreviation
    /// - Singular
    /// - Plural
    ///
    /// If not found, `None` is returned.
    pub fn find_in_list(&self, unit_str: &str) -> Option<&'static U> {
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
    /// Merges the list of Unit Lists with this one.
    ///
    /// Anything that evalutes to equal during merge is not added.
    pub fn merge_other(&mut self, other: &Vec<&'static U>) {
        // Grab the list upon which we will operate
        let list = self.get_list_mut();
        for entry in other {
            // If we don't already have this entry, add it
            if !list.contains(entry) {
                list.push(entry);
            }
        }
    }
}
