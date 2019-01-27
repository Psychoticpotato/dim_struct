use crate::length::units::Unit;

pub static METER: Unit = Unit {
    abbr: "m",
    singular: "meter",
    plural: "meters",
    in_meter: 1.0,
};

pub static CENTIMETER: Unit = Unit {
    abbr: "cm",
    singular: "centimeter",
    plural: "centimeters",
    in_meter: 100.0,
};

pub static MILLIMETER: Unit = Unit {
    abbr: "mm",
    singular: "millimeter",
    plural: "millimeters",
    in_meter: 1000.0,
};

pub static KILOMETER: Unit = Unit {
    abbr: "km",
    singular: "kilometer",
    plural: "kilometers",
    in_meter: 0.001,
};
