use crate::common::Float;
/// Module with SI units (Metre and such)
pub mod si;
/// Module with American spellings of SI units (Meter and such)
pub mod si_us;
/// Module with the United States customary units (Inche and such)
pub mod us;
/// A struct to hold a specific unit
pub struct Unit {
    /// The abbreviation for this unit
    pub abbr: &'static str,
    /// Full name of this unit (singular)
    pub singular: &'static str,
    /// Plural name of this unit
    pub plural: &'static str,
    /// How many of this unit does it take to make a metre (base unit)
    pub in_metre: Float,
}
