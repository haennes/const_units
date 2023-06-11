use self::quantity::QuantityDataTraits;
use crate::generated::quantity_from_name;
use const_units_macros::{AddingMul, Div, Mul, Neg, SubbingDiv};
use num_traits::{Num, NumAssign, NumAssignRef};
use parse_display::{Display, FromStr};
use quote::ToTokens;
use self_rust_tokenize::SelfRustTokenize;
use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{self, Div, Mul, Neg},
};

pub(crate) mod quantity;

pub(crate) enum Operation {
    Mul(QName, QName),
    Div(QName, QName),
    Neg(QName),
}

#[derive(
    PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, Neg, AddingMul, SubbingDiv, FromStr,
)]
pub struct SiExtended {
    a: i16,
}

#[derive(PartialEq, Eq, Display, SelfRustTokenize, FromStr)]
pub enum QName {
    Velocity,
}

// example: pub type Velocity = Quantity<"velocity", _>

// pub type Velocity<DT> =
//     Unit<DT, { quantity::Quantity::from_name(QName::Velocity, System::SiExtended) }>;

pub struct Unit<
    StorageDt: QuantityDataTraits,
    const QUANTITY: quantity::Quantity,
    const INITIALIZED: bool = false,
> {
    value: StorageDt,
}

impl<StorageDt: QuantityDataTraits, const QUANTITY: quantity::Quantity>
    Unit<StorageDt, QUANTITY, false>
where
    StorageDt: QuantityDataTraits,
{
    pub const fn new() -> Self {
        Self {
            value: <StorageDt as quantity::One>::one,
        }
    }
}

impl<
        StorageDt: QuantityDataTraits,
        const QUANTITY: quantity::Quantity,
        const INITIALIZED: bool,
    > Unit<StorageDt, QUANTITY, INITIALIZED>
where
    StorageDt: QuantityDataTraits,
{
    /// declared as unsafe as you are leaving the dimension-checking realm
    pub const unsafe fn raw_value(&self) -> StorageDt {
        self.value
    }
}
