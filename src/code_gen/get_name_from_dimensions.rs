use proc_macro2::TokenStream;
use quote::quote;

use crate::global_types::quantity::Quantity;

use super::{generate_quantities, parse_quantities, quantities, systems::QSystemSer, QuantitySer};

pub(crate) fn generate_get_name_from_dimensions_and_op<'a>(
    systems: Vec<QSystemSer>,
    quantities: Vec<QuantitySer>,
) -> TokenStream {
    let variants = systems
        .iter()
        .map(|system| generate_get_name_from_dimensions_and_op_system(system));
    quote!(
        use crate::global_types::{DimType, Operation, QName, SystemDim};

        pub(crate) fn get_name_from_dimensions_and_op(
            result: SystemDim,
            operation: Operation,
        ) -> QName {
            match result {
                #(#variants),*
            }
        }
    )
}

fn generate_get_name_from_dimensions_and_op_system(system: &QSystemSer) -> TokenStream {
    //TODO: VERY INEFFICIENT AS PARSING HAPPENS EVERYTIME
    // let quantities = parse_quantities(system.get_path())
    //     .iter()
    //     .map(|quantity_ser| {
    //         let quantity: Quantity = quantity_ser.into();
    //         quantity.to_tokens()
    //     });

    todo!()
}
