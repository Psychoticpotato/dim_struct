use super::si::*;
use crate::unit_creation::*;

macro_rules! generate_si {
    ($typeVal: ty, $abbr: literal, $singular: literal, $plural: literal) => {
        for (entry in SI_ENTRIES) {

        }
    };
}
/// Generate an SI Unit with the specified info
pub fn gen_si<U: UnitTrait>(abbr: &str, singular: &str, plural: &str, entry: &SiEntry) -> U {
    let abbr = format!("{}{}", entry.abbr, abbr);
    let singular = format!("{}{}", entry.prefix, singular);
    let plural = format!("{}{}", entry.prefix, plural);
    U::new(
        Box::leak(abbr.into_boxed_str()),
        Box::leak(singular.into_boxed_str()),
        Box::leak(plural.into_boxed_str()),
        10_f32.powi(-entry.power).into(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::common::float::RoundTo;
    use crate::units::length::LengthUnit;
    #[test]
    fn gen_si_test() {
        // TODO: This is a brittle test
        let len: LengthUnit = gen_si("m", "metre", "metres", &SI_ENTRIES[0]);
        assert_eq!(len.get_abbr(), "dm");
        assert_eq!(len.get_plural(), "decimetres");
        assert_eq!(len.get_singular(), "decimetre");
        assert_eq!(len.in_base().round_to(5), 0.10);
        let len: LengthUnit = gen_si("m", "metre", "metres", &SI_ENTRIES[1]);
        assert_eq!(len.get_abbr(), "m");
        assert_eq!(len.get_plural(), "metres");
        assert_eq!(len.get_singular(), "metre");
        assert_eq!(len.in_base().round_to(5), 1.00);
    }
}