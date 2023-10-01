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
    extend_one,
    const_float_bits_conv,
    arc_unwrap_or_clone
)]



use std::path::Path;

use code_gen::{
    generate_q_from_name, generate_q_name_enum, generate_quantities,
    prefix::{
        generate_factor_into_p_name, generate_p_name_enum, generate_p_name_enum_from_str,
        generate_prefix_from_name,
    },
    systems::generate_systems_base,
};
use itertools::Itertools;
use parsing::parse_quantities;
use proc_macro2::TokenStream;

use crate::{
    code_gen::{
        generate_uname_enum, generate_units,
        get_name_from_dimensions::generate_get_name_from_dimensions_and_op,
    },
    parsing::parse_units,
    parsing::QUANTITIES_PATH
};

mod code_gen;
mod expected_tests;
mod parsing;

#[cfg(test)]
pub mod tests;

pub fn generate() -> TokenStream {
    let systems = parsing::parse_systems();
    let systems_clone = systems.clone();
    let (systems_code, systems_hashmap): (Vec<_>, Vec<_>) = systems_clone
        .iter()
        .map(|system| {
            let systempath = system.get_path();
            let systempath = Path::new(&systempath);

            let prefixes = parsing::parse_prefixes(systempath);
            let prefixes_vec = prefixes
                .clone()
                .iter()
                .flat_map(|(_, prefix_g)| prefix_g.clone().into_iter())
                .collect_vec();

            let prefixes_code: TokenStream = [
                generate_p_name_enum(prefixes_vec.clone()),
                generate_p_name_enum_from_str(prefixes_vec.clone()),
                generate_factor_into_p_name(prefixes_vec.clone()),
                generate_prefix_from_name(prefixes_vec),
            ]
            .iter()
            .cloned()
            .collect();

            let quantities = parse_quantities(systempath);
            let quantities_vec: Vec<_> = quantities.clone().values().cloned().collect();
            let quantities_code: TokenStream = [
                generate_quantities(quantities_vec.clone(), system.name().clone()),
                generate_q_name_enum(quantities_vec),
            ]
            .iter()
            .cloned()
            .collect();

            // let (units, units_code): (Vec<_>, Vec<_>) = quantities
            //     .iter()
            //     .map(|quantity| {
            //         let units = parse_units(&quantity.path().parent().unwrap(), prefixes.clone());
            //         (
            //             units.clone(),
            //             generate_units(
            //                 "EN".to_string(),
            //                 units.iter().map(|(k,v)|v).cloned().collect(),
            //                 system.name().clone(),
            //                 quantity.clone(),
            //             ),
            //         )
            //     })
            //     .unzip();
            //panic!("pre parsing units");
            let units = parse_units(&systempath.join(QUANTITIES_PATH), prefixes, quantities.clone());
            panic!("parsed {} units", units.len());
            let units_code =  generate_units("EN", units.values().cloned().collect(), system.name().clone());
            let u_name_code = vec![
                generate_uname_enum(units.values().cloned().collect(), "EN"),
                //generate_uname_inv_mul(units.iter().flatten().cloned().collect(), "EN"),
            ]
            .iter()
            .cloned()
            .collect();

            (
                vec![
                    prefixes_code,
                    quantities_code,
                    units_code,
                    u_name_code,
                ]
                .iter()
                .cloned()
                .collect::<TokenStream>(),
                (system.name().raw().clone(), quantities).clone(),
            )
        })
        .unzip();

    let system_code: TokenStream = [
        systems_code.iter().cloned().collect(),
        generate_systems_base(systems.clone()),
        generate_q_from_name(systems_hashmap.iter().map(|(k, v)|(k.clone(), v.values().cloned().collect_vec().clone())).collect()),
        generate_get_name_from_dimensions_and_op(
            systems,
            systems_hashmap
                .iter()
                .map(|(_, sys_vec)| sys_vec.values())
                .flatten()
                .cloned()
                .collect_vec(),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    println!("// CUT -----------------");
    println!("{}", system_code);
    println!("// CUT -----------------");
    system_code
}
