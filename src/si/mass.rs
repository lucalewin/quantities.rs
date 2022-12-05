use crate::prelude::*;

#[quantity]
#[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
#[unit(Milligram, "mg", MILLI, 0.000001, "0.001·g")]
#[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
pub struct Mass;