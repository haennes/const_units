use crate::code_gen::{self};
use crate::expected_tests::{
    CURRENT_SYSTEMS, EXPECTED_PREFIXES, EXPECTED_QUANTITIES, EXPECTED_QUANTITIES_SI,
};

use crate::parsing::{
    self, parse_quantities, ConversionSerSer, ConversionWAcc, FactorSer, PrefixGroup, PrefixSer,
    QuantitySer, UnitNameSerSer, UnitSer, UnitSerSer,
};
use convert_case::{Case, Casing};
use either::Either;
use hashmap_macro::hashmap;
use itertools::Itertools;
use std::path::Path;

#[test]
fn get_systems() {
    let systems = parsing::parse_systems()
        .iter()
        .map(|q| {
            (
                q.name().clone(),
                q.path().clone(),
                q.dimensions()
                    .iter()
                    .sorted()
                    .map(|(a, b)| (a.clone(), b.clone()))
                    .collect_vec(),
            )
        })
        .collect_vec();

    assert_eq!(systems, CURRENT_SYSTEMS());
}

#[test]
fn generate_systems() {
    let systems = parsing::parse_systems();
    let systems_code = code_gen::systems::generate_systems_base(systems);

    println!("{}", systems_code);
    println!("")
}

#[test]
fn generate_q_name_enum() {
    let quantities = parse_quantities(Path::new("data/si_extended"));
    let code = code_gen::generate_q_name_enum(quantities);

    println!("{}", code);
}

#[test]
fn generate_q_from_name() {
    let systems = parsing::parse_systems();
    let system = systems.first().unwrap();
    let quantities = parse_quantities(Path::new("data/si_extended"));

    let code = code_gen::generate_q_from_name(
        hashmap!(system.name().raw().to_case(Case::UpperCamel).clone() => quantities),
    );

    println!("{}", code);
}

#[test]
fn generate_generic_units() {
    let quantities = parse_quantities(Path::new("data/si_extended"));
    let code = code_gen::generate_generic_units(quantities);
    println!("{}", code);
}

#[test]
fn parse_prefixes() {
    let expected_prefixes = EXPECTED_PREFIXES();

    let systems = parsing::parse_systems();
    let prefixes = parsing::parse_prefixes(Path::new(&systems.first().unwrap().get_path()))
        .iter()
        .map(|(name, prefix)| (name.clone(), prefix.clone()))
        .collect_vec();

    let prefixes = sort_prefixes(prefixes);
    //panic!("{:#?}", prefixes);
    assert_eq!(prefixes, expected_prefixes);
}

fn sort_prefixes(prefixes: Vec<(String, PrefixGroup)>) -> Vec<(String, Vec<PrefixSer>)> {
    prefixes
        .iter()
        .map(|(group, data)| {
            (
                group.clone(),
                data.clone()
                    .into_iter()
                    .sorted_by_key(|prefix| prefix.name().to_string())
                    .collect_vec(),
            )
        })
        .sorted_by_key(|(name, _)| name.clone())
        .collect_vec()
}

#[test]
fn generate_p_name_p_name_from_factor() {
    let prased_systems = parsing::parse_systems();
    let system = prased_systems.first().unwrap();
    let system_path = system.get_path();
    let systempath = Path::new(&system_path);
    let prefixes = parsing::parse_prefixes(&systempath)
        .iter()
        .flat_map(|(_, prefix_g)| prefix_g.clone().into_iter())
        .collect_vec();

    let code = code_gen::prefix::generate_factor_into_p_name(prefixes);
    println!("{}", code)
}

#[test]
fn parse_quantities_si_extended() {
    let quantities = parse_quantities(Path::new("data/si_extended"));
    assert_eq!(quantities, EXPECTED_QUANTITIES())
}

#[test]
fn generate_quantities_si_extended() {
    let quantities = parse_quantities(Path::new("data/si_extended"));
    let code = code_gen::generate_quantities(quantities, "si_extended".encased()).to_string();
    println!("{}", code);
    assert_eq!(code, EXPECTED_QUANTITIES_SI().to_string())
}

#[test]
fn serialize_unit_test() {
    for line in serialize_unit().split("\n") {
        println!("{}", line)
    }
}

fn serialize_unit() -> String {
    let unit_name_en =
        UnitNameSerSer::new(Either::Right(("meter".to_string(), "meters".to_string())));
    let unit_name_de = UnitNameSerSer::new(Either::Left("Meter".to_string()));
    let unit = UnitSerSer {
        symbol: "m".to_string(),
        names: hashmap!("EN".to_string() => unit_name_en.into(), "DE".to_string() => unit_name_de),
        derive_prefixes: None,
        conversions: None,
    };
    let toml = toml::ser::to_string_pretty(&unit).expect("failed to parse");

    toml
}

#[test]
fn generate_p_name_enum() {
    let prased_systems = parsing::parse_systems();
    let system = prased_systems.first().unwrap();
    let system_path = system.get_path();
    let systempath = Path::new(&system_path);
    let prefixes = parsing::parse_prefixes(&systempath)
        .iter()
        .flat_map(|(_, prefix_g)| prefix_g.clone().into_iter())
        .collect_vec();

    let code = code_gen::prefix::generate_p_name_enum(prefixes);
    println!("{}", code)
}

#[test]
fn generate_p_name_enum_from_str() {
    let prased_systems = parsing::parse_systems();
    let system = prased_systems.first().unwrap();
    let system_path = system.get_path();
    let systempath = Path::new(&system_path);
    let prefixes = parsing::parse_prefixes(&systempath)
        .iter()
        .flat_map(|(_, prefix_g)| prefix_g.clone().into_iter())
        .collect_vec();

    let code = code_gen::prefix::generate_p_name_enum_from_str(prefixes);
    println!("{}", code)
}

#[test]
fn generate_p_name_enum_all() {
    let prased_systems = parsing::parse_systems();
    let system = prased_systems.first().unwrap();
    let system_path = system.get_path();
    let systempath = Path::new(&system_path);
    let prefixes = parsing::parse_prefixes(&systempath)
        .iter()
        .flat_map(|(_, prefix_g)| prefix_g.clone().into_iter())
        .collect_vec();

    let code_impl = code_gen::prefix::generate_p_name_enum_from_str(prefixes.clone());
    let code = code_gen::prefix::generate_p_name_enum(prefixes);

    println!("{}{}", code_impl, code)
}
#[test]
fn generate_get_name_from_dimensions_and_op() {
    let prased_systems = parsing::parse_systems();
    let quantities = parse_quantities(Path::new("data/si_extended"));
    let code = code_gen::get_name_from_dimensions::generate_get_name_from_dimensions_and_op(
        prased_systems,
        quantities,
    );

    println!("{}", code)
}

#[test]
fn generate_u_name_enum() {
    let prased_systems = parsing::parse_systems();
    let system = prased_systems.first().unwrap();
    let system_path = system.get_path();
    let systempath = Path::new(&system_path);
    let quantities = parse_quantities(&systempath);
    let prefixes = parsing::parse_prefixes(&systempath);
    // .iter()
    // .map(|(group_name, prefixes)| {
    //     let prefixes = prefixes.clone().into_iter();
    //     (group_name.clone(), prefixes.collect_vec())
    // })
    // .collect();
    let units = parsing::parse_units(&systempath.join(Path::new("quantities/length")), prefixes);
    let code = code_gen::unit::generate_uname(units, "EN");
    println!("{}", code)
}

#[test]
fn generate_units_length() {
    let prased_systems = parsing::parse_systems();
    let system = prased_systems.first().unwrap();
    let system_path = system.get_path();
    let systempath = Path::new(&system_path);
    let quantities = parse_quantities(&systempath);
    let length = quantities
        .iter()
        .find(|quantity| quantity.name() == "Length")
        .unwrap();
    let prefixes = parsing::parse_prefixes(&systempath);
    // .iter()
    // .map(|(group_name, prefixes)| {
    //     let prefixes = prefixes.clone().into_iter();
    //     (group_name.clone(), prefixes.collect_vec())
    // })
    // .collect();

    let units: Vec<UnitSer> = vec![
        (parsing::parse_units(&systempath.join(Path::new("quantities/length")), prefixes)
            .iter()
            .find(|unit| unit.symbol == "m".to_string())
            .unwrap()
            .clone()),
    ];

    println!("{:#?}", units);
    let code = code_gen::unit::generate_units(
        "EN".to_string(),
        units,
        system.name().clone(),
        QuantitySer::new(
            "length",
            Path::new("/").to_path_buf(),
            None::<&str>,
            "length",
            hashmap!("L" => 1),
        ),
    );
    println!("success \n {}", code);
    println!("");
}
