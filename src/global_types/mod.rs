use core::fmt::Display;
use core::ops::{Add, Div, Mul, Sub};

use crate::generated::QName;

use self::prefix::Prefix;
use self::quantity::{Quantity, QuantityDataTraits};
pub mod prefix;
use crate::generated::UName;
pub use prefix::*;

pub(crate) mod quantity;

#[derive(Clone, Copy)]
pub enum Operation {
    Mul(QName, QName),
    Div(QName, QName),
    Neg(QName),
}

pub struct Unit<
    StorageDt: QuantityDataTraits,
    const NAME: UName,
    const PREFIX: Prefix,
    const QUANTITY: Quantity,
    const INITIALIZED: bool = false,
> {
    value: StorageDt,
}

impl<
        StorageDt: QuantityDataTraits,
        const NAME: UName,
        const PREFIX: Prefix,
        const QUANTITY: Quantity,
    > Unit<StorageDt, NAME, PREFIX, QUANTITY, false>
where
    StorageDt: QuantityDataTraits,
{
    pub const fn new() -> Self {
        Self {
            value: <StorageDt as quantity::One>::ONE,
        }
    }
}

impl<
        StorageDt: QuantityDataTraits,
        const NAME: UName,
        const QUANTITY: Quantity,
        const PREFIX: Prefix,
    > Unit<StorageDt, NAME, PREFIX, QUANTITY, false>
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
        const NAME: UName,
        const QUANTITY: Quantity,
        const PREFIX: Prefix,
        const INITIALIZED: bool,
    > Unit<StorageDt, NAME, PREFIX, QUANTITY, INITIALIZED>
where
    StorageDt: QuantityDataTraits,
{
    pub const fn quantity() -> Quantity {
        QUANTITY
    }
    pub const fn prefix() -> Prefix {
        PREFIX
    }
    pub const fn initialized() -> bool {
        INITIALIZED
    }
    pub const fn name() -> UName {
        NAME
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const NAME_1: UName,
        const QUANTITY_1: Quantity,
        const PREFIX_1: Prefix,
        const INITIALIZED_1: bool,
        const NAME_2: UName,
        const QUANTITY_2: Quantity,
        const PREFIX_2: Prefix,
        const INITIALIZED_2: bool,
    > Mul<Unit<StorageDt, NAME_2, PREFIX_2, QUANTITY_2, INITIALIZED_2>>
    for Unit<StorageDt, NAME_1, PREFIX_1, QUANTITY_1, INITIALIZED_1>
where
    Unit<
        StorageDt,
        NAME_1,
        { const_ops::Mul::mul(PREFIX_1, PREFIX_2) },
        { const_ops::Mul::mul(QUANTITY_1, QUANTITY_2) },
        true,
    >:,
{
    type Output = Unit<
        StorageDt,
        NAME_1,
        { const_ops::Mul::mul(PREFIX_1, PREFIX_2) },
        { const_ops::Mul::mul(QUANTITY_1, QUANTITY_2) },
        true,
    >;

    fn mul(
        self,
        rhs: Unit<StorageDt, NAME_2, PREFIX_2, QUANTITY_2, INITIALIZED_2>,
    ) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const NAME: UName,
        const PREFIX: Prefix,
        const QUANTITY: Quantity,
        const INITIALIZED: bool,
    > Mul<StorageDt> for Unit<StorageDt, NAME, { PREFIX }, { QUANTITY }, INITIALIZED>
{
    type Output = Unit<StorageDt, NAME, { PREFIX }, { QUANTITY }, true>;

    fn mul(self, rhs: StorageDt) -> Self::Output {
        Self::Output {
            value: self.value * rhs,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const NAME_1: UName,
        const QUANTITY_1: Quantity,
        const PREFIX_1: Prefix,
        const INITIALIZED_1: bool,
        const NAME_2: UName,
        const QUANTITY_2: Quantity,
        const PREFIX_2: Prefix,
        const INITIALIZED_2: bool,
    > Div<Unit<StorageDt, NAME_2, PREFIX_2, QUANTITY_2, INITIALIZED_2>>
    for Unit<StorageDt, NAME_1, PREFIX_1, QUANTITY_1, INITIALIZED_1>
where
    Unit<
        StorageDt,
        NAME_1,
        { const_ops::Div::div(PREFIX_1, PREFIX_2) },
        { const_ops::Div::div(QUANTITY_1, QUANTITY_2) },
        true,
    >:,
{
    type Output = Unit<
        StorageDt,
        NAME_1,
        { const_ops::Div::div(PREFIX_1, PREFIX_2) },
        { const_ops::Div::div(QUANTITY_1, QUANTITY_2) },
        true,
    >;

    fn div(
        self,
        rhs: Unit<StorageDt, NAME_2, PREFIX_2, QUANTITY_2, INITIALIZED_2>,
    ) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const NAME_1: UName,
        const QUANTITY_1: Quantity,
        const PREFIX_1: Prefix,
        const INITIALIZED_1: bool,
    > Add for Unit<StorageDt, NAME_1, PREFIX_1, QUANTITY_1, INITIALIZED_1>
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
        const NAME_1: UName,
        const QUANTITY_1: Quantity,
        const PREFIX_1: Prefix,
        const INITIALIZED_1: bool,
    > Sub for Unit<StorageDt, NAME_1, PREFIX_1, QUANTITY_1, INITIALIZED_1>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<
        StorageDT: QuantityDataTraits,
        const NAME: UName,
        const PREFIX: prefix::Prefix,
        const QUANTITY: Quantity,
    > Display for Unit<StorageDT, NAME, PREFIX, QUANTITY, true>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            //TODO Determine what to change when alternate is selected
            write!(f, "{} {}", self.value, NAME)
        } else {
            write!(f, "{} {}", self.value, NAME)
        }
    }
}
impl<
        StorageDT: QuantityDataTraits,
        const NAME: UName,
        const PREFIX: prefix::Prefix,
        const QUANTITY: Quantity,
    > Display for Unit<StorageDT, NAME, PREFIX, QUANTITY, false>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{} (uninit)", NAME)
        } else {
            write!(f, "{}", NAME)
        }
    }
}
