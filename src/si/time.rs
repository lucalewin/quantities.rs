use crate::prelude::*;

#[quantity]
#[ref_unit(Second, "s", NONE, "Reference unit of quantity `Time`")]
#[unit(Nanosecond, "ns", NANO, 0.000000001, "0.000000001·s")]
#[unit(Microsecond, "µs", MICRO, 0.000001, "0.000001·s")]
#[unit(Millisecond, "ms", MILLI, 0.001, "0.001·s")]
#[unit(Minute, "min", 60, "60·s")]
#[unit(Hour, "h", 3600, "60·min")]
#[unit(Day, "d", 86400, "24·h")]
pub struct Time;