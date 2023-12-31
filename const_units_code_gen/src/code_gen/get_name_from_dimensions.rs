use super::generate_dimension_struct;
use crate::parsing::QSystemSer;
use crate::parsing::QuantitySer;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::Ident;

pub(crate) fn generate_get_name_from_dimensions_and_op(
    systems: Vec<QSystemSer>,
    quantities: Vec<QuantitySer>,
) -> TokenStream {
    let variants = systems
        .iter()
        .map(|system| generate_system(system, &quantities))
        .intersperse(quote!(,));
    quote!(

        pub(crate) const fn get_name_from_dimensions_and_op(
            result: SystemDim,
            operation: Operation,
        ) -> Option<QName> {
            match result {
                #(#variants),*
            }
        }
    )
}

fn generate_system(system: &QSystemSer, quantities: &Vec<QuantitySer>) -> TokenStream {
    // let duplicates = quantities
    //     .iter()
    //     .map(
    //         |QuantitySer {
    //              name,
    //              description,
    //              reference_unit,
    //              dimension,
    //          }| dimension.iter().map(|(_, power)| power).collect_vec(),
    //     )
    //     .duplicates();

    // if duplicates.count() > 0 {
    //     panic!("found units with the same dimensions")
    // }

    let q_dimensions = quantities.iter().map(|quantity| {
        let dimension_struct =
            generate_dimension_struct(system.name().clone(), &quantity.dimension());
        let quantity_name_option =
            get_name_from_dimensions_system_op(&quantities, &quantity.dimension(), system);
        let quantity_name_tokens = match quantity_name_option {
            Some(q_name) => {
                let q_name: Ident = syn::parse_str(&q_name).unwrap();
                quote!(QName::#q_name)
            }
            None => quote!(QName::None),
        };
        quote!( const {#dimension_struct} => Some(#quantity_name_tokens),)
    });

    let systemname: Ident = syn::parse_str(&system.name().raw()).expect(&format!(
        "failed to parse {} to an Ident",
        system.name().raw()
    ));
    quote!(
        SystemDim::#systemname (dimensions) => {
            match dimensions{
                #(#q_dimensions)*
                _ => None
            }
        }
    )
}

fn get_name_from_dimensions_system_op(
    quantities: &Vec<QuantitySer>,
    dimension: &HashMap<String, i8>,
    system: &QSystemSer,
) -> Option<String> {
    let mut candidates = quantities
        .iter()
        .filter(|quantity| *quantity.dimension() == *dimension);
    match candidates.next() {
        None => {
            panic!("not enough candidates");
            None
        }
        Some(first) => match candidates.next() {
            None => Some(first.name().clone()),
            Some(_second) => {
                //TODO change this to support different units having the same dimensions: e.g. Frequency and angular velocity in the Si-System
                //system will be useful here
                panic!("too many candidates");
                None
            }
        },
    }
}
