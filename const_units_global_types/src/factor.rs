use const_ops::{Div, Mul};
//use const_traits::{From, Into};
use core::marker::ConstParamTy;
use getset::{CopyGetters, Getters};
use num_rational::Ratio as RatioNonConst;
use num_traits::FromPrimitive;
use self_rust_tokenize::SelfRustTokenize;
use std::{num::ParseFloatError, str::FromStr};

pub trait RatioConstTypeTrait:
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
pub struct RatioConst {
    pub(crate) numerator: i128,
    pub(crate) denominator: i128,
}

impl SelfRustTokenize for RatioConst {
    fn append_to_token_stream(&self, token_stream: &mut proc_macro2::TokenStream) {
        let (num, denom) = (self.numerator(), self.denominator());
        let tokens = quote::quote!(
            RatioConst::new_raw(#num, #denom)
        );
        token_stream.extend_one(tokens);
    }
}

impl RatioConst {
    pub const fn new_raw(numerator: i128, denominator: i128) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
    pub const fn set_one(&mut self) {
        self.numerator = 1;
        self.denominator = 1;
    }

    pub const fn is_one(&self) -> bool {
        self.numerator == 1 && self.denominator == 1
    }

    pub const fn new_ratio(ratio: RatioNonConst<i128>) -> Self {
        Self {
            numerator: *ratio.numer(),
            denominator: *ratio.denom(),
        }
        .reduced()
    }

    pub const fn inv(self) -> Self {
        Self::new_raw(self.denominator, self.numerator)
    }

    pub const fn reduced(self) -> Self {
        let mut val = self;
        val.reduce();
        val
    }

    pub const fn reduce(&mut self) {
        if self.denominator == 0 {
            panic!("denominator == 0");
        }
        if self.numerator == 0 {
            self.denominator = 1;
            return;
        }
        if self.numerator == self.denominator {
            self.set_one();
            return;
        }
        let g: i128 = Self::gcd(self.numerator, &self.denominator);

        // FIXME(#5992): assignment operator overloads
        // T: Clone + Integer != T: Clone + NumAssign

        #[allow(dead_code)]
        #[inline]
        const fn replace_with(x: &mut i128, f: impl ~const FnOnce(i128) -> i128) {
            let y = core::mem::replace(x, 0);
            *x = f(y);
        }

        self.numerator /= g;
        //replace_with(&mut self.numerator, |x| x / g.clone());

        self.denominator /= g;
        //replace_with(&mut self.denominator, |x| x / g);

        // keep denom positive!
        if self.denominator < 0 {
            //replace_with(&mut self.numerator, |x| -x);
            self.numerator = -self.numerator;
            //replace_with(&mut self.denominator, |x| -x);
            self.denominator = -self.denominator;
        }
    }

    const fn gcd(left: i128, other: &i128) -> i128 {
        // Use Stein's algorithm
        let mut m = left;
        let mut n = *other;
        if m == 0 || n == 0 {
            return (m | n).abs();
        }

        // find common factors of 2
        let shift = (m | n).trailing_zeros();

        // The algorithm needs positive numbers, but the minimum value
        // can't be represented as a positive one.
        // It's also a power of two, so the gcd can be
        // calculated by bitshifting in that case

        // Assuming two's complement, the number created by the shift
        // is positive for all numbers except gcd = abs(min value)
        // The call to .abs() causes a panic in debug mode
        if m == i128::min_value() || n == i128::min_value() {
            return ((1 << shift) as i128).checked_abs().unwrap();
        }

        // guaranteed to be positive now, rest like unsigned algorithm
        m = m.abs();
        n = n.abs();

        // divide n and m by 2 until odd
        m >>= m.trailing_zeros();
        n >>= n.trailing_zeros();

        while m != n {
            if m > n {
                m -= n;
                m >>= m.trailing_zeros();
            } else {
                n -= m;
                n >>= n.trailing_zeros();
            }
        }
        m << shift
    }
}

impl const const_traits::Into<RatioNonConst<i128>> for RatioConst {
    fn into(self) -> RatioNonConst<i128> {
        RatioNonConst::new_raw(self.numerator, self.denominator)
    }
}
// why is this restirction necessare
//FIXME
impl const Mul for RatioConst {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = const_ops::Mul::mul(self.numerator, rhs.numerator);
        let denom = const_ops::Mul::mul(self.denominator, rhs.denominator);

        Self::new_raw(num, denom)
    }
}

impl const Mul<F64> for RatioConst {
    type Output = Self;

    fn mul(self, _rhs: F64) -> Self::Output {
        todo!()
    }
}

impl const Div for RatioConst {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inv = rhs.inv();

        rhs_inv.mul(self)
    }
}

impl const Div<F64> for RatioConst
where
    RatioNonConst<i128>: FromPrimitive,
{
    type Output = Self;

    fn div(self, _rhs: F64) -> Self::Output {
        todo!()
    }
}

impl const Div<RatioConst> for F64
where
    RatioNonConst<i128>: FromPrimitive,
{
    type Output = RatioConst;

    fn div(self, _rhs: RatioConst) -> Self::Output {
        todo!()
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
    Ratio(RatioConst),
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
                Factor::Float(b) => Factor::Ratio(a.div(b)),
            },
            Factor::Float(a) => match rhs {
                Factor::Ratio(b) => Factor::Ratio(a.div(b)),
                Factor::Float(b) => Factor::Float({
                    let a: f64 = const_traits::Into::into(a);
                    let b: f64 = const_traits::Into::into(b);
                    const_traits::From::from(a / b)
                }),
            },
        }
    }
}
