#![allow(incomplete_features)]
#![feature(
    adt_const_params,
    const_trait_impl,
    inline_const_pat,
    generic_const_exprs,
    const_option
)]

pub mod global_types;
pub use global_types::*;

pub use const_units_global_types::Factor;
pub use const_units_global_types::RatioConst;
pub use const_units_global_types::F64;
pub use const_units_uuse::uuse;
pub use generated::PName;
pub use generated::QName;
pub use generated::System;
pub use generated::UName;
pub use global_types::prefix::Prefix;
pub use global_types::quantity::Quantity;

pub mod generated {
    #![allow(unused_variables, non_upper_case_globals)]

    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
pub use generated::*;

macro_rules! impl_one {
    ($($type:ty) *) => {
        $(
            impl One for $type {
                const ONE: Self = 1;
            }
        )*
    };
    ($($type:ty) *, $one:expr) => {
        $(
            impl One for $type {
                const ONE: Self = $one;
            }
        )*
    }
}

impl_one!(f32 f64, 1.0);
impl_one!(u8 u16 u32 u64 u128);
impl_one!(i8 i16 i32 i64 i128);

use global_types::quantity::One;
//HACK This is a HACK see below
impl From<Option<PName>> for PName {
    fn from(value: Option<PName>) -> Self {
        match value {
            Some(panme) => panme,
            None => PName::Unknown,
        }
    }
}
