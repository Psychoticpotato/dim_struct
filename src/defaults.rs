/// The default Float type (currently f64).
/// This is to avoid having to change hundreds of units in the event of an f128
pub type Float = f64;
impl RoundTo for Float {
    /// Returns a floating point number rounded to the specified digits
    fn round_to(&self, decimals: usize) -> Float {
        let decimals: i32 = decimals as i32;
        let mut temp = self * 10.0_f64.powi(decimals);
        temp = temp.round();
        temp /= 10.0_f64.powi(decimals);
        temp
    }
}
pub trait RoundTo {
    /// Returns a floating point number rounded to the specified digits
    fn round_to(&self, decimals: usize) -> Float;
}

#[cfg(test)]
mod test {
    use super::{Float, RoundTo};
    #[test]
    fn test_round() {
        let val: Float = 125.0;
        assert_eq!(val.round_to(2), 125.0);
        assert_eq!(val.round_to(0), 125.0);
        let val: Float = 125.1250;
        assert_eq!(val.round_to(0), 125.0);
        assert_eq!(val.round_to(1), 125.1);
        assert_eq!(val.round_to(2), 125.13);
        assert_eq!(val.round_to(3), 125.125);
    }
}
