use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::UnitSer;

pub(crate) fn generate_get_uname_from_op(unit_names: Vec<UnitSer>) -> TokenStream {
    quote!(
        pub fn get_uname_from_op(a: UName, b: UName, op: Operation) -> UName {}
    )
}
