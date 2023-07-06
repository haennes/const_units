use std::path::Path;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::{parse_quantities, Dimensions, QSystemSer};

use super::generate_quantities;

const CODE_PATH: &str = "code";

pub(crate) fn generate_systems_base(systems: Vec<QSystemSer>) -> TokenStream {
    // Systems-Enums
    let dimensions_code = generate_sys_dim_types(&systems);
    let dim_structs_code = systems
        .iter()
        .map(|system| generate_system_dim_struct(system.get_name(), system.dimensions().clone()));
    // generate the system-specific stuff

    quote!(
        use super::*;

        #dimensions_code

        //------------------Dim Structs

        #(#dim_structs_code)*
    )
}

fn generate_system(system: &QSystemSer) -> TokenStream {
    let dimensions = generate_system_dim_struct(system.get_name(), system.dimensions().clone());

    let quantities = generate_quantities(
        parse_quantities(Path::new(&system.get_path())),
        system.get_name(),
    );
    todo!()
}

/// generates the systems corresponding Dimensions and NONE and implements Default
fn generate_system_dim_struct(systemname: String, dimension: Dimensions) -> TokenStream {
    let fields_struct = dimension.iter().map(|(name, description)| {
        // let comment: syn::Expr = syn::parse_str(&format!("/// {}", description))
        //     .expect(&format!("parsing failed input: {} |", description));
        let name: syn::Ident =
            syn::parse_str(name).expect(&format!("parsing {} to Ident failed", name));
        quote!(

            #[doc = #description]
            #name: i8
        )
    });

    let fields_names = dimension
        .iter()
        .map(|(name, _)| -> syn::Ident { syn::parse_str(name).expect("parsing failed") });

    let systemname: syn::Ident =
        syn::parse_str(&systemname).expect(&format!("parsing {} to Ident failed", systemname));

    quote!(
        pub use #systemname::#systemname;
        mod #systemname{
            #[derive(Default)]
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
        syn::parse_str(&system.get_name().to_case(Case::UpperCamel)).expect("parsing failed")
    });
    let names_clone = names.clone();
    //panic!("{:?}", names.collect_vec());
    quote!(
        #[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, FromStr, ConstParamTy)]
        pub enum System {
            #(#names)*
        }

        #[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, Neg, Mul, Div, ConstParamTy)]
        #[display("{}")]
        pub enum SystemDim {
            #(#names_clone (#names_clone))*
        }
    )
}
