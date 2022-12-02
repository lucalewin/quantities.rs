// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Frequency`.

use crate::{duration::Duration, prelude::*};

#[quantity(Amount / Duration)]
#[ref_unit(Hertz, "Hz", NONE, "Reference unit of quantity `Frequency`")]
#[unit(Kilohertz, "kHz", KILO, 1000, "1000·Hz")]
#[unit(Megahertz, "MHz", MEGA, 1000000, "1000000·Hz")]
#[unit(Gigahertz, "GHz", GIGA, 1000000000, "1000000000·Hz")]
/// Number of occurrences of a repeating event per unit of time
///
/// Definition: 1/Duration
///
/// Reference unit: Hertz ('Hz' = '1/s')
///
/// Predefined units:
///
/// | Symbol | Name                  | Definition        | Equivalent in 'Hz'  |
/// |--------|-----------------------|-------------------|---------------------|
/// | kHz    | Kilohertz             | 1000·Hz           | 1000                |
/// | MHz    | Megahertz             | 1000000·Hz        | 1000000             |
/// | GHz    | Gigahertz             | 1000000000·Hz     | 1000000000          |
pub struct Frequency {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_almost_eq, duration::MILLISECOND};

    #[test]
    fn test_amount_div_duration() {
        let a = 9030.;
        let at = 2.5;
        let t = at * MILLISECOND;
        let f = a / t;
        assert_almost_eq!(f.value(), a / at);
        assert_eq!(f.unit(), KILOHERTZ);
    }
}
