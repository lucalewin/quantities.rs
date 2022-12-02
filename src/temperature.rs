// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of basic quantity `Temperature`.

use crate::{converter::ConversionTable, prelude::*};

#[rustfmt::skip]
#[quantity]
#[unit(Kelvin, "K", "K")]
#[unit(Degree_Celsius, "°C", "°C")]
#[unit(Degree_Fahrenheit, "°F", "°F")]
/// Measure of thermal energy
///
/// Predefined units:
///
/// | Symbol | Name              | Equivalents                   |
/// |--------|-------------------|-------------------------------|
/// | K      | Kelvin            | 0 K = -273,25 °C = -459.67 °F |
/// | °C     | Degree Celsius    | 0 °C = 32 °F = 273,25 K       |
/// | °F     | Degree Fahrenheit | 0 °F ≅ -17.778 °C ≅ 255.372 K |
///
/// Temperature units are converted using the following formulas:
///
/// | from \ to  | Kelvin                          | Celsius                      | Fahrenheit                    |
/// |------------|---------------------------------|------------------------------|-------------------------------|
/// | Kelvin     | -                               | \[°C\] = \[K\] - 273.15      | \[°F\] = \[K\] * 9/5 - 459.67 |
/// | Celsius    | \[K\] = \[°C\] + 273.15         | -                            | \[°F\] = \[°C\] * 9/5 + 32    |
/// | Fahrenheit | \[K\] = (\[°F\] + 459.67) * 5/9 | \[°C\] = (\[°F\] - 32) * 5/9 | -                             |
pub struct Temperature {}

/// Temperature conversion table
pub const TEMPERATURE_CONVERTER: ConversionTable<Temperature, 6> =
    ConversionTable {
        mappings: [
            (KELVIN, DEGREE_CELSIUS, 1.0, -273.15),
            (DEGREE_CELSIUS, KELVIN, 1.0, 273.15),
            (KELVIN, DEGREE_FAHRENHEIT, 1.8, -459.67),
            (
                DEGREE_FAHRENHEIT,
                KELVIN,
                0.555_555_555_555_555_6,
                255.372_222_222_222_23,
            ),
            (DEGREE_CELSIUS, DEGREE_FAHRENHEIT, 1.8, 32.0),
            (
                DEGREE_FAHRENHEIT,
                DEGREE_CELSIUS,
                0.555_555_555_555_555_6,
                -17.777_777_777_777_78,
            ),
        ],
    };

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_almost_eq, converter::Converter};

    #[test]
    fn test_temperature() {
        let amnt = 21.5;
        let m = amnt * KELVIN;
        assert_eq!(m.value, amnt);
        assert_eq!(m.unit, KELVIN);
        #[cfg(feature = "std")]
        assert_eq!(m.to_string(), "21.5 K");
    }

    #[test]
    fn test_temp_converter() {
        let tk: Temperature = 17.5 * KELVIN;
        assert_eq!(TEMPERATURE_CONVERTER.convert(&tk, KELVIN), Some(tk));
        let tc = TEMPERATURE_CONVERTER.convert(&tk, DEGREE_CELSIUS).unwrap();
        assert_eq!(tc.unit(), DEGREE_CELSIUS);
        assert_almost_eq!(tc.value(), -255.65_f64);
        let tk2 = TEMPERATURE_CONVERTER.convert(&tc, KELVIN).unwrap();
        assert_almost_eq!(tk2.value(), tk.value());
        let tf = TEMPERATURE_CONVERTER
            .convert(&tk, DEGREE_FAHRENHEIT)
            .unwrap();
        assert_eq!(tf.unit(), DEGREE_FAHRENHEIT);
        assert_almost_eq!(tf.value(), -428.17_f64);
        let tk2 = TEMPERATURE_CONVERTER.convert(&tf, KELVIN).unwrap();
        assert_almost_eq!(tk2.value(), tk.value());
        let tc: Temperature = 34.7 * DEGREE_CELSIUS;
        let tf = TEMPERATURE_CONVERTER
            .convert(&tc, DEGREE_FAHRENHEIT)
            .unwrap();
        assert_almost_eq!(tf.value(), 94.46_f64);
        let tc2 = TEMPERATURE_CONVERTER.convert(&tf, DEGREE_CELSIUS).unwrap();
        assert_almost_eq!(tc2.value, tc.value);
    }
}
