use super::super::LengthUnit;
use crate::unit_creation::UnitList;
pub static MILLIMETRE: LengthUnit = LengthUnit {
    abbr: "mm",
    singular: "millimetre",
    plural: "millimetres",
    in_metre: 1000.0,
};

pub static CENTIMETRE: LengthUnit = LengthUnit {
    abbr: "cm",
    singular: "centimetre",
    plural: "centimetres",
    in_metre: 100.0,
};

pub static DECIMETRE: LengthUnit = LengthUnit {
    abbr: "dm",
    singular: "decimetre",
    plural: "decimetres",
    in_metre: 10.0,
};

pub static METRE: LengthUnit = LengthUnit {
    abbr: "m",
    singular: "metre",
    plural: "metres",
    in_metre: 1.0,
};

pub static KILOMETRE: LengthUnit = LengthUnit {
    abbr: "km",
    singular: "kilometre",
    plural: "kilometres",
    in_metre: 0.001,
};

lazy_static! {
    pub static ref SI_LIST: UnitList<'static, LengthUnit> = UnitList::new (
        "SI",
        vec!(&MILLIMETRE, &CENTIMETRE, &DECIMETRE, &METRE, &KILOMETRE,),
    );
}
