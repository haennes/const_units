use const_ops::Mul;

use crate::generated::QName;

use self::prefix::Prefix;
use self::quantity::QuantityDataTraits;
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
    const QUANTITY: quantity::Quantity,
    const INITIALIZED: bool = false,
> {
    value: StorageDt,
}

impl<
        StorageDt: QuantityDataTraits,
        const NAME: UName,
        const PREFIX: Prefix,
        const QUANTITY: quantity::Quantity,
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
        const QUANTITY: quantity::Quantity,
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
///FIXME convert this to a const impl
impl<
        StorageDt: QuantityDataTraits,
        const NAME_1: UName,
        const QUANTITY_1: quantity::Quantity,
        const PREFIX_1: Prefix,
        const INITIALIZED_1: bool,
        const NAME_2: UName,
        const QUANTITY_2: quantity::Quantity,
        const PREFIX_2: Prefix,
        const INITIALIZED_2: bool,
    > Mul<Unit<StorageDt, NAME_2, PREFIX_2, QUANTITY_2, INITIALIZED_2>>
    for Unit<StorageDt, NAME_1, PREFIX_1, QUANTITY_1, INITIALIZED_1>
where
    Unit<StorageDt, NAME_1, { PREFIX_1.mul(PREFIX_2) }, { QUANTITY_1.mul(QUANTITY_2) }, true>:,
{
    type Output =
        Unit<StorageDt, NAME_1, { PREFIX_1.mul(PREFIX_2) }, { QUANTITY_1.mul(QUANTITY_2) }, true>;

    fn mul(
        self,
        rhs: Unit<StorageDt, NAME_2, PREFIX_2, QUANTITY_2, INITIALIZED_2>,
    ) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

// ///FIXME convert this to a const impl
// impl<
//         StorageDt: QuantityDataTraits,
//         const NAME: UName,
//         const QUANTITY: quantity::Quantity,
//         const PREFIX: Prefix,
//         const INITIALIZED: bool,
//     > Mul for Unit<StorageDt, NAME, PREFIX, QUANTITY, INITIALIZED>
// {
//     type Output = Unit<StorageDt, NAME, PREFIX, QUANTITY, true>;

//     fn mul(self, rhs: Self) -> Self::Output {
//         Self::Output {
//             value: self.value * rhs.value,
//         }
//     }
// }
