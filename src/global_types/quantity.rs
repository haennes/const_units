use std;

use std::fmt::Display;

use std::ops::Div;

use std::ops::Mul;

use num_traits::NumAssignRef;
use self_rust_tokenize::SelfRustTokenize;
//use num_traits::One;

use super::Operation;

use crate::generated::get_name_from_dimensions_and_op;

use std::ops::Neg;

use crate::generated::QName;
use crate::generated::SystemDim;
use core::marker::ConstParamTy;

#[derive(PartialEq, Eq, SelfRustTokenize, ConstParamTy)]
pub struct Quantity {
    pub(crate) name: QName,
    pub(crate) dimensions: SystemDim,
}

impl Neg for Quantity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let result = self.dimensions.neg();
        Self {
            name: get_name_from_dimensions_and_op(result, Operation::Neg(self.name)),
            dimensions: result,
        }
    }
}

impl Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = self.dimensions.mul(rhs.dimensions);
        Self {
            name: get_name_from_dimensions_and_op(result, Operation::Mul(self.name, rhs.name)),
            dimensions: result,
        }
    }
}

impl Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let result = self.dimensions.div(rhs.dimensions);
        Self {
            name: get_name_from_dimensions_and_op(result, Operation::Div(self.name, rhs.name)),
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
    //+ Display
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
