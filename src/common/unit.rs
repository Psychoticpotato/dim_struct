use super::Float;

/// A struct to hold a specific unit
pub struct Unit {
    /// The abbreviation for this unit
    pub abbr: &'static str,
    /// Full name of this unit (singular)
    pub singular: &'static str,
    /// Plural name of this unit
    pub plural: &'static str,
    /// How many of this unit does it take to make a base unit.
    ///
    /// Base Units:
    /// - length: metre
    /// - area: square metre
    /// - volume: litre
    /// - mass: gram
    /// - TODO: finish out these
    pub in_base: Float,
}
