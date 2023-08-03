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
use syn::{parse_macro_input, DeriveInput, Path};

#[proc_macro_derive(NegUseConst)]
pub fn derive_neg_const(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    let path: Path = syn::parse_str("const_ops").unwrap();
    generate_derive_neg(ident, data, path).into()
}
#[proc_macro_derive(NegUseCore)]
pub fn derive_neg_core(_item: TokenStream) -> TokenStream {
    let path: Path = syn::parse_str("core::ops").unwrap();
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_neg(ident, data, path).into()
}

#[proc_macro_derive(MulUseConst)]
pub fn derive_mul_const(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    let path: Path = syn::parse_str("const_ops").unwrap();
    generate_derive_mul(ident, data, path).into()
}
#[proc_macro_derive(MulUseCore)]
pub fn derive_mul_core(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    let path: Path = syn::parse_str("core::ops").unwrap();
    generate_derive_mul(ident, data, path).into()
}

#[proc_macro_derive(DivUseConst)]
pub fn derive_div_const(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    let path: Path = syn::parse_str("const_ops").unwrap();
    generate_derive_div(ident, data, path).into()
}
#[proc_macro_derive(DivUseCore)]
pub fn derive_div_core(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    let path: Path = syn::parse_str("core::ops").unwrap();
    generate_derive_div(ident, data, path).into()
}

/// similar to deriving Mul, can only be used on structs and implements the Mul trait
/// DIFFERENCE: each FIELD is ADDED to get the Product instead of multiplied
#[proc_macro_derive(AddingMulUseCore)]
pub fn derive_adding_mul_core(_item: TokenStream) -> TokenStream {
    let path: Path = syn::parse_str("core::ops").unwrap();
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_adding_mul(ident, data, path).into()
}
/// similar to deriving Mul, can only be used on structs and implements the Mul trait
/// DIFFERENCE: each FIELD is ADDED to get the Product instead of multiplied
#[proc_macro_derive(AddingMulUseConst)]
pub fn derive_adding_mul_const(_item: TokenStream) -> TokenStream {
    let path: Path = syn::parse_str("const_ops").unwrap();
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    generate_derive_adding_mul(ident, data, path).into()
}

/// similar to deriving Div, can only be used on structs and implements the Div trait
/// DIFFERENCE: each FIELD is SUBTRACTED to get the Product instead of divided
#[proc_macro_derive(SubbingDivUseCore)]
pub fn derive_subbing_div_core(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    let path: Path = syn::parse_str("core::ops").unwrap();
    generate_derive_subbing_div(ident, data, path).into()
}
/// similar to deriving Div, can only be used on structs and implements the Div trait
/// DIFFERENCE: each FIELD is SUBTRACTED to get the Product instead of divided
#[proc_macro_derive(SubbingDivUseConst)]
pub fn derive_subbing_div_const(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_item);
    let path: Path = syn::parse_str("const_ops").unwrap();
    generate_derive_subbing_div(ident, data, path).into()
}
