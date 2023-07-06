use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use std::{collections::HashMap, str::FromStr};
use syn::Ident;

use crate::parsing::QuantitySer;

pub(crate) fn generate_quantities(input: Vec<QuantitySer>, systemname: String) -> TokenStream {
    let all = input.iter().map(|q| generate_quantity(&systemname, q));
    TokenStream::from_iter(all)
}

fn generate_quantity(systemname: &String, quantity: &QuantitySer) -> TokenStream {
    let name: syn::Expr = syn::parse_str(&quantity.name()).expect("failed to parse quantity name");
    let systemname_expr: syn::Expr =
        syn::parse_str(&systemname.to_case(Case::UpperCamel)).expect("failed to parse systemname");

    let dim_struct = generate_dimension_struct(&systemname, &quantity.dimension());

    match &quantity.description() {
        Some(description) => TokenStream::from_str(&format!(
            "///{}
            {}",
            description,
            quote!(
                pub const #name: #systemname_expr = #dim_struct;
            )
        ))
        .expect("error"),
        None => {
            quote!(
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
    systemname: &String,
    dimension: &HashMap<String, i8>,
) -> TokenStream {
    let systemname: syn::Expr =
        syn::parse_str(&systemname.to_case(Case::UpperCamel)).expect("failed to parse systemname");
    let fields = generate_dimension_fields(dimension);

    quote!( #systemname { #(#fields),* ..NONE} )
}

pub(crate) fn generate_q_name_enum(quantities: Vec<QuantitySer>) -> TokenStream {
    let variants = quantities.iter().map(|quantity| -> Ident {
        syn::parse_str(quantity.name())
            .expect(&format!("failed to parse {} to an Ident", quantity.name()))
    });
    quote!(
        enum QName{
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
            let dim_struct = generate_dimension_struct(name, quantity.dimension());
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

        use super::{dim_type::System, SystemDim};
        use crate::global_types::{quantity::Quantity, QName, SiExtended};

        impl Quantity {
            pub const fn from_name(name: QName, dim_type: System) -> Self {
                match dim_type {
                    #(#systems),*
                }
            }
        }
    )
}
