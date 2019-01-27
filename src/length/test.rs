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
    let first = Dim::new(0.5, &METER);
    assert_eq!(first.display(2), "0.50 meters");
    assert_eq!(first.display(1), "0.5 meters");
}
