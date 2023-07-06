use crate::generated::prefix::PName;
use crate::global_types::quantity::Quantity;
use crate::parsing::{QuantitySer, UnitNameSer, UnitSer};
use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use self_rust_tokenize::SelfRustTokenize;
use syn::Ident;

pub(crate) fn generate_unit_code_name(
    q_name: String,
    name: UnitNameSer,
    data_type: String,
    dim_type: String,
    prefix: PName,
) -> TokenStream {
    let singular: TokenStream = syn::parse_str(&name.singular).unwrap();
    let q_const = Quantity::from_name(
        q_name
            .to_case(Case::UpperCamel)
            .parse()
            .expect(&format!("QName  {} not found ", q_name)),
        dim_type.parse().expect("DimType not found"),
    )
    .to_tokens();
    let prefix = prefix.to_tokens();
    quote::quote!(
        pub const #singular: Unit<#data_type, #q_const, #prefix> = Unit::new();
    )
}

pub(crate) fn generate_unit_code_lang(
    q_name: String,
    unit: &UnitSer,
    language: &String,
    data_type: String,
    dim_type: String,
    prefix: PName,
) -> TokenStream {
    generate_unit_code_name(
        q_name,
        unit.names[language].clone(),
        data_type,
        dim_type,
        prefix,
    )
}

pub(crate) fn generate_unit_code_symbol(
    unit: &UnitSer,
    data_type: String,
    dim_type: String,
    q_name: String,
    prefix: PName,
) -> TokenStream {
    generate_unit_code_name(
        q_name,
        unit.symbol.clone().into(),
        data_type,
        dim_type,
        prefix,
    )
}

pub(crate) fn generate_units(
    languages: Vec<String>,
    units: Vec<UnitSer>,
    data_type: String,
    dim_type: String,
    q_name: String,
) -> TokenStream {
    units
        .iter()
        .map(|unit| -> TokenStream {
            unit.prefixes
                .iter()
                .map(|prefix| -> TokenStream {
                    [prefix.name.clone(), prefix.alias.clone()]
                        .iter()
                        .map(|p_name| -> TokenStream {
                            languages
                                .iter()
                                .map(|language| {
                                    generate_unit_code_lang(
                                        q_name.clone(),
                                        unit,
                                        &language.to_case(Case::Upper).clone(),
                                        data_type.clone(),
                                        dim_type.clone(),
                                        prefix.name.clone(),
                                    )
                                })
                                .chain(
                                    [generate_unit_code_symbol(
                                        unit,
                                        data_type.clone(),
                                        dim_type.clone(),
                                        q_name.clone(),
                                        prefix.name.clone(),
                                    )]
                                    .into_iter(),
                                )
                                .collect()
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

pub(crate) fn generate_generic_units(quantities: Vec<QuantitySer>) -> TokenStream {
    quantities.iter().map(|quantity| {
        let name: Ident = syn::parse_str(quantity.name())
            .expect(&format!("failed to parse {} to an Iden", quantity.name()));
        quote!(
            pub type #name <DT,const NAME: UName, const PREFIX: Prefix> = Unit<DT, NAME, {crate::quantity::Quantity::from_name(QName::#name)}, PREFIX>;
        )
    }).collect()
}

pub(crate) fn generate_uname(units: Vec<UnitSer>, default_lang: impl ToString) -> TokenStream {
    let names = units.iter().map(|unit| -> Ident {
        syn::parse_str(&unit.symbol.clone()).expect(&format!(
            "failed to parse {} to an Ident",
            unit.symbol.clone()
        ))
    });

    quote!(
        pub enum UName{
            #(#names),*
        }
    )
}
