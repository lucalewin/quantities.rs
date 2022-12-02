// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of basic quantity `Mass`.

use quantities::prelude::*;

#[quantity]
#[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
#[unit(Milligram, "mg", MILLI, 0.000001, "0.001·g")]
#[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
#[unit(Tonne, "t", MEGA, 1000, "1000·kg")]
/// The quantity of matter in a physical body.
///
/// Also used as measure of a physical body's resistance to acceleration.
///
/// Reference unit: Kilogram ('kg')
///
/// Predefined units:
///
/// | Symbol | Name                   | Definition        | Equivalent in 'kg' |
/// |--------|------------------------|-------------------|--------------------|
/// | mg     | Milligram              | 0.001·g           | 0.000001           |
/// | ct     | Carat                  | 0.2·g             | 0.0002             |
/// | g      | Gram                   | 0.001·kg          | 0.001              |
/// | oz     | Ounce                  | 0.0625·lb         | 0.028349523125     |
/// | lb     | Pound                  | 0.45359237·kg     | 0.45359237         |
/// | st     | Stone                  | 14·lb             | 6.35029318         |
/// | t      | Tonne                  | 1000·kg           | 1000               |
pub struct Mass {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass() {
        assert_eq!(<Mass as HasRefUnit>::REF_UNIT, MassUnit::REF_UNIT);
        assert!(KILOGRAM.is_ref_unit());
        let mass = 29.35 * KILOGRAM;
        assert_eq!(mass.value, 29.35);
        assert_eq!(mass.unit, KILOGRAM);
        #[cfg(feature = "std")]
        assert_eq!(mass.to_string(), "29.35 kg");
    }
}
