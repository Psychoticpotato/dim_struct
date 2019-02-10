use crate::common::Unit;

pub static CENTIMETRE: Unit = Unit {
    abbr: "cm",
    singular: "centimetre",
    plural: "centimetres",
    in_base: 100.0,
};

pub static METRE: Unit = Unit {
    abbr: "m",
    singular: "metre",
    plural: "metres",
    in_base: 1.0,
};

pub static MILLIMETRE: Unit = Unit {
    abbr: "mm",
    singular: "millimetre",
    plural: "millimetres",
    in_base: 1000.0,
};

pub static KILOMETRE: Unit = Unit {
    abbr: "km",
    singular: "kilometre",
    plural: "kilometres",
    in_base: 0.001,
};
