use crate::common::Unit;

pub static CENTIMETER: Unit = Unit {
    abbr: "cm",
    singular: "centimeter",
    plural: "centimeters",
    in_base: 100.0,
};

pub static METER: Unit = Unit {
    abbr: "m",
    singular: "meter",
    plural: "meters",
    in_base: 1.0,
};

pub static MILLIMETER: Unit = Unit {
    abbr: "mm",
    singular: "millimeter",
    plural: "millimeters",
    in_base: 1000.0,
};

pub static KILOMETER: Unit = Unit {
    abbr: "km",
    singular: "kilometer",
    plural: "kilometers",
    in_base: 0.001,
};
