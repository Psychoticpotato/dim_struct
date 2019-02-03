// Move on to the tests
use super::Dim;
use super::*;

#[test]
fn test_add_sub() {
    // Start out with a basic number
    let mut first = Dim::new(0.5, &METER);
    // Ensure the most basic thing; that it actually is the same
    assert_eq!(first.get_val(), 0.5);
    // Create some kilometers
    let second = Dim::new(0.75, &KILOMETER);
    assert_eq!(second.get_val_as(&METER), 750.0);
    assert_eq!(second.get_val_as(&CENTIMETER), 75000.0);
    assert_eq!(second.get_val_as(&MILLIMETER), 750000.0);

    // Add some kilometers to it
    first += second;
    assert_eq!(first.get_val(), 750.50);
    // subtract it back out
    first -= second;
    assert_eq!(first.get_val(), 0.5);
    // Create one from millimeter
    let third = Dim::new(17.3, &MILLIMETER);
    assert_eq!(third.get_val_as(&METER), 0.0173);
    assert_eq!(third.get_val_as(&CENTIMETER), 1.73);
    assert_eq!(third.get_val_as(&MILLIMETER), 17.3);

    // Add to get a new value
    let dim = second + third;
    assert_eq!(dim.get_val(), 0.7500173);
    let dim = second - third;
    // Subtract, as well
    assert_eq!(dim.get_val(), 0.7499827);
}
#[test]
fn test_str() {
    // Test the singular and plural
    let val = Dim::new(1.0, &METER);
    assert_eq!(val.display(0), "1 meter");
    assert_eq!(val.display(1), "1.0 meter");
    assert_eq!(val.display(2), "1.00 meter");
    assert_eq!(val.display_plural(2), "1.00 meters");
    assert_eq!(val.display_abbr(1), "1.0m");

    let val = Dim::new(0.5, &METER);
    assert_eq!(val.display(1), "0.5 meters");
    assert_eq!(val.display(2), "0.50 meters");
    assert_eq!(val.display_singular(1), "0.5 meter");
    // Test various rounding
    let val = Dim::new(0.125, &METER);
    assert_eq!(val.display(0), "0 meters");
    assert_eq!(val.display(1), "0.1 meters");
    assert_eq!(val.display(2), "0.13 meters");
    assert_eq!(val.display(3), "0.125 meters");
    assert_eq!(val.display(4), "0.1250 meters");
    assert_eq!(val.display_abbr(3), "0.125m");
    // Check with a whole number
    let val = Dim::new(24.0, &METER);
    assert_eq!(val.display(0), "24 meters");
    assert_eq!(val.display(1), "24.0 meters");
    assert_eq!(val.display(2), "24.00 meters");
    assert_eq!(val.display(3), "24.000 meters");
    assert_eq!(val.display(4), "24.0000 meters");
    assert_eq!(val.display_abbr(3), "24.000m");
}
#[test]
fn convert() {
    let mut val = Dim::new(24.0, &KILOMETER);
    // Best to ensure that we have a decent starting ground
    assert_eq!(val.get_val(), 24.0);
    // Check the number of millimeters
    assert_eq!(val.get_val_as(&MILLIMETER), 24000000.0);
    val.convert_to(&METER);
    // Check after this conversion
    assert_eq!(val.get_val(), 24000.0);
    // Check after converting, to ensure that the millimeters line up
    assert_eq!(val.get_val_as(&MILLIMETER), 24000000.0);
    assert_eq!(val.get_val_as(&KILOMETER), 24.0);
}

#[test]
fn check_borrow() {
    let mut val = Dim::new(125.0, &CENTIMETER);
    let val2 = val.clone();
    borrow(&mut val, &val2);
    assert_eq!(val.get_val(), 250.0);
}
// TODO: FIGURE OUT PROPER BORROW AND OPERATORS
fn borrow(dim: &mut Dim, dim2: &Dim) {
    dim.add_other(dim2);
}
