use std::fmt::format;

use crate::{
    global_types::factor::{Factor, RatioConst, F64},
    parsing::PrefixSer,
};
use convert_case::Casing;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use self_rust_tokenize::SelfRustTokenize;
use syn::Ident;

pub(crate) fn generate_p_name_enum(prefixes: Vec<PrefixSer>) -> TokenStream {
    let names = prefixes.iter().map(|prefix| -> syn::Ident {
        let name = (*prefix).name();
        syn::parse_str(&name).expect(&format!("parsing {} to a Ident failed", name))
    });
    quote!(

        #[derive(Debug, Clone, ConstParamTy, PartialEq, Eq, SelfRustTokenize, Display)]
        enum PName{
            //HACK
            Unkonw,
            #(#names),*
        }
    )
}

pub(crate) fn generate_p_name_enum_from_str(prefixes: Vec<PrefixSer>) -> TokenStream {
    fn generate_match_item(name: &String, to: &String) -> TokenStream {
        let to: Ident = syn::parse_str(to).expect(&format!("failed to parse {} to an ident", to));
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

pub(crate) fn generate_p_name_p_name_from_factor(prefixes: Vec<PrefixSer>) -> TokenStream {
    let (ratios, floats): (Vec<_>, Vec<_>) = prefixes.iter().partition_map(|prefix| {
        let name: Ident = syn::parse_str(prefix.name())
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
            match alias {
                Some(alias) => {
                    quote!(#float => (PName::#name, PName::#alias))
                }
                // HACK
                None => quote!(#float => (PName::#name, PName::Unknown)),
            }
        })
        .chain(default_unknown.iter().cloned());
    let variants_ratio = ratios
        .iter()
        .map(|(name, alias, ratio)| {
            let ratio = ratio.to_tokens();
            match alias {
                Some(alias) => {
                    quote!(#ratio => (PName::#name, PName::#alias))
                }
                // HACK
                None => quote!(#ratio => (PName::#name, PName::Unknown)),
            }
        })
        .chain(default_unknown.iter().cloned());
    quote!(
        //HACK Should be "for (Option<PName>, Option<PName>)"
        /// (name, alias)
        impl From<Factor> for (PName, PName) {
            fn from(value: Factor) -> Self {
                match value {
                    Factor::Ratio(ratio) => match ratio {
                        #(#variants_ratio),*
                    },
                    Factor::Float(float) => match float {
                        #(#variants_float),*
                    },
                }
            }
        }
    )
}
