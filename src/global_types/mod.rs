use crate::generated::QName;
pub mod prefix;
pub use prefix::*;
pub use unit::*;
pub use u_name::*;

pub mod printing_style;
pub mod quantity;
pub mod unit;
pub mod u_name;

#[derive(Clone, Copy)]
pub enum Operation {
    Mul(QName, QName),
    Div(QName, QName),
    Inv(QName),
}
