// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Volume`.

use crate::{area::Area, length::Length, prelude::*};

#[quantity(Length * Area)]
#[ref_unit(Cubic_Meter, "m³", NONE, "Reference unit of quantity `Volume`")]
#[unit(Cubic_Millimeter, "mm³", NANO, 0.000000001, "mm³")]
#[unit(Cubic_Centimeter, "cm³", MICRO, 0.000001, "cm³")]
#[unit(Milliliter, "ml", MICRO, 0.000001, "0.001·l")]
#[unit(Centiliter, "cl", 0.00001, "0.01·l")]
#[unit(Deciliter, "dl", 0.0001, "0.1·l")]
#[unit(Cubic_Decimeter, "dm³", MILLI, 0.001, "dm³")]
#[unit(Liter, "l", MILLI, 0.001, "0.001·m³")]
#[unit(Cubic_Kilometer, "km³", GIGA, 1000000000, "km³")]
/// The quantity expressing the amount of three-dimensional space enclosed by a
/// closed surface.
///
/// Definition: Length³
///
/// Reference unit: Cubic Meter ('m³')
///
/// Predefined units:
///
/// | Symbol | Name                  | Definition        | Equivalent in 'm³'  |
/// |--------|-----------------------|-------------------|---------------------|
/// | mm³    | Cubic Millimeter      | mm³               | 0.000000001         |
/// | cm³    | Cubic Centimeter      | cm³               | 0.000001            |
/// | ml     | Milliliter            | 0.001·l           | 0.000001            |
/// | cl     | Centiliter            | 0.01·l            | 0.00001             |
/// | dl     | Deciliter             | 0.1·l             | 0.0001              |
/// | dm³    | Cubic Decimeter       | dm³               | 0.001               |
/// | l      | Liter                 | 0.001·m³          | 0.001               |
/// | km³    | Cubic Kilometer       | km³               | 1000000000          |
pub struct Volume {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        area::{SQUARE_KILOMETER, SQUARE_METER},
        assert_almost_eq,
        length::{DECIMETER, MILLIMETER},
    };

    #[test]
    fn test_volume() {
        assert_eq!(<Volume as HasRefUnit>::REF_UNIT, VolumeUnit::REF_UNIT);
        assert!(CUBIC_METER.is_ref_unit());
        let amnt = 29.305;
        let v = amnt * CUBIC_DECIMETER;
        assert_eq!(v.value(), amnt);
        assert_eq!(v.unit(), CUBIC_DECIMETER);
        #[cfg(feature = "std")]
        assert_eq!(v.to_string(), "29.305 dm³");
    }

    #[test]
    fn test_length_mul_area() {
        let amnt = 2.1;
        let l = amnt * DECIMETER;
        let a = l * l;
        let v = l * a;
        assert_almost_eq!(v.value(), amnt * amnt * amnt);
        assert_eq!(v.unit(), CUBIC_DECIMETER);
        let b = 0.02 * SQUARE_KILOMETER;
        let h = amnt * DECIMETER;
        let v = b * h;
        assert_almost_eq!(v.value(), 2000. * amnt);
        assert_eq!(v.unit(), CUBIC_METER);
    }

    #[test]
    fn test_volume_div_length() {
        let amnt = -0.42;
        let v = amnt * LITER;
        let a = 0.7 * SQUARE_METER;
        let h = v / a;
        assert_almost_eq!(h.value(), v.value() / a.value());
        assert_eq!(h.unit(), MILLIMETER);
    }
}
