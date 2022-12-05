use crate::prelude::*;

use super::{length::Length, time::Time};


#[quantity(Length / Time)]
#[ref_unit(Meter_per_Second, "m/s", NONE, "Reference unit of quantity `Velocity`")]
#[unit(Kilometer_per_Hour, "km/h", 0.2777777777777778, "km/h")]
pub struct Velocity;