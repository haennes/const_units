mod derive_adding_mul;
mod derive_div;
mod derive_mul;
mod derive_neg;
mod derive_subbing_div;

use derive_adding_mul::generate_derive_adding_mul;
use derive_div::generate_derive_div;
use derive_mul::generate_derive_mul;
use derive_neg::generate_derive_neg;
use derive_subbing_div::generate_derive_subbing_div;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Neg)]
pub fn derive_neg(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_neg(ident, data).into()
}

#[proc_macro_derive(Mul)]
pub fn derive_mul(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_mul(ident, data).into()
}

#[proc_macro_derive(Div)]
pub fn derive_div(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_div(ident, data).into()
}

/// similar to deriving Mul, can only be used on structs and implements the Mul trait
/// DIFFERENCE: each FIELD is ADDED to get the Product instead of multiplied
#[proc_macro_derive(AddingMul)]
pub fn derive_adding_mul(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_adding_mul(ident, data).into()
}

/// similar to deriving Div, can only be used on structs and implements the Div trait
/// DIFFERENCE: each FIELD is SUBTRACTED to get the Product instead of divided
#[proc_macro_derive(SubbingDiv)]
pub fn derive_subbing_div(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_subbing_div(ident, data).into()
}
