# Example

```rust
use quantities::prelude::*; // This dependency can't be fulfilled here!
#[quantity]
#[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
#[unit(Milligram, "mg", MILLI, 0.000001, "0.001·g")]
#[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
#[unit(Ounce, "oz", 0.028349523125, "0.0625·lb")]
#[unit(Pound, "lb", 0.45359237, "0.45359237·kg")]
#[unit(Tonne, "t", MEGA, 1000, "1000·kg")]
/// The quantity of matter in a physical body.
struct Mass {}
```

This results in the following code:

```rust
#[doc = " The quantity of matter in a physical body."]
#[derive(Copy, Clone, Debug)]
pub struct Mass {
    amount: AmountT,
    unit: MassUnit,
}
impl Quantity for Mass {
    type UnitType = MassUnit;
    #[inline(always)]
    fn new(amount: AmountT, unit: Self::UnitType) -> Self {
        Self { amount, unit }
    }
    #[inline(always)]
    fn amount(&self) -> AmountT {
        self.amount
    }
    #[inline(always)]
    fn unit(&self) -> Self::UnitType {
        self.unit
    }
}
#[doc = "Unit of quantity `Mass`."]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MassUnit {
    #[doc = "0.001·g"]
    Milligram,
    #[doc = "0.001·kg"]
    Gram,
    #[doc = "0.0625·lb"]
    Ounce,
    #[doc = "0.45359237·kg"]
    Pound,
    #[doc = "Reference unit of quantity `Mass`"]
    Kilogram,
    #[doc = "1000·kg"]
    Tonne,
}
impl MassUnit {
    const VARIANTS: [MassUnit; 6usize] = [
        MassUnit::Milligram,
        MassUnit::Gram,
        MassUnit::Ounce,
        MassUnit::Pound,
        MassUnit::Kilogram,
        MassUnit::Tonne,
    ];
}
impl Unit for MassUnit {
    type QuantityType = Mass;
    fn iter<'a>() -> core::slice::Iter<'a, Self> {
        Self::VARIANTS.iter()
    }
    fn name(&self) -> &'static str {
        match self {
            MassUnit::Milligram => "Milligram",
            MassUnit::Gram => "Gram",
            MassUnit::Ounce => "Ounce",
            MassUnit::Pound => "Pound",
            MassUnit::Kilogram => "Kilogram",
            MassUnit::Tonne => "Tonne",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            MassUnit::Milligram => "mg",
            MassUnit::Gram => "g",
            MassUnit::Ounce => "oz",
            MassUnit::Pound => "lb",
            MassUnit::Kilogram => "kg",
            MassUnit::Tonne => "t",
        }
    }
    fn si_prefix(&self) -> Option<SIPrefix> {
        match self {
            MassUnit::Milligram => Some(SIPrefix::MILLI),
            MassUnit::Gram => Some(SIPrefix::NONE),
            MassUnit::Kilogram => Some(SIPrefix::KILO),
            MassUnit::Tonne => Some(SIPrefix::MEGA),
            _ => None,
        }
    }
}
impl LinearScaledUnit for MassUnit {
    const REF_UNIT: Self = MassUnit::Kilogram;
    fn scale(&self) -> AmountT {
        match self {
            MassUnit::Milligram => 0.000001 as f64,
            MassUnit::Gram => 0.001 as f64,
            MassUnit::Ounce => 0.028349523125 as f64,
            MassUnit::Pound => 0.45359237 as f64,
            MassUnit::Kilogram => 1.0 as f64,
            MassUnit::Tonne => 1000 as f64,
        }
    }
}
impl HasRefUnit for Mass {
    const REF_UNIT: MassUnit = MassUnit::Kilogram;
}
impl Eq for Mass {}
impl PartialEq<Self> for Mass {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        <Self as HasRefUnit>::eq(self, other)
    }
}
impl PartialOrd for Mass {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <Self as HasRefUnit>::partial_cmp(self, other)
    }
}
impl Add<Self> for Mass {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        <Self as HasRefUnit>::add(self, rhs)
    }
}
impl Sub<Self> for Mass {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        <Self as HasRefUnit>::sub(self, rhs)
    }
}
impl Div<Self> for Mass {
    type Output = AmountT;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        <Self as HasRefUnit>::div(self, rhs)
    }
}
#[doc = "0.001·g"]
pub const MILLIGRAM: MassUnit = MassUnit::Milligram;
#[doc = "0.001·kg"]
pub const GRAM: MassUnit = MassUnit::Gram;
#[doc = "0.0625·lb"]
pub const OUNCE: MassUnit = MassUnit::Ounce;
#[doc = "0.45359237·kg"]
pub const POUND: MassUnit = MassUnit::Pound;
#[doc = "Reference unit of quantity `Mass`"]
pub const KILOGRAM: MassUnit = MassUnit::Kilogram;
#[doc = "1000·kg"]
pub const TONNE: MassUnit = MassUnit::Tonne;
impl Mul<MassUnit> for AmountT {
    type Output = Mass;
    #[inline(always)]
    fn mul(self, rhs: MassUnit) -> Self::Output {
        Mass::new(self, rhs)
    }
}
impl Mul<AmountT> for MassUnit {
    type Output = Mass;
    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Mass::new(rhs, self)
    }
}
impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Quantity>::fmt(self, f)
    }
}
impl Mul<Mass> for AmountT {
    type Output = Mass;
    #[inline(always)]
    fn mul(self, rhs: Mass) -> Self::Output {
        Self::Output::new(self * rhs.amount(), rhs.unit())
    }
}
impl Mul<AmountT> for Mass {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Self::Output::new(self.amount() * rhs, self.unit())
    }
}
impl Div<AmountT> for Mass {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: AmountT) -> Self::Output {
        Self::Output::new(self.amount() / rhs, self.unit())
    }
}
```
