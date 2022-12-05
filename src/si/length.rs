use crate::prelude::*;

#[quantity]
#[ref_unit(Meter,  "m",  NONE,  "Reference unit of quantity `Length`")]
#[unit(Nanometer,  "nm", NANO,  0.000000001, "0.000000001·m")]
#[unit(Micrometer, "µm", MICRO, 0.000001,    "0.000001·m")]
#[unit(Millimeter, "mm", MILLI, 0.001, "0.001·m")]
#[unit(Centimeter, "cm", CENTI, 0.01,  "0.01·m")]
#[unit(Decimeter,  "dm", DECI,  0.1,   "0.1·m")]
#[unit(Kilometer,  "km", KILO,  1000,  "1000·m")]
pub struct Length;