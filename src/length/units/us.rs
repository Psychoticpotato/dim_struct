use super::{LengthUnit, LengthUnitList};

pub static INCH: LengthUnit = LengthUnit {
    abbr: "in",
    singular: "inch",
    plural: "inches",
    in_metre: 39.37007874,
};

pub static FOOT: LengthUnit = LengthUnit {
    abbr: "ft",
    singular: "foot",
    plural: "feet",
    in_metre: 3.280839895,
};

pub static YARD: LengthUnit = LengthUnit {
    abbr: "yd",
    singular: "yard",
    plural: "yards",
    in_metre: 1.093613298,
};

pub static MILE: LengthUnit = LengthUnit {
    abbr: "mi",
    singular: "mile",
    plural: "miles",
    in_metre: 0.000621371,
};

lazy_static! {
    /// The list of US Length units are stored here
    ///
    /// If you need a custom list, tough luck
    pub static ref US_LENGTH_LIST: LengthUnitList = LengthUnitList {
        title: "",
        units: vec!(&INCH, &FOOT, &YARD, &MILE,),
    };
}
