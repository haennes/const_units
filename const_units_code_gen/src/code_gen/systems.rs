use std::path::Path;

use convert_case::{Case, Casing, Encased};
use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::{parse_quantities, Dimensions, QSystemSer};

use super::generate_quantities;

const CODE_PATH: &str = "code";

pub(crate) fn generate_systems_base(systems: Vec<QSystemSer>) -> TokenStream {
    // Systems-Enums
    let dimensions_code = generate_sys_dim_types(&systems);
    let dim_structs_code = systems.iter().map(|system| {
        generate_system_dim_struct(system.name().clone(), system.dimensions().clone())
    });
    // generate the system-specific stuff

    quote!(
        use super::*;

        #dimensions_code

        //------------------Dim Structs

        #(#dim_structs_code)*
    )
}

fn generate_system(system: &QSystemSer) -> TokenStream {
    let dimensions = generate_system_dim_struct(system.name().clone(), system.dimensions().clone());

    let quantities = generate_quantities(
        parse_quantities(Path::new(&system.get_path())),
        system.name().clone(),
    );
    todo!()
}

/// generates the systems corresponding Dimensions and NONE and implements Default
fn generate_system_dim_struct(
    name: Encased<{ Case::UpperCamel }>,
    dimension: Dimensions,
) -> TokenStream {
    let fields_struct = dimension.iter().map(|(name, description)| {
        let name: syn::Ident =
            syn::parse_str(name).expect(&format!("parsing {} to Ident failed", name));
        quote!(

            #[doc = #description]
            pub #name: i8
        )
    });

    let fields_names = dimension
        .iter()
        .map(|(name, _)| -> syn::Ident { syn::parse_str(name).expect("parsing failed") });

    let systemname: syn::Ident =
        syn::parse_str(&name.raw()).expect(&format!("parsing {} to Ident failed", name.raw()));
    let system_module_name: syn::Ident = syn::parse_str(&name.raw().to_case(Case::Snake))
        .expect(&format!("parsing {} to Ident failed", name.raw()));

    quote!(
        #[allow(non_snake_case)]
        pub mod #systemname{
            #[derive(Default, core::marker::ConstParamTy, PartialEq, Eq, const_units_macros::DivUseConst, const_units_macros::MulUseConst, const_units_macros::NegUseConst, Clone, Copy, self_rust_tokenize::SelfRustTokenize)]
            pub struct #systemname {
                #( #fields_struct),*
            }
            pub const NONE: #systemname = #systemname{
                #(#fields_names: 0 ),*
            };
        }
    )
}

/// generates all the Dimension-related types that are generic over the systems
fn generate_sys_dim_types(systems: &Vec<QSystemSer>) -> TokenStream {
    let names = systems.iter().map(|system| -> syn::Ident {
        syn::parse_str(&system.name().raw()).expect("parsing failed")
    });
    let names_clone = names.clone();
    quote!(
        #[derive(PartialEq, Eq, parse_display::Display, self_rust_tokenize::SelfRustTokenize, Clone, parse_display::FromStr, core::marker::ConstParamTy)]
        pub enum System {
            #(#names)*
        }

        #[derive(Copy, PartialEq, Eq, parse_display::Display, self_rust_tokenize::SelfRustTokenize, Clone, const_units_macros::NegUseConst, const_units_macros::MulUseConst, const_units_macros::DivUseConst, core::marker::ConstParamTy)]
        #[display("{}")]
        pub enum SystemDim {
            #(#names_clone (#names_clone :: #names_clone))*
        }
    )
}
