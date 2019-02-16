use super::LengthUnit;

pub static CENTIMETER: LengthUnit = LengthUnit {
    abbr: "cm",
    singular: "centimeter",
    plural: "centimeters",
    in_metre: 100.0,
};

pub static METER: LengthUnit = LengthUnit {
    abbr: "m",
    singular: "meter",
    plural: "meters",
    in_metre: 1.0,
};

pub static MILLIMETER: LengthUnit = LengthUnit {
    abbr: "mm",
    singular: "millimeter",
    plural: "millimeters",
    in_metre: 1000.0,
};

pub static KILOMETER: LengthUnit = LengthUnit {
    abbr: "km",
    singular: "kilometer",
    plural: "kilometers",
    in_metre: 0.001,
};
