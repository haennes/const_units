use super::systems::{get_systems, QSystemSer};
use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, str::FromStr};

const DIMENSIONS_PATH: &str = "dimensions.toml";
pub(crate) type Dimensions = HashMap<String, String>;

pub(crate) fn parse_dimensions(systempath: &Path) -> Dimensions {
    #[derive(Serialize, Deserialize)]
    struct DimensionsSer {
        Dimensions: HashMap<String, String>,
    }

    let path = systempath.join(DIMENSIONS_PATH);
    let contents =
        fs::read_to_string(path.clone()).expect(&format!("failed to read file {}", path.display()));
    let dim: DimensionsSer = toml::de::from_str(&contents).expect(&format!(
        "failed to parse file {} \n contents: {}",
        path.display(),
        contents
    ));
    dim.Dimensions
}

/// generates the systems corresponding Dimensions and NONE and implements Default
pub(crate) fn generate_dimension(systemname: String, dimension: Dimensions) -> TokenStream {
    let fields_struct = dimension.iter().map(|(name, description)| {
        let comment: syn::Expr =
            syn::parse_str(&format!("/// {description}")).expect("parsing failed");
        quote!(
            #comment
            #name: i8,
        )
    });

    let fields_names = dimension
        .iter()
        .map(|(name, _)| -> syn::Expr { syn::parse_str(name).expect("parsing failed") });

    quote!(
        pub struct #systemname {
            #( #fields_struct),*
        };

        pub const NONE: #systemname {
            #(#fields_names: 0 )*,
        }

        impl const Default for #systemname {
            fn default() -> Self { NONE }
        }
    )
}

/// generates all the Dimension-related types that are generic over the systems
pub(crate) fn generate_dim_types(systems: &Vec<QSystemSer>) -> TokenStream {
    let names = systems.iter().map(|system| -> syn::Expr {
        syn::parse_str(&system.get_name().to_case(Case::UpperCamel)).expect("parsing failed")
    });
    let names_clone = names.clone();
    quote!(
        #[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, FromStr)]
        pub enum System {
            #(#names),*
        }

        #[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, Neg, Mul, Div)]
        #[display("{}")]
        pub enum SystemDim {
            #(#names_clone (#names_clone)),*
        }
    )
}
