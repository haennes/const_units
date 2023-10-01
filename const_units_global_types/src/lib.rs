#![allow(incomplete_features)]
#![feature(
    adt_const_params,
    const_trait_impl,
    const_float_bits_conv,
    const_refs_to_cell,
    extend_one,
    const_fn_floating_point_arithmetic,
    const_option,
    const_mut_refs,
    const_replace
)]

pub mod factor;
pub use factor::*;
use syn::Ident;

pub fn str_to_ident(string: impl ToString) -> Ident{
    syn::parse_str(&string.to_string()).unwrap()
}