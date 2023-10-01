use const_units_global_types::str_to_ident;
use convert_case::{Case, Casing, Encased};
use proc_macro2::TokenStream;
use quote::quote;
use std::{collections::HashMap, str::FromStr};

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
    let name = str_to_ident(&quantity.name());
    let systemname_expr = str_to_ident(&systemname.raw());

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
            let dimension_name = str_to_ident(&dimension_name);
            quote!(
                #dimension_name: #power
            )
        })
        .collect()
}

pub(crate) fn generate_dimension_struct(
    systemname: Encased<{ Case::UpperCamel }>,
    dimension: &HashMap<String, i8>,
) -> TokenStream {
    let systemname = str_to_ident(&systemname.raw());
    let fields = generate_dimension_fields(dimension);

    quote!( #systemname::#systemname { #(#fields),* , ..#systemname::NONE} )
}

pub(crate) fn generate_q_name_enum(quantities: Vec<QuantitySer>) -> TokenStream {
    let variants = quantities.iter().map(|quantity|{
        str_to_ident(quantity.name())
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
        let systemname = str_to_ident(name);
        let variants = quantities.iter().map(|quantity| {
            let q_name = str_to_ident(quantity.name());
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
