//for dev:
#![allow(unused_imports)]
//
#![allow(incomplete_features)]
#![feature(iterator_try_collect, adt_const_params, const_trait_impl)]

mod code_gen;
mod generated;
mod global_types;
mod unit;
mod uuse;

pub(crate) use code_gen::*;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, parse_macro_input, DeriveInput, Ident, LitStr, Token};
use uuse::{generate_uuse, UUse};

#[proc_macro]
pub fn uuse(ts: TokenStream) -> TokenStream {
    let uuse = parse_macro_input!(ts as UUse);
    generate_uuse(uuse)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::code_gen::{
        self,
        systems::{self, QSystemSer},
        PrefixSer,
    };
    use hashmap_macro::hashmap;
    use itertools::Itertools;

    #[test]
    fn get_systems() {
        #[derive(Debug, PartialEq)]
        struct QSystemSerVec {
            name: String,
            path: String,
            dimensions: Vec<(String, String)>,
        }

        impl Into<QSystemSerVec> for &QSystemSer {
            fn into(self) -> QSystemSerVec {
                QSystemSerVec {
                    name: self.get_name_raw(),
                    path: self.get_path_raw(),
                    dimensions: {
                        let mut vec = self
                            .dimensions
                            .iter()
                            .map(|(a, b)| (a.clone(), b.clone()))
                            .collect_vec();
                        vec.sort();
                        vec
                    },
                }
            }
        }

        let CURRENT_SYSTEMS: Vec<QSystemSerVec> = vec![QSystemSerVec {
            name: "si_extended".to_string(),
            path: "data/si_extended".to_string(),
            dimensions: {
                let mut vec = vec![
                    ("L".to_string(), "length".to_string()),
                    ("M".to_string(), "mass".to_string()),
                    ("T".to_string(), "time".to_string()),
                    ("I".to_string(), "electrical current".to_string()),
                    ("Θ".to_string(), "thermodynamic temprature".to_string()),
                    ("N".to_string(), "amount of substance".to_string()),
                    ("J".to_string(), "luminous intensity".to_string()),
                    ("A".to_string(), "Angle".to_string()),
                    ("ΔΘ".to_string(), "temperature interval".to_string()),
                    ("INFO".to_string(), "Information".to_string()),
                ];
                vec.sort();
                vec
            },
        }];
        let systems = code_gen::systems::get_systems()
            .iter()
            .map(|system| -> QSystemSerVec { system.into() })
            .collect_vec();

        assert_eq!(systems, CURRENT_SYSTEMS);
        // panic!(
        //     "failed successfully \n {:?} \n compare to \n {:?}",
        //     systems, CURRENT_SYSTEMS
        // );
    }

    #[test]
    fn expand_dims() {
        let systems = &code_gen::systems::get_systems();
        let dims = code_gen::dimension::generate_dim_types(systems);
        let expected = "# [derive (PartialEq , Eq , Display , SelfRustTokenize , Clone , Copy , FromStr)] pub enum System { SiExtended } # [derive (PartialEq , Eq , Display , SelfRustTokenize , Clone , Copy , Neg , Mul , Div)] # [display (\"{}\")] pub enum SystemDim { SiExtended (SiExtended) }";

        assert_eq!(dims.to_string(), expected);
        //panic!("{}", dims.to_string())
    }

    #[test]
    fn parse_prefixes() {
        let pref = PrefixSer {
            alias: Some("alias".to_owned()),
            factor: code_gen::FactorSer::Ratio("10^30/1".to_owned()),
        };
        let toml = toml::ser::to_string(&pref).unwrap();
        panic!("serialized: \n{}", toml);

        let systems = code_gen::systems::get_systems();
        let prefixes =
            code_gen::prefix::parse_prefixes(Path::new(&systems.first().unwrap().get_path()));
        panic!("{:?} \n {:?}", systems, prefixes);
    }
}
