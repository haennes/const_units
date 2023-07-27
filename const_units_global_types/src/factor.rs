use core::marker::ConstParamTy;
use core::ops::Mul;
use getset::{CopyGetters, Getters};
use num_rational::Ratio as RatioNonConst;
use num_traits::FromPrimitive;
use self_rust_tokenize::SelfRustTokenize;
use std::{num::ParseFloatError, ops::Div, str::FromStr};

#[derive(Debug, ConstParamTy, PartialEq, Eq, Copy, Clone, Getters, CopyGetters)]
#[getset(get = "pub")]
pub struct RatioConst<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> {
    pub(crate) numerator: T,
    pub(crate) denominator: T,
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> SelfRustTokenize
    for RatioConst<T>
{
    fn append_to_token_stream(&self, token_stream: &mut proc_macro2::TokenStream) {
        let (num, denom) = (self.numerator(), self.denominator());
        let tokens = quote::quote!(
            RatioConst::new_raw(#num, #denom)
        );
        token_stream.extend_one(tokens);
    }
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> RatioConst<T> {
    pub const fn new_raw(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn new_ratio(ratio: RatioNonConst<T>) -> Self {
        let ratio = ratio.reduced();
        Self {
            numerator: ratio.numer().clone(),
            denominator: ratio.denom().clone(),
        }
    }
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Into<RatioNonConst<T>>
    for RatioConst<T>
{
    fn into(self) -> RatioNonConst<T> {
        RatioNonConst::new(self.numerator, self.denominator)
    }
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Mul for RatioConst<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a: RatioNonConst<T> = self.into();
        let b: RatioNonConst<T> = rhs.into();

        Self::new_ratio(a * b)
    }
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Mul<F64> for RatioConst<T>
where
    RatioNonConst<T>: FromPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: F64) -> Self::Output {
        let a: RatioNonConst<T> = self.into();
        let b: f64 = rhs.into();
        RatioConst::new_ratio(a * RatioNonConst::from_f64(b).unwrap())
    }
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Div for RatioConst<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let a: RatioNonConst<T> = self.into();
        let b: RatioNonConst<T> = rhs.into();

        Self::new_ratio(a / b)
    }
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Div<F64> for RatioConst<T>
where
    RatioNonConst<T>: FromPrimitive,
{
    type Output = Self;

    fn div(self, rhs: F64) -> Self::Output {
        let a: RatioNonConst<T> = self.into();
        let b: f64 = rhs.into();
        RatioConst::new_ratio(a / RatioNonConst::from_f64(b).unwrap())
    }
}

impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Div<RatioConst<T>> for F64
where
    RatioNonConst<T>: FromPrimitive,
{
    type Output = RatioConst<T>;

    fn div(self, rhs: RatioConst<T>) -> Self::Output {
        let a: f64 = self.into();
        let b: RatioNonConst<T> = rhs.into();
        RatioConst::new_ratio(RatioNonConst::from_f64(a).unwrap() / b)
    }
}

#[derive(ConstParamTy, PartialEq, Eq, Debug, Copy, Clone, SelfRustTokenize)]
pub struct F64([u8; 8]); //big endian

impl FromStr for F64 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            0: f64::from_str(s)?.to_be_bytes(),
        })
    }
}

impl Into<f64> for F64 {
    fn into(self) -> f64 {
        f64::from_be_bytes(self.0)
    }
}

// impl const From<f64> for F64 {
//     fn from(value: f64) -> Self {
//         Self(value.to_be_bytes())
//     }
// }

impl F64 {
    pub const fn from_f64(value: f64) -> Self {
        Self {
            0: value.to_be_bytes(),
        }
    }
}

#[derive(Debug, Copy, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize)]
pub enum Factor {
    Ratio(RatioConst<i128>),
    Float(F64),
}

impl Mul<Factor> for Factor {
    type Output = Self;

    fn mul(self, rhs: Factor) -> Self::Output {
        match self {
            Factor::Ratio(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(a * b),
                Factor::Float(b) => Factor::Ratio(a * b),
            },
            Factor::Float(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(b * a),
                Factor::Float(b) => Factor::Float({
                    let a: f64 = a.into();
                    let b: f64 = b.into();
                    F64::from_f64(a * b)
                }),
            },
        }
    }
}

impl Div<Factor> for Factor {
    type Output = Self;

    fn div(self, rhs: Factor) -> Self::Output {
        match self {
            Factor::Ratio(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(a / b),
                Factor::Float(b) => Factor::Ratio(a / b),
            },
            Factor::Float(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(a / b),
                Factor::Float(b) => Factor::Float({
                    let a: f64 = a.into();
                    let b: f64 = b.into();
                    F64::from_f64(a / b)
                }),
            },
        }
    }
}
