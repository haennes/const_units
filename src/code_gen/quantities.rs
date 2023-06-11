use std::{collections::HashMap, fs::File, io::Read, path::Path, str::FromStr};

use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};

const QUANTITIES_PATH: &str = "quantities";
const QUANTITIES_FILE_NAME: &str = "quantity.toml";

#[derive(Serialize, Deserialize)]
pub(crate) struct QuantitySer {
    name: String,
    description: Option<String>,
    reference_unit: String,
    dimension: HashMap<String, i8>,
}

pub(crate) fn parse_quantities(systempath: &Path) -> Vec<QuantitySer> {
    let path = systempath.join(QUANTITIES_PATH);
    path.read_dir()
        .expect(&format!("failed to open {}", path.display()))
        .into_iter()
        .filter_map(|folder_or_file| {
            let folder_or_file = folder_or_file.expect("could not  read folder");
            if folder_or_file.path().is_dir() {
                let mut children = folder_or_file.path().read_dir().expect(&format!(
                    "could not read folder {}",
                    folder_or_file.path().display(),
                ));
                let children_names = children.find_map(|readdir| {
                    let readdir = readdir.expect(&format!(
                        "could not read child of {}",
                        folder_or_file.path().display()
                    ));
                    if readdir.path().ends_with(QUANTITIES_FILE_NAME) {
                        Some(readdir)
                    } else {
                        None
                    }
                });
                Some(children_names.unwrap())
            } else {
                None
            }
        })
        .map(|quantity| {
            let mut file = File::open(quantity.path()).expect(&format!(
                "could not open file: {}",
                quantity.path().display()
            ));
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect(&format!(
                "could not read from: {}",
                quantity.path().display()
            ));
            toml::de::from_str(&contents).expect(&format!(
                "could not parse file: {} \n contents: {}",
                quantity.path().display(),
                contents
            ))
        })
        .collect()
}

pub(crate) fn generate_quantities(input: Vec<QuantitySer>, systemname: String) -> TokenStream {
    let all = input.iter().map(|q| generate_quantity(&systemname, q));
    TokenStream::from_iter(all)
}

fn generate_quantity(systemname: &String, quantity: &QuantitySer) -> TokenStream {
    let name: syn::Expr = syn::parse_str(&quantity.name).expect("failde to parse quantity name");

    let fields = quantity.dimension.iter().map(|(dimension_name, power)| {
        let dimension_name: syn::Expr = syn::parse_str(&dimension_name).expect("parsing failed");
        quote!(
            #dimension_name: #power,
        )
    });

    match &quantity.description {
        Some(description) => {
            let description: syn::Expr = syn::parse_str(&description).expect("parsing failed");

            quote!(
                #description
                pub const #name: #systemname = #systemname{ #(#fields),* }
            )
        }
        None => {
            quote!(
                pub const #name: #systemname = #systemname { #(#fields),* }
            )
        }
    }
}
