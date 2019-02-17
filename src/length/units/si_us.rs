use super::{LengthUnit, LengthUnitList};

pub static MILLIMETER: LengthUnit = LengthUnit {
    abbr: "mm",
    singular: "millimeter",
    plural: "millimeters",
    in_metre: 1000.0,
};

pub static CENTIMETER: LengthUnit = LengthUnit {
    abbr: "cm",
    singular: "centimeter",
    plural: "centimeters",
    in_metre: 100.0,
};

pub static DECIMETER: LengthUnit = LengthUnit {
    abbr: "dm",
    singular: "decimeter",
    plural: "decimeters",
    in_metre: 10.0,
};

pub static METER: LengthUnit = LengthUnit {
    abbr: "m",
    singular: "meter",
    plural: "meters",
    in_metre: 1.0,
};

pub static KILOMETER: LengthUnit = LengthUnit {
    abbr: "km",
    singular: "kilometer",
    plural: "kilometers",
    in_metre: 0.001,
};

lazy_static! {
    pub static ref SI_US_LIST: LengthUnitList = LengthUnitList {
        title: "",
        units: vec!(&MILLIMETER, &CENTIMETER, &DECIMETER, &METER, &KILOMETER,),
    };
}
