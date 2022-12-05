use self::{mass::{Mass, KILOGRAM}, velocity::{METER_PER_SECOND, Velocity}};

pub mod length;
pub mod mass;
pub mod temperature;
pub mod time;
pub mod velocity;

const MY_WEIGHT: Mass = 47.0 * KILOGRAM;
const SPEED_OF_LIGHT: Velocity = 299792458.0 * METER_PER_SECOND;

#[cfg(test)]
mod tests {
    use crate::Quantity;

    use super::{MY_WEIGHT, mass::KILOGRAM};

    #[test]
    fn const_quantities() {
        let _t = MY_WEIGHT.value();
        assert_eq!(MY_WEIGHT, 47.0 * KILOGRAM);
    }
}
