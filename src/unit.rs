use std::collections::HashMap;

use crate::global_types::QName;
use itertools::Itertools;
use num_rational::Ratio;
use proc_macro2::TokenStream;
use self_rust_tokenize::SelfRustTokenize;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

use crate::{
    code_gen::factor::{parse_to_int, Factor, FactorSer},
    global_types::quantity::Quantity,
    quantities::QuantitySer,
};

const UNIT_PATH: &str = "units";

#[derive(Serialize, Deserialize)]
pub(crate) struct UnitSerSer {
    symbol: String,
    names: HashMap<String, UnitNameSerSer>,
    derive_prefixes: Vec<String>,
    conversions: HashMap<String, ConversionSerSer>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ConversionSerSer {
    factor: FactorSer,
    accuracy: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UnitNameSerSer {
    singular: String,
    plural: Option<String>,
}

pub(crate) struct UnitSer {
    symbol: String,
    names: HashMap<String, UnitNameSer>,
    derive_prefixes: Vec<String>,
    conversions: HashMap<String, ConversionSer>,
}

pub(crate) struct ConversionSer {
    factor: Factor,
    accuracy: AccuracySer,
}

#[derive(Clone)]
pub(crate) struct UnitNameSer {
    singular: String,
    plural: String,
}

impl IntoIterator for UnitNameSer {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        if self.singular == self.plural {
            vec![self.singular].into_iter()
        } else {
            vec![self.singular, self.plural].into_iter()
        }
    }
}

pub(crate) enum AccuracySer {
    Exact,
    Inaccurate(i32),
}

impl Into<UnitSer> for UnitSerSer {
    fn into(self) -> UnitSer {
        UnitSer {
            symbol: self.symbol,
            names: self
                .names
                .iter()
                .map(|(key, value)| (key.clone(), (value.clone()).into()))
                .collect::<HashMap<String, UnitNameSer>>(),
            derive_prefixes: self.derive_prefixes,
            conversions: self
                .conversions
                .iter()
                .map(|(key, value)| (key.clone(), (value.clone()).into()))
                .collect::<HashMap<String, ConversionSer>>(), //.into(),
        }
    }
}

impl Into<ConversionSer> for &ConversionSerSer {
    fn into(self) -> ConversionSer {
        ConversionSer {
            factor: self.factor.clone().into(),
            accuracy: if self.accuracy.to_lowercase() == "e" {
                AccuracySer::Exact
            } else {
                AccuracySer::Inaccurate(parse_to_int(&self.accuracy) as i32)
            },
        }
    }
}

impl Into<UnitNameSer> for UnitNameSerSer {
    fn into(self) -> UnitNameSer {
        UnitNameSer {
            singular: self.singular.clone(),
            plural: {
                match self.plural {
                    Some(plural) => plural,
                    None => self.singular.clone(),
                }
            },
        }
    }
}

impl From<String> for UnitNameSer {
    fn from(value: String) -> Self {
        UnitNameSer {
            singular: value.clone(),
            plural: value,
        }
    }
}

pub(crate) fn generate_unit_code_name(
    q_name: String,
    name: UnitNameSer,
    data_type: String,
    dim_type: String,
) -> TokenStream {
    let singular = name.singular;
    let q_const = Quantity::from_name(
        q_name.parse().expect("UnitName not found"),
        dim_type.parse().expect("DimType not found"),
    )
    .to_tokens();
    quote::quote!(
        pub const #singular: Unit<#data_type, #q_name, #q_const> = Unit::new();
    )
}

pub(crate) fn generate_unit_code_lang(
    q_name: String,
    unit: &UnitSer,
    language: &String,
    data_type: String,
    dim_type: String,
) -> TokenStream {
    generate_unit_code_name(q_name, unit.names[language].clone(), data_type, dim_type)
}

pub(crate) fn generate_unit_code_symbol(
    unit: UnitSer,
    data_type: String,
    dim_type: String,
    q_name: String,
) -> TokenStream {
    generate_unit_code_name(q_name, unit.symbol.into(), data_type, dim_type)
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
            languages
                .iter()
                .map(|language| {
                    generate_unit_code_lang(
                        q_name.clone(),
                        unit,
                        language,
                        data_type.clone(),
                        dim_type.clone(),
                    )
                })
                .collect()
        })
        .collect()
}
