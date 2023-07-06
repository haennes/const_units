use crate::{
    global_types::{factor::Factor::Ratio, factor::RatioConst, prefix::Prefix},
    parsing::{FactorSer, PrefixSer, PrefixSerSer, QSystemSer, QuantitySer},
};
use hashmap_macro::hashmap;
use proc_macro2::TokenStream;
use quote::quote;
use std::{collections::HashMap, path::Path, string::String};

#[allow(non_snake_case)]
pub(crate) fn CURRENT_SYSTEMS() -> Vec<QSystemSer> {
    vec![QSystemSer::new("si_extended", "data/si_extended", {
        hashmap![
            "L" => "length",
            "M" => "mass",
            "T" => "time",
            "I" => "electrical current",
            "Θ" => "thermodynamic temprature",
            "N" => "amount of substance",
            "J" => "luminous intensity",
            "A" => "Angle",
            "ΔΘ" => "temperature interval",
            "INFO" => "Information",
        ]
    })]
}

#[allow(non_snake_case)]
pub(crate) fn EXPECTED_DIMS() -> TokenStream {
    quote!(
        #[derive(PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, FromStr, ConstParamTy)]
        pub enum System {
            SiExtended,
        }
        #[derive(
            PartialEq, Eq, Display, SelfRustTokenize, Clone, Copy, Neg, Mul, Div, ConstParamTy,
        )]
        #[display("{}".to_string())]
        pub enum SystemDim {
            SiExtended(SiExtended),
        }
    )
}

#[allow(non_snake_case)]
pub(crate) fn EXPECTED_QUANTITIES_SI() -> TokenStream {
    quote!(
        # [doc = "time"] pub const time : "si_extended" = SiExtended { T : 1i8 , } # [doc = "measure of distance"] pub const length : "si_extended" = SiExtended { L : 1i8 , }
    )
}

#[allow(non_snake_case)]
pub(crate) fn EXPECTED_QUANTITIES() -> Vec<QuantitySer> {
    vec![
        QuantitySer::new(
            "time",
            Path::new("time").to_path_buf(),
            Some("time"),
            "second",
            { hashmap!("T" => 1) },
        ),
        QuantitySer::new(
            "length",
            Path::new("length").to_path_buf(),
            Some("measure of distance"),
            "meter",
            { hashmap!("L" => 1) },
        ),
    ]
}

#[allow(non_snake_case)]
pub(crate) fn EXPECTED_PREFIXES() -> Vec<(String, Vec<PrefixSer>)> {
    vec![
        (
            "binary".to_string(),
            vec![
                PrefixSer::new("exbi", None::<&str>, FactorSer::new("1024^6/1").into()),
                PrefixSer::new("gibi", None::<&str>, FactorSer::new("1024^3/1").into()),
                PrefixSer::new("kibi", None::<&str>, FactorSer::new("1024^1/1").into()),
                PrefixSer::new("mebi", None::<&str>, FactorSer::new("1024^2/1").into()),
                PrefixSer::new("pebi", None::<&str>, FactorSer::new("1024^5/1").into()),
                PrefixSer::new("tebi", None::<&str>, FactorSer::new("1024^4/1").into()),
                PrefixSer::new("yobi", None::<&str>, FactorSer::new("1024^8/1").into()),
                PrefixSer::new("zebi", None::<&str>, FactorSer::new("1024^7/1").into()),
            ],
        ),
        (
            "metric".to_string(),
            vec![
                PrefixSer::new("atto", Some("a"), FactorSer::new("1/10^18").into()),
                PrefixSer::new("centi", Some("c"), FactorSer::new("1/10^2").into()),
                PrefixSer::new("deca", Some("da"), FactorSer::new("10^1/1").into()),
                PrefixSer::new("deci", Some("d"), FactorSer::new("1/10^1").into()),
                PrefixSer::new("exa", Some("E"), FactorSer::new("10^18/1").into()),
                PrefixSer::new("femto", Some("f"), FactorSer::new("1/10^15").into()),
                PrefixSer::new("giga", Some("G"), FactorSer::new("10^9/1").into()),
                PrefixSer::new("hecto", Some("h"), FactorSer::new("10^2/1").into()),
                PrefixSer::new("kilo", Some("k"), FactorSer::new("10^3/1").into()),
                PrefixSer::new("mega", Some("M"), FactorSer::new("10^6/1").into()),
                PrefixSer::new("micro", Some("μ"), FactorSer::new("1/10^6").into()),
                PrefixSer::new("milli", Some("m"), FactorSer::new("1/10^3").into()),
                PrefixSer::new("nano", Some("n"), FactorSer::new("1/10^9").into()),
                PrefixSer::new("peta", Some("P"), FactorSer::new("10^15/1").into()),
                PrefixSer::new("pico", Some("p"), FactorSer::new("1/10^12").into()),
                PrefixSer::new("quecto", Some("q"), FactorSer::new("1/10^30").into()),
                PrefixSer::new("quetta", Some("Q"), FactorSer::new("10^30/1").into()),
                PrefixSer::new("ronna", Some("R"), FactorSer::new("10^27/1").into()),
                PrefixSer::new("ronto", Some("r"), FactorSer::new("1/10^27").into()),
                PrefixSer::new("tera", Some("T"), FactorSer::new("10^12/1").into()),
                PrefixSer::new("yocto", Some("y"), FactorSer::new("1/10^24").into()),
                PrefixSer::new("yotta", Some("Y"), FactorSer::new("10^24/1").into()),
                PrefixSer::new("zepto", Some("z"), FactorSer::new("1/10^21").into()),
                PrefixSer::new("zetta", Some("Z"), FactorSer::new("10^21/1").into()),
            ],
        ),
    ]
}
