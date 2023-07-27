use convert_case::{Case, Casing, Encased};
use proc_macro2::TokenStream;
use quote::quote;
use std::{collections::HashMap, str::FromStr};
use syn::Ident;

use crate::parsing::QuantitySer;

pub(crate) fn generate_quantities(
    input: Vec<QuantitySer>,
    systemname: Encased<{ Case::UpperCamel }>,
) -> TokenStream {
    input
        .iter()
        .map(|q| generate_quantity(systemname.clone(), q))
        .collect()
}

fn generate_quantity(
    systemname: Encased<{ Case::UpperCamel }>,
    quantity: &QuantitySer,
) -> TokenStream {
    let name: syn::Expr = syn::parse_str(&quantity.name()).expect("failed to parse quantity name");
    let systemname_expr: syn::Expr =
        syn::parse_str(&systemname.raw()).expect("failed to parse systemname");

    let dim_struct = generate_dimension_struct(systemname, &quantity.dimension());

    let systemname_expr = quote!(#systemname_expr :: #systemname_expr);

    match &quantity.description() {
        Some(description) => TokenStream::from_str(&format!(
            "///{}
            {}",
            description,
            //FIXME allow dead_code
            quote!(
                #[allow(dead_code)]
                pub const #name: #systemname_expr = #dim_struct;
            )
        ))
        .expect("error"),
        None => {
            //FIXME allow dead_code
            quote!(
                #[allow(dead_code)]
                pub const #name: #systemname_expr = #dim_struct;
            )
        }
    }
}

pub(crate) fn generate_dimension_fields(dimension: &HashMap<String, i8>) -> Vec<TokenStream> {
    dimension
        .iter()
        .map(|(dimension_name, power)| {
            let dimension_name: syn::Expr =
                syn::parse_str(&dimension_name).expect("parsing failed");
            quote!(
                #dimension_name: #power,
            )
        })
        .collect()
}

pub(crate) fn generate_dimension_struct(
    systemname: Encased<{ Case::UpperCamel }>,
    dimension: &HashMap<String, i8>,
) -> TokenStream {
    let systemname: syn::Expr =
        syn::parse_str(&systemname.raw()).expect("failed to parse systemname");
    let fields = generate_dimension_fields(dimension);

    quote!( #systemname::#systemname { #(#fields),* ..#systemname::NONE} )
}

pub(crate) fn generate_q_name_enum(quantities: Vec<QuantitySer>) -> TokenStream {
    let variants = quantities.iter().map(|quantity| -> Ident {
        syn::parse_str(quantity.name())
            .expect(&format!("failed to parse {} to an Ident", quantity.name()))
    });
    quote!(
        #[derive(Eq, PartialEq, std::marker::ConstParamTy, Clone, Copy, parse_display::Display, self_rust_tokenize::SelfRustTokenize)]
        pub enum QName{
            #(#variants),*
        }
    )
}

pub(crate) fn generate_q_from_name(systems: HashMap<String, Vec<QuantitySer>>) -> TokenStream {
    let systems = systems.iter().map(|(name, quantities)| {
        let systemname: Ident =
            syn::parse_str(name).expect(&format!("failed to parse {} to an Ident", name));
        let variants = quantities.iter().map(|quantity| {
            let q_name: Ident = syn::parse_str(quantity.name())
                .expect(&format!("failed to parse {} to an Ident", quantity.name()));
            let dim_struct = generate_dimension_struct(name.encased(), quantity.dimension());
            quote!(QName::#q_name => Self{
                name,
                dimensions: SystemDim::#systemname(#dim_struct)
            })
        });
        quote!(System::#systemname => match name {
            #(#variants),*
        })
    });
    quote!(

        impl crate::Quantity {
            pub const fn from_name(name: crate::QName, dim_type: crate::System) -> Self {
                match dim_type {
                    #(#systems),*
                }
            }
        }
    )
}
