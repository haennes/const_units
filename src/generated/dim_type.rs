// DONE
use const_units_macros::{Div, Mul, Neg};
use core::marker::ConstParamTy;
use parse_display::{Display, FromStr};
use self_rust_tokenize::SelfRustTokenize;
use std::ops::{Div, Mul, Neg};

use crate::global_types::SiExtended;

#[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, FromStr)]
pub enum System {
    SiExtended,
}

#[derive(
    PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, Neg, Mul, Div, FromStr, ConstParamTy,
)]
#[display("{0}")]
pub enum SystemDim {
    SiExtended(SiExtended),
}
