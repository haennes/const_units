//for dev:
#![allow(unused)]
#![warn(unused_imports)]
//
#![allow(incomplete_features)]
#![feature(
    iterator_try_collect,
    adt_const_params,
    const_trait_impl,
    iter_intersperse,
    allocator_api,
    inline_const,
    extend_one
)]

use std::{fs::File, io::Read, io::Write, path::Path};

use code_gen::{
    generate_q_from_name, generate_q_name_enum, generate_quantities,
    prefix::{
        generate_p_name_enum, generate_p_name_enum_from_str, generate_p_name_p_name_from_factor,
    },
    quantities,
    systems::generate_systems_base,
};
use itertools::Itertools;
use parsing::{parse_quantities, QuantitySer};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{code_gen::generate_units, parsing::parse_units};

mod code_gen;
mod expected_tests;
mod generated;
mod global_types;
mod parsing;
mod uuse;

#[cfg(test)]
pub mod tests;

fn main() {
    let systems = parsing::parse_systems();
    let systems_clone = systems.clone();
    let mut mod_file = File::open("src/generated/mod.rs").unwrap();
    let systems_hashmap = systems_clone.iter().map(|system| {
        let systempath = system.get_path();
        let systempath = Path::new(&systempath);

        let contents = &mut String::new();
        mod_file.read_to_string(contents);

        let prefixes = parsing::parse_prefixes(systempath.clone());
        let prefixes_vec = prefixes
            .clone()
            .iter()
            .flat_map(|(_, prefix_g)| prefix_g.clone().into_iter())
            .collect_vec();

        let prefixes_code: TokenStream = [
            generate_p_name_enum(prefixes_vec.clone()),
            generate_p_name_enum_from_str(prefixes_vec.clone()),
            generate_p_name_p_name_from_factor(prefixes_vec),
        ]
        .iter()
        .cloned()
        .collect();

        let all = format!("{}{}", contents, prefixes_code);
        print!("{}", all);
        write!(mod_file, "{}", all);

        let quantities = parse_quantities(systempath.clone());
        let quantities_code: TokenStream = [
            generate_quantities(quantities.clone(), system.get_name()),
            generate_q_name_enum(quantities.clone()),
        ]
        .iter()
        .cloned()
        .collect();

        let all = format!("{}{}{}", contents, prefixes_code, quantities_code);
        write!(mod_file, "{}", all);

        let units_code: TokenStream = quantities
            .iter()
            .map(|quantity| {
                let units = parse_units(&quantity.path().clone(), prefixes.clone());
                generate_units(
                    vec!["EN".to_string()],
                    units,
                    "u16".to_string(),
                    system.get_name(),
                    quantity.name().clone(),
                )
            })
            .collect();

        let all = format!(
            "{}{}{}{}",
            contents, prefixes_code, quantities_code, units_code
        );
        write!(mod_file, "{}", all);

        (system.get_name(), quantities).clone()
    });

    let system_code: TokenStream = [
        generate_systems_base(systems),
        generate_q_from_name(systems_hashmap.collect()),
    ]
    .iter()
    .cloned()
    .collect();

    println!("{}", system_code);
    println!("// CUT -----------------");
}

// use proc_macro2::TokenStream;
// use syn::parse_macro_input;
// use uuse::{generate_uuse, UUse};
//
// #[proc_macro]
// pub fn uuse(ts: TokenStream) -> TokenStream {
//     let uuse = parse_macro_input!(ts as UUse);
//     generate_uuse(uuse)
// }
