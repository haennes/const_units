use core::fmt::Display;

use super::quantity::{Quantity, QuantityDataTraits};
use crate::printing_style::PrintingStyle;
use crate::printing_style::DEFAULT_PRINTINGSTYLE;
use crate::quantity::One;
use crate::PName;
use crate::Prefix;
use crate::UName;
use const_ops::{Add, Div, Mul, Neg, Sub};
pub struct Unit<
    StorageDt: QuantityDataTraits,
    const UNIT: UName,
    const QUANTITY: Quantity,
    const PREFIX: Prefix = { Prefix::from(PName::None) },
    const INITIALIZED: bool = false,
    const PRINTINGSTYLE: PrintingStyle = { DEFAULT_PRINTINGSTYLE },
> {
    value: StorageDt,
}

impl<
        StorageDt: QuantityDataTraits,
        const UNIT: UName,
        const PREFIX: Prefix,
        const QUANTITY: Quantity,
    > Unit<StorageDt, UNIT, QUANTITY, PREFIX, false>
where
    StorageDt: QuantityDataTraits,
{
    pub const fn new() -> Self {
        Self {
            value: <StorageDt as One>::ONE,
        }
    }
}

impl<
        StorageDt: QuantityDataTraits,
        const UNIT: UName,
        const QUANTITY: Quantity,
        const PREFIX: Prefix,
    > Unit<StorageDt, UNIT, QUANTITY, PREFIX, true>
where
    StorageDt: QuantityDataTraits,
{
    /// only defined on initialized Units to avoid mistakes
    /// you are leaving the dimension checking realm
    pub const fn raw_value(&self) -> StorageDt {
        self.value
    }
}

impl<
        StorageDt: QuantityDataTraits,
        const UNIT: UName,
        const QUANTITY: Quantity,
        const PREFIX: Prefix,
        const INITIALIZED: bool,
    > Unit<StorageDt, UNIT, QUANTITY, PREFIX, INITIALIZED>
where
    StorageDt: QuantityDataTraits,
{
    pub const fn quantity(&self) -> Quantity {
        QUANTITY
    }
    pub const fn prefix(&self) -> Prefix {
        PREFIX
    }
    pub const fn initialized(&self) -> bool {
        INITIALIZED
    }
    pub const fn name(&self) -> UName {
        UNIT
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const UNIT_1: UName,
        const QUANTITY_1: Quantity,
        const PREFIX_1: Prefix,
        const INITIALIZED_1: bool,
        const UNIT_2: UName,
        const QUANTITY_2: Quantity,
        const PREFIX_2: Prefix,
        const INITIALIZED_2: bool,
    > Mul<Unit<StorageDt, UNIT_2, QUANTITY_2, PREFIX_2, INITIALIZED_2>>
    for Unit<StorageDt, UNIT_1, QUANTITY_1, PREFIX_1, INITIALIZED_1>
where
    Unit<
        StorageDt,
        { const_ops::Mul::mul(UNIT_1, UNIT_2) },
        { const_ops::Mul::mul(QUANTITY_1, QUANTITY_2) },
        { const_ops::Mul::mul(PREFIX_1, PREFIX_2) },
        true,
    >:,
{
    type Output = Unit<
        StorageDt,
        { const_ops::Mul::mul(UNIT_1, UNIT_2) },
        { const_ops::Mul::mul(QUANTITY_1, QUANTITY_2) },
        { const_ops::Mul::mul(PREFIX_1, PREFIX_2) },
        true,
    >;

    fn mul(
        self,
        rhs: Unit<StorageDt, UNIT_2, QUANTITY_2, PREFIX_2, INITIALIZED_2>,
    ) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const UNIT: UName,
        const PREFIX: Prefix,
        const QUANTITY: Quantity,
        const INITIALIZED: bool,
    > Mul<StorageDt> for Unit<StorageDt, UNIT, QUANTITY, PREFIX, INITIALIZED>
{
    type Output = Unit<StorageDt, UNIT, QUANTITY, PREFIX, true>;

    fn mul(self, rhs: StorageDt) -> Self::Output {
        Self::Output {
            value: self.value * rhs,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const UNIT_1: UName,
        const QUANTITY_1: Quantity,
        const PREFIX_1: Prefix,
        const INITIALIZED_1: bool,
        const UNIT_2: UName,
        const QUANTITY_2: Quantity,
        const PREFIX_2: Prefix,
        const INITIALIZED_2: bool,
    > Div<Unit<StorageDt, UNIT_2, QUANTITY_2, PREFIX_2, INITIALIZED_2>>
    for Unit<StorageDt, UNIT_1, QUANTITY_1, PREFIX_1, INITIALIZED_1>
where
    Unit<
        StorageDt,
        { const_ops::Div::div(UNIT_1, UNIT_2) },
        { const_ops::Div::div(QUANTITY_1, QUANTITY_2) },
        { const_ops::Div::div(PREFIX_1, PREFIX_2) },
        true,
    >:,
{
    type Output = Unit<
        StorageDt,
        { const_ops::Div::div(UNIT_1, UNIT_2) },
        { const_ops::Div::div(QUANTITY_1, QUANTITY_2) },
        { const_ops::Div::div(PREFIX_1, PREFIX_2) },
        true,
    >;

    fn div(
        self,
        rhs: Unit<StorageDt, UNIT_2, QUANTITY_2, PREFIX_2, INITIALIZED_2>,
    ) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const UNIT: UName,
        const QUANTITY: Quantity,
        const PREFIX: Prefix,
        const INITIALIZED: bool,
    > Div<StorageDt> for Unit<StorageDt, UNIT, QUANTITY, PREFIX, INITIALIZED>
where
    Unit<StorageDt, { UNIT.neg() }, { QUANTITY.neg() }, { PREFIX.neg() }, true>:,
{
    type Output = Unit<StorageDt, { UNIT.neg() }, { QUANTITY.neg() }, { PREFIX.neg() }, true>;

    fn div(self, rhs: StorageDt) -> Self::Output {
        Self::Output {
            value: self.value / rhs,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const UNIT: UName,
        const QUANTITY: Quantity,
        const PREFIX: Prefix,
        const INITIALIZED: bool,
    > Add for Unit<StorageDt, UNIT, QUANTITY, PREFIX, INITIALIZED>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const UNIT: UName,
        const QUANTITY: Quantity,
        const PREFIX: Prefix,
        const INITIALIZED: bool,
    > Sub for Unit<StorageDt, UNIT, QUANTITY, PREFIX, INITIALIZED>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
        }
    }
}
//TODO custom formatter!!!
// impl<
//         StorageDT: QuantityDataTraits,
//         const UNIT: UName,
//         const PREFIX: Prefix,
//         const QUANTITY: Quantity,
//     > Display for Unit<StorageDT, UNIT, QUANTITY, PREFIX, true>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if f.alternate() {
//             write!(f, "{} {}", self.value, UNIT)
//         } else {
//             write!(f, "{} {}", self.value, UNIT)
//         }
//     }
// }
// impl<
//         StorageDT: QuantityDataTraits,
//         const UNIT: UName,
//         const PREFIX: Prefix,
//         const QUANTITY: Quantity,
//     > Display for Unit<StorageDT, UNIT, QUANTITY, PREFIX, false>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if f.alternate() {
//             write!(f, "{} (uninit)", UNIT)
//         } else {
//             write!(f, "{}", UNIT)
//         }
//     }
// }
