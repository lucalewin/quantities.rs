#[cfg(target_pointer_width = "32")]
pub type Amount = f32;

#[cfg(target_pointer_width = "64")]
pub type Amount = f64;

pub const AMNT_ZERO: Amount = 0.;
pub const AMNT_ONE: Amount = 1.;

#[doc(hidden)]
#[macro_export]
macro_rules! assert_almost_eq {
    ($x:expr, $y:expr) => {
        let t = if ($x).abs() >= ($y).abs() {
            ($x).abs() / (10_f64).powi(Amount::DIGITS as i32)
        } else {
            ($y).abs() / (10_f64).powi(Amount::DIGITS as i32)
        };
        assert!(($x - $y).abs() < t, "{} ≉ {}", ($x), ($y));
    };
}
