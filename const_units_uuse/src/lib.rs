#![feature(iterator_try_collect)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Error, Ident, Token};

pub(crate) struct UUse(Vec<UnitImpl>);

pub(crate) struct UnitImpl {
    name: String,
    data_type: String,
}

#[proc_macro]
pub fn uuse(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let uuse = parse_macro_input!(ts as UUse);
    generate_uuse(uuse).into()
}

pub(crate) fn generate_uuse(uuse: UUse) -> TokenStream {
    uuse.0
        .iter()
        .map(|unit_impl| generate_unit(unit_impl))
        .collect()
}

fn generate_unit(unit_impl: &UnitImpl) -> TokenStream {
    let name: Ident = syn::parse_str(&unit_impl.name).unwrap();
    let data_type: Ident = syn::parse_str(&unit_impl.data_type).unwrap();
    let name_type: Ident =
        syn::parse_str(&format!("{}{}", unit_impl.name, unit_impl.data_type)).unwrap();
    quote!(
        pub const #name_type: crate::generated::#name<#data_type> = crate::generated::#name::<#data_type>::new();
    )
}

impl Parse for UUse {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut idents = Vec::new();
        idents.push(input.parse::<Ident>()?);
        let mut lookahead = input.lookahead1();
        while lookahead.peek(Token!(,)) {
            input.parse::<Token!(,)>()?;
            idents.push(input.parse()?);
            lookahead = input.lookahead1();
        }
        Ok(UUse {
            0: idents
                .iter()
                .map(|ident| match ident.to_string().split_once("_") {
                    Some((name, data_type)) => Ok(UnitImpl {
                        name: name.to_owned(),
                        data_type: data_type.to_owned(),
                    }),
                    None => return Err(Error::new(ident.span(), "_ not found")),
                })
                .try_collect()?,
        })
    }
}
