
/// An SI unit entry for prefix
pub struct SiEntry {
    /// Prefix that will be applied to the base unit
    pub prefix: &'static str,
    /// Abbreviation of the entry ("c" in "cm")
    pub abbr: &'static str,
    /// Power applied to 10 (EX: 3 for Kilo gives 1000)
    pub power: i32,
}
/// Manually keyed SI Entries
pub static SI_ENTRIES: [SiEntry; 2] = [
    SiEntry {
        prefix: "deci",
        abbr: "d",
        power: 1,
    },
    SiEntry {
        prefix: "",
        abbr: "",
        power: 0,
    },
];
