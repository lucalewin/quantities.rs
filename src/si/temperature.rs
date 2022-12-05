use crate::{prelude::*, ConversionTable};

#[quantity]
#[unit(Kelvin, "K", "K")]
#[unit(Degree_Celsius, "°C", "°C")]
pub struct Temperature;

/// Temperature conversion table
/// 
/// taken from https://github.com/mamrhein/quantities.rs/blob/main/src/temperature.rs#L39-L59
/// edited by Luca Lewin on 02/12/2022
pub const TEMPERATURE_CONVERTER: ConversionTable<Temperature, 2> =
    ConversionTable {
        mappings: [
            (KELVIN, DEGREE_CELSIUS, 1.0, -273.15),
            (DEGREE_CELSIUS, KELVIN, 1.0, 273.15),
        ],
    };
