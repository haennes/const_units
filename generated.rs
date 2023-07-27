#![allow(unused_imports)]

///DONE
pub(crate) mod dim_type;
///DONE
pub mod generic_units;
///DONE
pub(crate) mod get_name_from_dimensions_and_op;
///DONE
pub(crate) mod prefix;
///DONE
pub(crate) mod quantity_from_name;
///TODO
pub(crate) mod units;

pub(crate) use crate::generated::dim_type::SystemDim;
pub(crate) use crate::global_types::{Operation, QName};
pub(crate) use const_units_macros::{Div, Mul, Neg};
pub(crate) use core::marker::ConstParamTy;
pub(crate) use parse_display::{Display, FromStr};
pub(crate) use prefix::*;
pub(crate) use std::ops::{Div, Mul, Neg};
#[derive(Debug, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize, Display)]
enum PName {
    Unkonw,
    Mebi,
    Yobi,
    Exbi,
    Gibi,
    Zebi,
    Tebi,
    Kibi,
    Pebi,
    Kilo,
    Giga,
    Quetta,
    Ronto,
    Deci,
    Hecto,
    Ronna,
    Milli,
    Atto,
    Deca,
    Zepto,
    Quecto,
    Mega,
    Micro,
    Tera,
    Centi,
    Nano,
    Exa,
    Zetta,
    Femto,
    Pico,
    Peta,
    Yocto,
    Yotta,
}
impl ::core::str::FromStr for PName {
    type Err = ::parse_display::ParseError;
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "mebi" => {
                return ::core::result::Result::Ok(Self::Mebi);
            }
            "yobi" => {
                return ::core::result::Result::Ok(Self::Yobi);
            }
            "exbi" => {
                return ::core::result::Result::Ok(Self::Exbi);
            }
            "gibi" => {
                return ::core::result::Result::Ok(Self::Gibi);
            }
            "zebi" => {
                return ::core::result::Result::Ok(Self::Zebi);
            }
            "tebi" => {
                return ::core::result::Result::Ok(Self::Tebi);
            }
            "kibi" => {
                return ::core::result::Result::Ok(Self::Kibi);
            }
            "pebi" => {
                return ::core::result::Result::Ok(Self::Pebi);
            }
            "kilo" => {
                return ::core::result::Result::Ok(Self::Kilo);
            }
            "k" => {
                return ::core::result::Result::Ok(Self::Kilo);
            }
            "giga" => {
                return ::core::result::Result::Ok(Self::Giga);
            }
            "G" => {
                return ::core::result::Result::Ok(Self::Giga);
            }
            "quetta" => {
                return ::core::result::Result::Ok(Self::Quetta);
            }
            "Q" => {
                return ::core::result::Result::Ok(Self::Quetta);
            }
            "ronto" => {
                return ::core::result::Result::Ok(Self::Ronto);
            }
            "r" => {
                return ::core::result::Result::Ok(Self::Ronto);
            }
            "deci" => {
                return ::core::result::Result::Ok(Self::Deci);
            }
            "d" => {
                return ::core::result::Result::Ok(Self::Deci);
            }
            "hecto" => {
                return ::core::result::Result::Ok(Self::Hecto);
            }
            "h" => {
                return ::core::result::Result::Ok(Self::Hecto);
            }
            "ronna" => {
                return ::core::result::Result::Ok(Self::Ronna);
            }
            "R" => {
                return ::core::result::Result::Ok(Self::Ronna);
            }
            "milli" => {
                return ::core::result::Result::Ok(Self::Milli);
            }
            "m" => {
                return ::core::result::Result::Ok(Self::Milli);
            }
            "atto" => {
                return ::core::result::Result::Ok(Self::Atto);
            }
            "a" => {
                return ::core::result::Result::Ok(Self::Atto);
            }
            "deca" => {
                return ::core::result::Result::Ok(Self::Deca);
            }
            "da" => {
                return ::core::result::Result::Ok(Self::Deca);
            }
            "zepto" => {
                return ::core::result::Result::Ok(Self::Zepto);
            }
            "z" => {
                return ::core::result::Result::Ok(Self::Zepto);
            }
            "quecto" => {
                return ::core::result::Result::Ok(Self::Quecto);
            }
            "q" => {
                return ::core::result::Result::Ok(Self::Quecto);
            }
            "mega" => {
                return ::core::result::Result::Ok(Self::Mega);
            }
            "M" => {
                return ::core::result::Result::Ok(Self::Mega);
            }
            "micro" => {
                return ::core::result::Result::Ok(Self::Micro);
            }
            "μ" => {
                return ::core::result::Result::Ok(Self::Micro);
            }
            "tera" => {
                return ::core::result::Result::Ok(Self::Tera);
            }
            "T" => {
                return ::core::result::Result::Ok(Self::Tera);
            }
            "centi" => {
                return ::core::result::Result::Ok(Self::Centi);
            }
            "c" => {
                return ::core::result::Result::Ok(Self::Centi);
            }
            "nano" => {
                return ::core::result::Result::Ok(Self::Nano);
            }
            "n" => {
                return ::core::result::Result::Ok(Self::Nano);
            }
            "exa" => {
                return ::core::result::Result::Ok(Self::Exa);
            }
            "E" => {
                return ::core::result::Result::Ok(Self::Exa);
            }
            "zetta" => {
                return ::core::result::Result::Ok(Self::Zetta);
            }
            "Z" => {
                return ::core::result::Result::Ok(Self::Zetta);
            }
            "femto" => {
                return ::core::result::Result::Ok(Self::Femto);
            }
            "f" => {
                return ::core::result::Result::Ok(Self::Femto);
            }
            "pico" => {
                return ::core::result::Result::Ok(Self::Pico);
            }
            "p" => {
                return ::core::result::Result::Ok(Self::Pico);
            }
            "peta" => {
                return ::core::result::Result::Ok(Self::Peta);
            }
            "P" => {
                return ::core::result::Result::Ok(Self::Peta);
            }
            "yocto" => {
                return ::core::result::Result::Ok(Self::Yocto);
            }
            "y" => {
                return ::core::result::Result::Ok(Self::Yocto);
            }
            "yotta" => {
                return ::core::result::Result::Ok(Self::Yotta);
            }
            "Y" => {
                return ::core::result::Result::Ok(Self::Yotta);
            }
            _ => {}
        }
        ::core::result::Result::Err(::parse_display::ParseError::new())
    }
}
#[doc = r" (name, alias)"]
impl From<Factor> for (PName, PName) {
    fn from(value: Factor) -> Self {
        match value {
            Factor::Ratio(ratio) => match ratio {
                RatioConst {
                    numerator: 1048576i128,
                    denominator: 1i128,
                } => (PName::mebi, PName::Unknown),
                RatioConst {
                    numerator: 1208925819614629174706176i128,
                    denominator: 1i128,
                } => (PName::yobi, PName::Unknown),
                RatioConst {
                    numerator: 1152921504606846976i128,
                    denominator: 1i128,
                } => (PName::exbi, PName::Unknown),
                RatioConst {
                    numerator: 1073741824i128,
                    denominator: 1i128,
                } => (PName::gibi, PName::Unknown),
                RatioConst {
                    numerator: 1180591620717411303424i128,
                    denominator: 1i128,
                } => (PName::zebi, PName::Unknown),
                RatioConst {
                    numerator: 1099511627776i128,
                    denominator: 1i128,
                } => (PName::tebi, PName::Unknown),
                RatioConst {
                    numerator: 1024i128,
                    denominator: 1i128,
                } => (PName::kibi, PName::Unknown),
                RatioConst {
                    numerator: 1125899906842624i128,
                    denominator: 1i128,
                } => (PName::pebi, PName::Unknown),
                RatioConst {
                    numerator: 1000i128,
                    denominator: 1i128,
                } => (PName::kilo, PName::k),
                RatioConst {
                    numerator: 1000000000i128,
                    denominator: 1i128,
                } => (PName::giga, PName::G),
                RatioConst {
                    numerator: 1000000000000000000000000000000i128,
                    denominator: 1i128,
                } => (PName::quetta, PName::Q),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000000000000i128,
                } => (PName::ronto, PName::r),
                RatioConst {
                    numerator: 1i128,
                    denominator: 10i128,
                } => (PName::deci, PName::d),
                RatioConst {
                    numerator: 100i128,
                    denominator: 1i128,
                } => (PName::hecto, PName::h),
                RatioConst {
                    numerator: 1000000000000000000000000000i128,
                    denominator: 1i128,
                } => (PName::ronna, PName::R),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000i128,
                } => (PName::milli, PName::m),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000i128,
                } => (PName::atto, PName::a),
                RatioConst {
                    numerator: 10i128,
                    denominator: 1i128,
                } => (PName::deca, PName::da),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000000i128,
                } => (PName::zepto, PName::z),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000000000000000i128,
                } => (PName::quecto, PName::q),
                RatioConst {
                    numerator: 1000000i128,
                    denominator: 1i128,
                } => (PName::mega, PName::M),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000i128,
                } => (PName::micro, PName::μ),
                RatioConst {
                    numerator: 1000000000000i128,
                    denominator: 1i128,
                } => (PName::tera, PName::T),
                RatioConst {
                    numerator: 1i128,
                    denominator: 100i128,
                } => (PName::centi, PName::c),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000i128,
                } => (PName::nano, PName::n),
                RatioConst {
                    numerator: 1000000000000000000i128,
                    denominator: 1i128,
                } => (PName::exa, PName::E),
                RatioConst {
                    numerator: 1000000000000000000000i128,
                    denominator: 1i128,
                } => (PName::zetta, PName::Z),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000i128,
                } => (PName::femto, PName::f),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000i128,
                } => (PName::pico, PName::p),
                RatioConst {
                    numerator: 1000000000000000i128,
                    denominator: 1i128,
                } => (PName::peta, PName::P),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000000000i128,
                } => (PName::yocto, PName::y),
                RatioConst {
                    numerator: 1000000000000000000000000i128,
                    denominator: 1i128,
                } => (PName::yotta, PName::Y),
                _ => PName::Unknown,
            },
            Factor::Float(float) => match float {
                _ => PName::Unknown,
            },
        }
    }
}
use super::*;
#[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, FromStr, ConstParamTy)]
pub enum System {
    SiExtended,
}
#[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, Neg, Mul, Div, ConstParamTy)]
#[display("{}")]
pub enum SystemDim {
    SiExtended(SiExtended),
}
pub use SiExtended::SiExtended;
mod SiExtended {
    #[derive(Default)]
    pub struct SiExtended {
        #[doc = "luminous intensity"]
        J: i8,
        #[doc = "time"]
        T: i8,
        #[doc = "Information"]
        INFO: i8,
        #[doc = "thermodynamic temprature"]
        Θ: i8,
        #[doc = "Angle"]
        A: i8,
        #[doc = "temperature interval"]
        ΔΘ: i8,
        #[doc = "electrical current"]
        I: i8,
        #[doc = "mass"]
        M: i8,
        #[doc = "length"]
        L: i8,
        #[doc = "amount of substance"]
        N: i8,
    }
    pub const NONE: SiExtended = SiExtended {
        J: 0,
        T: 0,
        INFO: 0,
        Θ: 0,
        A: 0,
        ΔΘ: 0,
        I: 0,
        M: 0,
        L: 0,
        N: 0,
    };
}
use super::{dim_type::System, SystemDim};
use crate::global_types::{quantity::Quantity, QName, SiExtended};
impl Quantity {
    pub const fn from_name(name: QName, dim_type: System) -> Self {
        match dim_type {
            System::SiExtended => match name {
                QName::Time => Self {
                    name,
                    dimensions: SystemDim::SiExtended(SiExtended {
                        T: 1i8,
                        ..Default::default()
                    }),
                },
                QName::Length => Self {
                    name,
                    dimensions: SystemDim::SiExtended(SiExtended {
                        L: 1i8,
                        ..Default::default()
                    }),
                },
            },
        }
    }
}
