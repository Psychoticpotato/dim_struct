use crate::defaults::Float;
pub mod si;
/// A struct to hold a specific unit
pub struct Unit {
    /// The abbreviation for this unit
    pub abbr: &'static str,
    /// Full name of this unit (singular)
    pub singular: &'static str,
    /// Plural name of this unit
    pub plural: &'static str,
    /// How many of this unit does it take to make a meter (base unit)
    pub in_meter: Float,
}
