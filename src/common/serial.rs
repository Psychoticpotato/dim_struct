use lazy_static;
use regex::Regex;
lazy_static! {
    /// Regex to capture a serialized unit
    /// Capture groups are:
    /// 1. The value itself
    /// 2. The unit
    pub static ref SERIAL_REGEX: Regex = Regex::new(r"(?i)^ *([+-]? *(?:\d*\.?\d+)|(?:\d+\.?\d*)) *([a-z]+) *$").unwrap();
}

#[cfg(test)]
mod test {
    use super::SERIAL_REGEX;
    #[test]
    fn test_regex() {
        let vals: Vec<TestVals> = vec![
            TestVals {
                val: String::from("124m"),
                num: String::from("124"),
                unit: String::from("m"),
            },
            TestVals {
                val: String::from("124 m"),
                num: String::from("124"),
                unit: String::from("m"),
            },
            TestVals {
                val: String::from("124.25 m"),
                num: String::from("124.25"),
                unit: String::from("m"),
            },
            TestVals {
                val: String::from("124. AsDf"),
                num: String::from("124."),
                unit: String::from("AsDf"),
            },
            // Without a trailing zero
            TestVals {
                val: String::from(".75 PieFace"),
                num: String::from(".75"),
                unit: String::from("PieFace"),
            },
            // With the leading plus
            TestVals {
                val: String::from("+ .85 WithPlus"),
                num: String::from("+ .85"),
                unit: String::from("WithPlus"),
            },
            // With a leading negative
            TestVals {
                val: String::from("- .85 WithMinus"),
                num: String::from("- .85"),
                unit: String::from("WithMinus"),
            },
            TestVals {
                val: String::from("0.25 AsDf"),
                num: String::from("0.25"),
                unit: String::from("AsDf"),
            },
        ];
        // run the test
        run_test(vals);
    }

    fn run_test(vals: Vec<TestVals>) {
        for val in vals {
            let res = SERIAL_REGEX.captures(&val.val).unwrap();
            // Verify these values
            assert_eq!(res.get(1).unwrap().as_str(), val.num);
            assert_eq!(res.get(2).unwrap().as_str(), val.unit);
        }
    }
    struct TestVals {
        val: String,
        num: String,
        unit: String,
    }
}
