use crate::parsing::PrefixSer;
use const_units_global_types::Factor;
use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use self_rust_tokenize::SelfRustTokenize;
use syn::Ident;

pub(crate) fn generate_p_name_enum(prefixes: Vec<PrefixSer>) -> TokenStream {
    let names = prefixes.iter().map(|prefix| -> syn::Ident {
        let name = (*prefix).name().to_case(Case::UpperCamel);
        syn::parse_str(&name).expect(&format!("parsing {} to a Ident failed", name))
    });
    quote!(

        #[derive(Debug, Clone, Copy, core::marker::ConstParamTy, PartialEq, Eq, self_rust_tokenize::SelfRustTokenize, parse_display::Display)]
        pub enum PName{
            //HACK
            None,
            Unknown,
            #(#names),*
        }
    )
}

pub(crate) fn generate_p_name_enum_from_str(prefixes: Vec<PrefixSer>) -> TokenStream {
    fn generate_match_item(name: &String, to: &String) -> TokenStream {
        let to: Ident = syn::parse_str(&to.to_case(Case::UpperCamel))
            .expect(&format!("failed to parse {} to an ident", to));
        quote!( #name => {
            return ::core::result::Result::Ok(Self::#to);
        })
    }

    let match_items = prefixes.iter().map(|prefix| {
        let name = prefix.name();
        let first = generate_match_item(name, name);
        match prefix.alias() {
            Some(alias) => {
                let second = generate_match_item(alias, name);
                quote!(
                    #first,

                    #second
                )
            }
            None => quote!(#first),
        }
    });
    quote!(
        impl ::core::str::FromStr for PName {
            type Err = ::parse_display::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    #(#match_items),*
                    _ => {}
                }
                ::core::result::Result::Err(::parse_display::ParseError::new())
            }
        }
    )
}

pub(crate) fn generate_factor_into_p_name(prefixes: Vec<PrefixSer>) -> TokenStream {
    let (ratios, floats): (Vec<_>, Vec<_>) = prefixes.iter().partition_map(|prefix| {
        let name: Ident = syn::parse_str(&prefix.name().to_case(Case::UpperCamel))
            .expect(&format!("failed to parse {} to an Ident", prefix.name()));
        let alias: Option<Ident> = prefix.alias().as_ref().map(|alias| {
            syn::parse_str(&alias).expect(&format!("failed to parse {} to an Ident", alias))
        });
        match prefix.factor() {
            Factor::Ratio(ratio) => either::Either::Left((name, alias, ratio)),
            Factor::Float(float) => either::Either::Right((name, alias, float)),
        }
    });
    let default_unknown = [quote!(_ => PName::Unknown)];
    let variants_float = floats
        .iter()
        .map(|(name, alias, float)| {
            let float = float.to_tokens();
            quote!(#float => PName::#name)
        })
        .chain(default_unknown.iter().cloned());
    let variants_ratio = ratios
        .iter()
        .map(|(name, alias, ratio)| {
            let (num, denom) = (ratio.numerator(), ratio.denominator());
            quote!(const {crate::RatioConst::new_raw(#num, #denom)} => PName::#name)
        })
        .chain(default_unknown.iter().cloned());
    quote!(
        //HACK Should be only PName
        impl PName {
            pub const fn from_factor(factor: Factor) -> PName {
                match factor {
                    Factor::Ratio(ratio) => match ratio {
                        const {RatioConst::new_raw(1,1)}=> PName::None,
                        #(#variants_ratio),*
                    },
                    Factor::Float(float) => match float {
                        const {const_traits::Into::into(1.0)} => PName::None,
                        #(#variants_float),*
                    },
                }
            }
        }
    )
}

pub(crate) fn generate_prefix_from_name(prefixes: Vec<PrefixSer>) -> TokenStream {
    let items = prefixes.iter().map(|prefix| {
        let ident: syn::Ident = syn::parse_str(&prefix.name().to_case(Case::UpperCamel)).unwrap();
        let ratio = prefix.factor().to_tokens();
        //TODO also determine the alias
        quote!(PName::#ident => Self{
            name: PName::#ident,
            alias: PName::Unknown,
            factor: #ratio
        })
    });

    quote!(
        //cannot impl const Into / From
        impl crate::Prefix{
            pub const fn from(name: crate::PName) -> Self{
                match name{
                    PName::None => Prefix{name: PName::None, alias: PName::Unknown, factor: Factor::Ratio(RatioConst::new_raw(1,1))},
                    PName::Unknown => Prefix{name: PName::None, alias: PName::Unknown, factor: Factor::Ratio(RatioConst::new_raw(1, 1))},
                    #(#items),*
                }
            }
        }
    )
}
