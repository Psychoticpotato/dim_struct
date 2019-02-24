use super::systems::si::*;
use super::LengthUnit;
use crate::base_types::{UnitListTrait, UnitTrait};
use crate::common::*;
#[test]
fn test_si_list() {
    let test_list = vec![
        TestStruct {
            // Test Metre
            inputs: vec![
                "12.5 m",                     // Basic abbreviated test
                "12.500000000000000000001 m", // A test that goes over the precision
                "12.5 metre",                 // Test of singular
                "12.5 metres",                // Test of plural
                "12.5metres",                 // Test of no space
                "12.5    metres",             // Test of many spaces
            ],
            output: Some((12.5, &METRE)),
        },
        TestStruct {
            // Test Other ways it might be displayed
            inputs: vec![
                "12 m",       // Test without decimals
                "12.0 m",     // Test with decimals
                "12.00000 m", // Test with too many decimals
            ],
            output: Some((12.0, &METRE)),
        },
        TestStruct {
            // Test Other ways it might be displayed
            inputs: vec![
                "-11 m",              // Test without decimals
                "- 11.0 m",           // Test with decimals
                "  -     11.00000 m", // Test with too many decimals and spaces
            ],
            output: Some((-11.0, &METRE)),
        },
        TestStruct {
            // Test with fewer than 1 of the unit
            inputs: vec![
                "0.15 m",        // With leading zero
                ".15 m",         // Without leading zero
                "000.1500000 m", // Too many leading (and trailing) zeros
            ],
            output: Some((0.15, &METRE)),
        },
        // Test Centimetre
        TestStruct {
            inputs: vec!["13.5 cm", "13.5 centimetre", "13.5 centimetres"],
            output: Some((13.5, &CENTIMETRE)),
        },
        // Test Decimetre
        TestStruct {
            inputs: vec!["14.5 dm", "14.5 decimetre", "14.5 decimetres"],
            output: Some((14.5, &DECIMETRE)),
        },
        // Test Kilometre
        TestStruct {
            inputs: vec!["15.5 km", "15.5 kilometre", "15.5 kilometres"],
            output: Some((15.5, &KILOMETRE)),
        },
        // Test the thing(s) that should not be
        TestStruct {
            inputs: vec!["15.5.5 km", "15.A km", "NaN km", "12.5 mi"],
            output: None,
        },
    ];
    for val in test_list {
        run_test(val);
    }
}
fn run_test(val: TestStruct) {
    for input in val.inputs {
        let res = SI_LIST.parse_str(input);
        match res {
            Some(res) => {
                let val = val.output.expect(format!("Input:{}", input).as_str());
                assert_eq!(val.0, res.0);
                let val = val.1;
                let res = res.1;
                // Ensure these match exactly
                assert_eq!(val.get_singular(), res.get_singular());
                assert_eq!(val.get_plural(), res.get_plural());
                assert_eq!(val.get_abbr(), res.get_abbr());
                assert_eq!(val.in_base(), res.in_base());
            }
            None => match val.output {
                Some(_) => assert!(false, "Value not found"),
                None => (),
            },
        }
    }
}
struct TestStruct {
    /// A list of inputs that equal the output
    inputs: Vec<&'static str>,
    /// The output of each operation
    output: Option<(Float, &'static LengthUnit)>,
}
