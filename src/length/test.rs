// Move on to the tests
use super::units::si::*;
use super::Length;
use crate::base_types::*;

#[test]
fn test_add_sub() {
    // Start out with a basic number
    let mut first = Length::new(0.5, &METRE);
    // Ensure the most basic thing; that it actually is the same
    assert_eq!(first.get_val(), 0.5);
    // Create some kilometres
    let second = Length::new(0.75, &KILOMETRE);
    assert_eq!(second.get_val_as(&METRE), 750.0);
    assert_eq!(second.get_val_as(&CENTIMETRE), 75000.0);
    assert_eq!(second.get_val_as(&MILLIMETRE), 750000.0);

    // Add some kilometres to it
    first += second;
    assert_eq!(first.get_val(), 750.50);
    // subtract it back out
    first -= second;
    assert_eq!(first.get_val(), 0.5);
    // Create one from millimetre
    let third = Length::new(17.3, &MILLIMETRE);
    assert_eq!(third.get_val_as(&METRE), 0.0173);
    assert_eq!(third.get_val_as(&CENTIMETRE), 1.73);
    assert_eq!(third.get_val_as(&MILLIMETRE), 17.3);

    // Add to get a new value
    let length = second + third;
    assert_eq!(length.get_val(), 0.7500173);
    let length = second - third;
    // Subtract, as well
    assert_eq!(length.get_val(), 0.7499827);
}
#[test]
fn test_str() {
    // Test the singular and plural
    let val = Length::new(1.0, &METRE);
    assert_eq!(val.display(0), "1 metre");
    assert_eq!(val.display(1), "1.0 metre");
    assert_eq!(val.display(2), "1.00 metre");
    assert_eq!(val.display_plural(2), "1.00 metres");
    assert_eq!(val.display_abbr(1), "1.0m");

    let val = Length::new(0.5, &METRE);
    assert_eq!(val.display(1), "0.5 metres");
    assert_eq!(val.display(2), "0.50 metres");
    assert_eq!(val.display_singular(1), "0.5 metre");
    // Test various rounding
    let val = Length::new(0.125, &METRE);
    assert_eq!(val.display(0), "0 metres");
    assert_eq!(val.display(1), "0.1 metres");
    assert_eq!(val.display(2), "0.13 metres");
    assert_eq!(val.display(3), "0.125 metres");
    assert_eq!(val.display(4), "0.1250 metres");
    assert_eq!(val.display_abbr(3), "0.125m");
    // Check with a whole number
    let val = Length::new(24.0, &METRE);
    assert_eq!(val.display(0), "24 metres");
    assert_eq!(val.display(1), "24.0 metres");
    assert_eq!(val.display(2), "24.00 metres");
    assert_eq!(val.display(3), "24.000 metres");
    assert_eq!(val.display(4), "24.0000 metres");
    assert_eq!(val.display_abbr(3), "24.000m");
}
#[test]
fn convert() {
    let mut val = Length::new(24.0, &KILOMETRE);
    // Best to ensure that we have a decent starting ground
    assert_eq!(val.get_val(), 24.0);
    // Check the number of millimetres
    assert_eq!(val.get_val_as(&MILLIMETRE), 24000000.0);
    val.convert_to(&METRE);
    // Check after this conversion
    assert_eq!(val.get_val(), 24000.0);
    // Check after converting, to ensure that the millimetres line up
    assert_eq!(val.get_val_as(&MILLIMETRE), 24000000.0);
    assert_eq!(val.get_val_as(&KILOMETRE), 24.0);
}

#[test]
fn check_borrow() {
    let mut val = Length::new(125.0, &CENTIMETRE);
    let val2 = val.clone();
    borrow(&mut val, &val2);
    assert_eq!(val.get_val(), 250.0);
}
// TODO: FIGURE OUT PROPER BORROW AND OPERATORS
fn borrow(val: &mut Length, val2: &Length) {
    val.add_other(val2);
}
