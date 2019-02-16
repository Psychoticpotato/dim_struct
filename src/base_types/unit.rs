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
