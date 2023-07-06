use core::marker::ConstParamTy;
use core::ops::Mul;
use std::str::FromStr;

use self_rust_tokenize::SelfRustTokenize;

use crate::generated::PName;
use crate::parsing::{PrefixSer, PrefixSerSer};

use super::factor::Factor;

//HACK This is a HACK see below
impl From<Option<PName>> for PName {
    fn from(value: Option<PName>) -> Self {
        match value {
            Some(panme) => panme,
            None => PName::Unknown,
        }
    }
}

impl From<Factor> for Prefix {
    fn from(value: Factor) -> Self {
        let (name, alias) = value.into();
        Self {
            name,
            alias,
            factor: value,
        }
    }
}

#[derive(Debug, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize)]
pub struct Prefix {
    //TODO turn both into Options
    pub name: PName,
    pub alias: PName,
    pub factor: Factor,
}

impl From<PrefixSer> for Prefix {
    fn from(val: PrefixSer) -> Self {
        Self {
            name: PName::from_str(&val.name())
                .expect(&format!("failed to parse {} to a PName", val.name())),
            alias: match val.alias() {
                Some(alias) => {
                    PName::from_str(&alias).expect(&format!("failed to parse {} to a PName", alias))
                }
                //HACK should be an Option
                None => PName::Unknown,
            },
            factor: (*val.factor()).into(),
        }
    }
}

impl Mul for Prefix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let factor = self.factor * rhs.factor;
        let (name, alias) = factor.clone().into();
        Prefix {
            name,
            alias,
            factor,
        }
    }
}

impl Into<Prefix> for (String, PrefixSerSer) {
    fn into(self) -> Prefix {
        Prefix {
            name: PName::from_str(&self.0).ok().into(),
            alias: {
                self.1
                    .alias
                    .map(|string| PName::from_str(&string).ok().into())
                    .into()
                // match self.1.alias {
                //     Some(str) => PName::from_str(&str).ok().into(),
                //     None => None.into(),
                // }
            },
            factor: self.1.factor.into(),
        }
    }
}
