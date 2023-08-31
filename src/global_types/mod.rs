use crate::generated::QName;
pub mod prefix;
pub use prefix::*;
pub use unit::*;

pub mod printing_style;
pub mod quantity;
pub mod unit;

#[derive(Clone, Copy)]
pub enum Operation {
    Mul(QName, QName),
    Div(QName, QName),
    Inv(QName),
}
