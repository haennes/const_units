use core::marker::ConstParamTy;
use parse_display::Display;
use self_rust_tokenize::SelfRustTokenize;

use crate::global_types::factor::Factor;

#[derive(Debug, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize, Display)]
pub enum PName {
    Unknown, //HACK see below
    None,
    Centi,
}

impl ::core::str::FromStr for PName {
    type Err = ::parse_display::ParseError;
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "Unknown" => {
                return ::core::result::Result::Ok(Self::Unknown);
            }
            "None" => {
                return ::core::result::Result::Ok(Self::None);
            }
            "Centi" => {
                return ::core::result::Result::Ok(Self::Centi);
            }
            _ => {}
        }
        ::core::result::Result::Err(::parse_display::ParseError::new())
    }
}

//HACK Should be "for (Option<PName>, Option<PName>)"
/// (name, alias)
impl From<Factor> for (PName, PName) {
    fn from(value: Factor) -> Self {
        match value {
            Factor::Ratio(ratio) => todo!(),
            Factor::Float(float) => todo!(),
        }
    }
}
