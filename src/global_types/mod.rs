use crate::generated::units::UName;

use self::prefix::Prefix;
use self::quantity::QuantityDataTraits;
use const_units_macros::{AddingMul, Neg, SubbingDiv};
use parse_display::{Display, FromStr};
use self_rust_tokenize::SelfRustTokenize;
use std::marker::ConstParamTy;
use std::ops::{Div, Mul, Neg};

pub(crate) mod factor;
pub(crate) mod prefix;
pub(crate) mod quantity;

pub(crate) enum Operation {
    Mul(QName, QName),
    Div(QName, QName),
    Neg(QName),
}

#[derive(
    PartialEq,
    Eq,
    Display,
    SelfRustTokenize,
    Clone,
    Copy,
    Neg,
    AddingMul,
    SubbingDiv,
    FromStr,
    ConstParamTy,
)]
pub struct SiExtended {
    pub(crate) a: i16,
}

#[derive(PartialEq, Eq, Display, SelfRustTokenize, FromStr, ConstParamTy)]
pub enum QName {
    Time,
    Velocity,
    Length,
}

// example: pub type Velocity = Quantity<"velocity", _>

// pub type Velocity<DT> =
//     Unit<DT, { quantity::Quantity::from_name(QName::Velocity, System::SiExtended) }>;

pub struct Unit<
    StorageDt: QuantityDataTraits,
    const NAME: UName,
    const QUANTITY: quantity::Quantity,
    const PREFIX: Prefix,
    const INITIALIZED: bool = false,
> {
    value: StorageDt,
}

impl<
        StorageDt: QuantityDataTraits,
        const NAME: UName,
        const QUANTITY: quantity::Quantity,
        const PREFIX: Prefix,
    > Unit<StorageDt, NAME, QUANTITY, PREFIX, false>
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
    > Unit<StorageDt, NAME, QUANTITY, PREFIX, false>
where
    StorageDt: QuantityDataTraits,
{
    /// only defined on initialized Units to avoid mistakes
    /// declared as unsafe as you are leaving the dimension-checking realm
    pub const unsafe fn raw_value(&self) -> StorageDt {
        self.value
    }
}
