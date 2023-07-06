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
        #[doc = "Information"]
        INFO: i8,
        #[doc = "temperature interval"]
        ΔΘ: i8,
        #[doc = "time"]
        T: i8,
        #[doc = "mass"]
        M: i8,
        #[doc = "luminous intensity"]
        J: i8,
        #[doc = "amount of substance"]
        N: i8,
        #[doc = "electrical current"]
        I: i8,
        #[doc = "length"]
        L: i8,
        #[doc = "Angle"]
        A: i8,
        #[doc = "thermodynamic temprature"]
        Θ: i8,
    }
    pub const NONE: SiExtended = SiExtended {
        INFO: 0,
        ΔΘ: 0,
        T: 0,
        M: 0,
        J: 0,
        N: 0,
        I: 0,
        L: 0,
        A: 0,
        Θ: 0,
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
                    dimensions: SystemDim::SiExtended(SiExtended { T: 1i8, ..NONE }),
                },
                QName::Length => Self {
                    name,
                    dimensions: SystemDim::SiExtended(SiExtended { L: 1i8, ..NONE }),
                },
            },
        }
    }
}
// CUT -----------------
#[doc = "time"]
pub const Time: SiExtended = SiExtended { T: 1i8, ..NONE };
#[doc = "measure of distance"]
pub const Length: SiExtended = SiExtended { L: 1i8, ..NONE };
enum QName {
    Time,
    Length,
}
#[derive(Debug, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize, Display)]
enum PName {
    Unkonw,
    gibi,
    kibi,
    zebi,
    yobi,
    mebi,
    pebi,
    exbi,
    tebi,
    micro,
    yotta,
    tera,
    femto,
    zepto,
    quecto,
    centi,
    deci,
    hecto,
    ronna,
    yocto,
    ronto,
    atto,
    pico,
    quetta,
    exa,
    nano,
    mega,
    zetta,
    deca,
    peta,
    kilo,
    milli,
    giga,
}
impl ::core::str::FromStr for PName {
    type Err = ::parse_display::ParseError;
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "gibi" => {
                return ::core::result::Result::Ok(Self::gibi);
            }
            "kibi" => {
                return ::core::result::Result::Ok(Self::kibi);
            }
            "zebi" => {
                return ::core::result::Result::Ok(Self::zebi);
            }
            "yobi" => {
                return ::core::result::Result::Ok(Self::yobi);
            }
            "mebi" => {
                return ::core::result::Result::Ok(Self::mebi);
            }
            "pebi" => {
                return ::core::result::Result::Ok(Self::pebi);
            }
            "exbi" => {
                return ::core::result::Result::Ok(Self::exbi);
            }
            "tebi" => {
                return ::core::result::Result::Ok(Self::tebi);
            }
            "micro" => {
                return ::core::result::Result::Ok(Self::micro);
            }
            "μ" => {
                return ::core::result::Result::Ok(Self::micro);
            }
            "yotta" => {
                return ::core::result::Result::Ok(Self::yotta);
            }
            "Y" => {
                return ::core::result::Result::Ok(Self::yotta);
            }
            "tera" => {
                return ::core::result::Result::Ok(Self::tera);
            }
            "T" => {
                return ::core::result::Result::Ok(Self::tera);
            }
            "femto" => {
                return ::core::result::Result::Ok(Self::femto);
            }
            "f" => {
                return ::core::result::Result::Ok(Self::femto);
            }
            "zepto" => {
                return ::core::result::Result::Ok(Self::zepto);
            }
            "z" => {
                return ::core::result::Result::Ok(Self::zepto);
            }
            "quecto" => {
                return ::core::result::Result::Ok(Self::quecto);
            }
            "q" => {
                return ::core::result::Result::Ok(Self::quecto);
            }
            "centi" => {
                return ::core::result::Result::Ok(Self::centi);
            }
            "c" => {
                return ::core::result::Result::Ok(Self::centi);
            }
            "deci" => {
                return ::core::result::Result::Ok(Self::deci);
            }
            "d" => {
                return ::core::result::Result::Ok(Self::deci);
            }
            "hecto" => {
                return ::core::result::Result::Ok(Self::hecto);
            }
            "h" => {
                return ::core::result::Result::Ok(Self::hecto);
            }
            "ronna" => {
                return ::core::result::Result::Ok(Self::ronna);
            }
            "R" => {
                return ::core::result::Result::Ok(Self::ronna);
            }
            "yocto" => {
                return ::core::result::Result::Ok(Self::yocto);
            }
            "y" => {
                return ::core::result::Result::Ok(Self::yocto);
            }
            "ronto" => {
                return ::core::result::Result::Ok(Self::ronto);
            }
            "r" => {
                return ::core::result::Result::Ok(Self::ronto);
            }
            "atto" => {
                return ::core::result::Result::Ok(Self::atto);
            }
            "a" => {
                return ::core::result::Result::Ok(Self::atto);
            }
            "pico" => {
                return ::core::result::Result::Ok(Self::pico);
            }
            "p" => {
                return ::core::result::Result::Ok(Self::pico);
            }
            "quetta" => {
                return ::core::result::Result::Ok(Self::quetta);
            }
            "Q" => {
                return ::core::result::Result::Ok(Self::quetta);
            }
            "exa" => {
                return ::core::result::Result::Ok(Self::exa);
            }
            "E" => {
                return ::core::result::Result::Ok(Self::exa);
            }
            "nano" => {
                return ::core::result::Result::Ok(Self::nano);
            }
            "n" => {
                return ::core::result::Result::Ok(Self::nano);
            }
            "mega" => {
                return ::core::result::Result::Ok(Self::mega);
            }
            "M" => {
                return ::core::result::Result::Ok(Self::mega);
            }
            "zetta" => {
                return ::core::result::Result::Ok(Self::zetta);
            }
            "Z" => {
                return ::core::result::Result::Ok(Self::zetta);
            }
            "deca" => {
                return ::core::result::Result::Ok(Self::deca);
            }
            "da" => {
                return ::core::result::Result::Ok(Self::deca);
            }
            "peta" => {
                return ::core::result::Result::Ok(Self::peta);
            }
            "P" => {
                return ::core::result::Result::Ok(Self::peta);
            }
            "kilo" => {
                return ::core::result::Result::Ok(Self::kilo);
            }
            "k" => {
                return ::core::result::Result::Ok(Self::kilo);
            }
            "milli" => {
                return ::core::result::Result::Ok(Self::milli);
            }
            "m" => {
                return ::core::result::Result::Ok(Self::milli);
            }
            "giga" => {
                return ::core::result::Result::Ok(Self::giga);
            }
            "G" => {
                return ::core::result::Result::Ok(Self::giga);
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
                    numerator: 1073741824i128,
                    denominator: 1i128,
                } => (PName::gibi, PName::Unknown),
                RatioConst {
                    numerator: 1024i128,
                    denominator: 1i128,
                } => (PName::kibi, PName::Unknown),
                RatioConst {
                    numerator: 1180591620717411303424i128,
                    denominator: 1i128,
                } => (PName::zebi, PName::Unknown),
                RatioConst {
                    numerator: 1208925819614629174706176i128,
                    denominator: 1i128,
                } => (PName::yobi, PName::Unknown),
                RatioConst {
                    numerator: 1048576i128,
                    denominator: 1i128,
                } => (PName::mebi, PName::Unknown),
                RatioConst {
                    numerator: 1125899906842624i128,
                    denominator: 1i128,
                } => (PName::pebi, PName::Unknown),
                RatioConst {
                    numerator: 1152921504606846976i128,
                    denominator: 1i128,
                } => (PName::exbi, PName::Unknown),
                RatioConst {
                    numerator: 1099511627776i128,
                    denominator: 1i128,
                } => (PName::tebi, PName::Unknown),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000i128,
                } => (PName::micro, PName::μ),
                RatioConst {
                    numerator: 1000000000000000000000000i128,
                    denominator: 1i128,
                } => (PName::yotta, PName::Y),
                RatioConst {
                    numerator: 1000000000000i128,
                    denominator: 1i128,
                } => (PName::tera, PName::T),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000i128,
                } => (PName::femto, PName::f),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000000i128,
                } => (PName::zepto, PName::z),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000000000000000i128,
                } => (PName::quecto, PName::q),
                RatioConst {
                    numerator: 1i128,
                    denominator: 100i128,
                } => (PName::centi, PName::c),
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
                    denominator: 1000000000000000000000000i128,
                } => (PName::yocto, PName::y),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000000000000i128,
                } => (PName::ronto, PName::r),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000000000i128,
                } => (PName::atto, PName::a),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000000i128,
                } => (PName::pico, PName::p),
                RatioConst {
                    numerator: 1000000000000000000000000000000i128,
                    denominator: 1i128,
                } => (PName::quetta, PName::Q),
                RatioConst {
                    numerator: 1000000000000000000i128,
                    denominator: 1i128,
                } => (PName::exa, PName::E),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000000000i128,
                } => (PName::nano, PName::n),
                RatioConst {
                    numerator: 1000000i128,
                    denominator: 1i128,
                } => (PName::mega, PName::M),
                RatioConst {
                    numerator: 1000000000000000000000i128,
                    denominator: 1i128,
                } => (PName::zetta, PName::Z),
                RatioConst {
                    numerator: 10i128,
                    denominator: 1i128,
                } => (PName::deca, PName::da),
                RatioConst {
                    numerator: 1000000000000000i128,
                    denominator: 1i128,
                } => (PName::peta, PName::P),
                RatioConst {
                    numerator: 1000i128,
                    denominator: 1i128,
                } => (PName::kilo, PName::k),
                RatioConst {
                    numerator: 1i128,
                    denominator: 1000i128,
                } => (PName::milli, PName::m),
                RatioConst {
                    numerator: 1000000000i128,
                    denominator: 1i128,
                } => (PName::giga, PName::G),
                _ => PName::Unknown,
            },
            Factor::Float(float) => match float {
                _ => PName::Unknown,
            },
        }
    }
}
