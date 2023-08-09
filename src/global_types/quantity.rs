use std;

use std::fmt::Display;

use const_ops::{Div, Mul, Neg};

use num_traits::NumAssignRef;
use self_rust_tokenize::SelfRustTokenize;
//use num_traits::One;

use super::Operation;

use crate::generated::get_name_from_dimensions_and_op;

use crate::generated::QName;
use crate::generated::SystemDim;
use core::marker::ConstParamTy;

#[derive(PartialEq, Eq, SelfRustTokenize, ConstParamTy)]
pub struct Quantity {
    //FIXME make this an option to allow sth like time * distance
    pub(crate) name: QName,
    pub(crate) dimensions: SystemDim,
}

impl const Neg for Quantity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let result = self.dimensions.neg();
        Self {
            name: get_name_from_dimensions_and_op(result, Operation::Neg(self.name)).unwrap(),
            dimensions: result,
        }
    }
}

impl const Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = self.dimensions.mul(rhs.dimensions);
        Self {
            name: get_name_from_dimensions_and_op(result, Operation::Mul(self.name, rhs.name))
                .unwrap(),
            dimensions: result,
        }
    }
}

impl const Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let result = self.dimensions.div(rhs.dimensions);
        Self {
            name: get_name_from_dimensions_and_op(result, Operation::Div(self.name, rhs.name))
                .unwrap(),
            dimensions: result,
        }
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{} ({})", self.name, self.dimensions)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

pub trait QuantityDataTraits:
    NumAssignRef
    + One
    //+ ~ const PseudoFromRational128 // same here
    + Display
    + Copy // needed because of as_DT
    {
}

impl<DT> QuantityDataTraits for DT where
    DT: NumAssignRef
        + One
        //+ ~ const PseudoFrom<Rational128> // same here
        //+ PseudoFromRational128
        + Display
        + Copy // needed because of as_DT
{
}

pub trait One {
    const ONE: Self;
}
