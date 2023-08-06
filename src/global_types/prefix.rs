use core::marker::ConstParamTy;
//use core::ops::{Div, Mul};
use const_ops::{Div, Mul};

use crate::generated::PName;
use self_rust_tokenize::SelfRustTokenize;
//use crate::parsing::{PrefixSer, PrefixSerSer};
use crate::Factor;

//TODO make this active asap
// impl From<Factor> for Prefix {
//     fn from(value: Factor) -> Self {
//         let (name, alias) = value.into();
//         Self {
//             name,
//             alias,
//             factor: value,
//         }
//     }
// }

// impl Prefix {
//     const fn from(value: Factor) -> Self {
//         let (name, alias) = value.into();
//         Self {
//             name,
//             alias,
//             factor: value,
//         }
//     }
// }

#[derive(Debug, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize)]
pub struct Prefix {
    //TODO turn both into Options
    pub name: PName,
    pub alias: PName,
    pub factor: Factor,
}

// impl From<PrefixSer> for Prefix {
//     fn from(val: PrefixSer) -> Self {
//         Self {
//             name: PName::from_str(&val.name())
//                 .expect(&format!("failed to parse {} to a PName", val.name())),
//             alias: match val.alias() {
//                 Some(alias) => {
//                     PName::from_str(&alias).expect(&format!("failed to parse {} to a PName", alias))
//                 }
//                 //HACK should be an Option
//                 None => PName::Unknown,
//             },
//             factor: (*val.factor()).into(),
//         }
//     }
// }

impl const Mul for Prefix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let factor = const_ops::Mul::mul(self.factor, rhs.factor);
        let name = PName::from_factor(factor);
        Prefix {
            name,
            alias: name,
            factor,
        }
    }
}

impl const Div for Prefix {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let factor = self.factor.div(rhs.factor);
        let name = PName::from_factor(factor);
        Prefix {
            name,
            alias: name,
            factor,
        }
    }
}
