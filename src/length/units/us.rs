use crate::length::units::Unit;

pub static INCH: Unit = Unit {
    abbr: "in",
    singular: "inch",
    plural: "inches",
    in_meter: 39.37007874,
};

pub static FOOT: Unit = Unit {
    abbr: "ft",
    singular: "foot",
    plural: "feet",
    in_meter: 3.280839895,
};

pub static YARD: Unit = Unit {
    abbr: "yd",
    singular: "yard",
    plural: "yards",
    in_meter: 1.093613298,
};

pub static MILE: Unit = Unit {
    abbr: "mi",
    singular: "mile",
    plural: "miles",
    in_meter: 0.000621371,
};
