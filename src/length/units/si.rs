use super::LengthUnit;

pub static CENTIMETRE: LengthUnit = LengthUnit {
    abbr: "cm",
    singular: "centimetre",
    plural: "centimetres",
    in_metre: 100.0,
};

pub static METRE: LengthUnit = LengthUnit {
    abbr: "m",
    singular: "metre",
    plural: "metres",
    in_metre: 1.0,
};

pub static MILLIMETRE: LengthUnit = LengthUnit {
    abbr: "mm",
    singular: "millimetre",
    plural: "millimetres",
    in_metre: 1000.0,
};

pub static KILOMETRE: LengthUnit = LengthUnit {
    abbr: "km",
    singular: "kilometre",
    plural: "kilometres",
    in_metre: 0.001,
};
