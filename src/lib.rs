#![allow(incomplete_features)]
#![feature(
    adt_const_params,
    const_trait_impl,
    inline_const_pat,
    generic_const_exprs
)]

pub mod global_types;
pub use global_types::*;

pub use const_units_global_types::Factor;
pub use const_units_global_types::RatioConst;
pub use const_units_global_types::F64;
pub use generated::PName;
pub use generated::QName;
pub use generated::System;
pub use global_types::prefix::Prefix;
pub use global_types::quantity::Quantity;

//mod generated;
mod generated {
    #![allow(unused_variables, non_upper_case_globals)]
    use crate::Operation;

    pub fn get_name_from_dimensions_and_op(dim: SystemDim, op: Operation) -> QName {
        todo!()
    }

    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

impl One for u16 {
    const ONE: Self = 1;
}

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
