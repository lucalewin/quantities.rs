// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of basic quantity `Length`.

use quantities::prelude::*;

#[quantity]
#[ref_unit(Meter, "m", NONE, "Reference unit of quantity `Length`")]
#[unit(Nanometer, "nm", NANO, 0.000000001, "0.000000001·m")]
#[unit(Micrometer, "µm", MICRO, 0.000001, "0.000001·m")]
#[unit(Millimeter, "mm", MILLI, 0.001, "0.001·m")]
#[unit(Centimeter, "cm", CENTI, 0.01, "0.01·m")]
#[unit(Decimeter, "dm", DECI, 0.1, "0.1·m")]
#[unit(Kilometer, "km", KILO, 1000, "1000·m")]
/// The quantity of distance between two points in spacetime.
///
/// Reference unit: Meter ('m')
///
/// Predefined units:
///
/// | Symbol | Name                    | Definition        | Equivalent in 'm' |
/// |--------|-------------------------|-------------------|-------------------|
/// | nm     | Nanometer               | 0.000000001·m     | 0.000000001       |
/// | µm     | Micrometer              | 0.000001·m        | 0.000001          |
/// | mm     | Millimeter              | 0.001·m           | 0.001             |
/// | cm     | Centimeter              | 0.01·m            | 0.01              |
/// | in     | Inch                    | 2.54·cm           | 0.0254            |
/// | dm     | Decimeter               | 0.1·m             | 0.1               |
/// | ft     | Foot                    | 12·in             | 0.3048            |
/// | yd     | Yard                    | 3·ft              | 0.9144            |
/// | ch     | Chain                   | 22·yd             | 20.1168           |
/// | fur    | Furlong                 | 10·ch             | 201.168           |
/// | km     | Kilometer               | 1000·m            | 1000              |
/// | mi     | Mile                    | 8·fur             | 1609.344          |
pub struct Length {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(<Length as HasRefUnit>::REF_UNIT, LengthUnit::REF_UNIT);
        assert!(METER.is_ref_unit());
        let amnt = 29.35;
        let l = amnt * CENTIMETER;
        assert_eq!(l.value, amnt);
        assert_eq!(l.unit, CENTIMETER);
        #[cfg(feature = "std")]
        assert_eq!(l.to_string(), "29.35 cm");
    }
}
