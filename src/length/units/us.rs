use crate::common::Unit;

pub static INCH: Unit = Unit {
    abbr: "in",
    singular: "inch",
    plural: "inches",
    in_base: 39.37007874,
};

pub static FOOT: Unit = Unit {
    abbr: "ft",
    singular: "foot",
    plural: "feet",
    in_base: 3.280839895,
};

pub static YARD: Unit = Unit {
    abbr: "yd",
    singular: "yard",
    plural: "yards",
    in_base: 1.093613298,
};

pub static MILE: Unit = Unit {
    abbr: "mi",
    singular: "mile",
    plural: "miles",
    in_base: 0.000621371,
};
