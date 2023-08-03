use const_ops::{Div, Mul};
use const_traits::{From, Into};
use core::marker::ConstParamTy;
use getset::{CopyGetters, Getters};
use num_rational::Ratio as RatioNonConst;
use num_traits::FromPrimitive;
use self_rust_tokenize::SelfRustTokenize;
use std::{num::ParseFloatError, str::FromStr};

trait RatioConstTypeTrait:
    Copy
    + Eq
    + Ord
    + num_integer::Integer
    + quote::ToTokens
    + ~const const_ops::Mul<Output = Self>
    + ConstParamTy
{
}
impl<
        T: Copy
            + Eq
            + Ord
            + num_integer::Integer
            + quote::ToTokens
            + ~const const_ops::Mul<Output = Self>
            + ConstParamTy,
    > RatioConstTypeTrait for T
{
}

#[derive(Debug, ConstParamTy, PartialEq, Eq, Copy, Clone, Getters, CopyGetters)]
#[getset(get = "pub")]
pub struct RatioConst<T: RatioConstTypeTrait> {
    pub(crate) numerator: T,
    pub(crate) denominator: T,
}

impl<T: RatioConstTypeTrait> SelfRustTokenize for RatioConst<T> {
    fn append_to_token_stream(&self, token_stream: &mut proc_macro2::TokenStream) {
        let (num, denom) = (self.numerator(), self.denominator());
        let tokens = quote::quote!(
            RatioConst::new_raw(#num, #denom)
        );
        token_stream.extend_one(tokens);
    }
}

impl<T: RatioConstTypeTrait> RatioConst<T> {
    pub const fn new_raw(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub const fn new_ratio(ratio: RatioNonConst<T>) -> Self {
        Self {
            numerator: *ratio.numer(),
            denominator: *ratio.denom(),
        }
        .reduced_const()
    }

    pub const fn reduced_const(self) -> Self {
        //TODO
        self
    }
}

impl<T: RatioConstTypeTrait> const const_traits::Into<RatioNonConst<T>> for RatioConst<T> {
    fn into(self) -> RatioNonConst<T> {
        RatioNonConst::new_raw(self.numerator, self.denominator)
    }
}
// why is this restirction necessare FIXME
impl<T: RatioConstTypeTrait + ~const const_ops::Mul> const Mul for RatioConst<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num: T = const_ops::Mul::mul(self.numerator, rhs.numerator);
        let denom: T = const_ops::Mul::mul(self.denominator, rhs.denominator);

        Self::new_raw(num, denom)
    }
}

impl<T: RatioConstTypeTrait> const Mul<F64> for RatioConst<T>
where
    RatioNonConst<T>: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: F64) -> Self::Output {
        let a: RatioNonConst<T> = const_traits::Into::into(self);
        let b: f64 = const_traits::Into::into(rhs);
        let b_ratio = RatioNonConst::from_f64(b).unwrap();
        RatioConst::new_ratio(std::ops::Mul::mul(a, b_ratio))
    }
}

impl<T: RatioConstTypeTrait> const Div for RatioConst<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let a: RatioNonConst<T> = const_traits::Into::into(self);
        let b: RatioNonConst<T> = const_traits::Into::into(rhs);

        Self::new_ratio(const_ops::Div::div(a, b))
    }
}

impl<T: RatioConstTypeTrait> const Div<F64> for RatioConst<T>
where
    RatioNonConst<T>: FromPrimitive,
{
    type Output = Self;

    fn div(self, rhs: F64) -> Self::Output {
        let a: RatioNonConst<T> = const_traits::Into::into(self);
        let b: f64 = const_traits::Into::into(rhs);
        RatioConst::new_ratio(a / RatioNonConst::from_f64(b).unwrap())
    }
}

impl<T: RatioConstTypeTrait> Div<RatioConst<T>> for F64
where
    RatioNonConst<T>: FromPrimitive,
{
    type Output = RatioConst<T>;

    fn div(self, rhs: RatioConst<T>) -> Self::Output {
        let a: f64 = const_traits::Into::into(self);
        let b: RatioNonConst<T> = const_traits::Into::into(rhs);
        //RatioConst::new_ratio(const_ops::Div::div(RatioNonConst::from_f64(a).unwrap(), b))
        RatioConst::new_ratio(std::ops::Div::div(RatioNonConst::from_f64(a).unwrap(), b))
    }
}

#[derive(ConstParamTy, PartialEq, Eq, Debug, Copy, Clone, SelfRustTokenize)]
pub struct F64([u8; 8]); //big endian

impl FromStr for F64 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(const_traits::From::from(f64::from_str(s)?))
    }
}

impl const const_traits::Into<f64> for F64 {
    fn into(self) -> f64 {
        f64::from_be_bytes(self.0)
    }
}

impl const const_traits::From<f64> for F64 {
    fn from(value: f64) -> Self {
        Self(value.to_be_bytes())
    }
}

#[derive(Debug, Copy, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize)]
pub enum Factor {
    Ratio(RatioConst<i128>),
    Float(F64),
}

impl const Mul<Factor> for Factor {
    type Output = Self;

    fn mul(self, rhs: Factor) -> Self::Output {
        match self {
            Factor::Ratio(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(a.mul(b)),
                Factor::Float(b) => Factor::Ratio(a.mul(b)),
            },
            Factor::Float(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(b.mul(a)),
                Factor::Float(b) => Factor::Float({
                    let a: f64 = const_traits::Into::into(a);
                    let b: f64 = const_traits::Into::into(b);
                    const_traits::From::from(a * b)
                }),
            },
        }
    }
}

impl const Div<Factor> for Factor {
    type Output = Self;

    fn div(self, rhs: Factor) -> Self::Output {
        match self {
            Factor::Ratio(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(a.div(b)),
                Factor::Float(b) => Factor::Ratio(const_ops::Div::div(a, b)),
            },
            Factor::Float(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(const_ops::Div::div(a, b)),
                Factor::Float(b) => Factor::Float({
                    let a: f64 = const_traits::Into::into(a);
                    let b: f64 = const_traits::Into::into(b);
                    const_traits::From::from(a / b)
                }),
            },
        }
    }
}
