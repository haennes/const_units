
#![feature(prelude_import)]
#![allow(unused)]
#![warn(unused_imports)]
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
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod code_gen {
    pub(crate) mod get_name_from_dimensions {
        use super::generate_dimension_struct;
        use crate::parsing::QuantitySer;
        use crate::{global_types::QName, parsing::QSystemSer};
        use convert_case::{Case, Casing};
        use proc_macro2::TokenStream;
        use quote::quote;
        use self_rust_tokenize::SelfRustTokenize;
        use std::{collections::HashMap, str::FromStr};
        pub(crate) fn generate_get_name_from_dimensions_and_op(
            systems: Vec<QSystemSer>,
            quantities: Vec<QuantitySer>,
        ) -> TokenStream {
            let variants = systems
                .iter()
                .map(|system| generate_system(system, &quantities))
                .intersperse({
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_comma(&mut _s);
                    _s
                });
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "use");
                ::quote::__private::push_ident(&mut _s, "crate");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "global_types");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "DimType");
                        ::quote::__private::push_comma(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Operation");
                        ::quote::__private::push_comma(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "QName");
                        ::quote::__private::push_comma(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "SystemDim");
                        _s
                    },
                );
                ::quote::__private::push_semi(&mut _s);
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "crate");
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "fn");
                ::quote::__private::push_ident(
                    &mut _s,
                    "get_name_from_dimensions_and_op",
                );
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "result");
                        ::quote::__private::push_colon(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "SystemDim");
                        ::quote::__private::push_comma(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "operation");
                        ::quote::__private::push_colon(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Operation");
                        ::quote::__private::push_comma(&mut _s);
                        _s
                    },
                );
                ::quote::__private::push_rarrow(&mut _s);
                ::quote::__private::push_ident(&mut _s, "QName");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "match");
                        ::quote::__private::push_ident(&mut _s, "result");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                {
                                    use ::quote::__private::ext::*;
                                    let mut _i = 0usize;
                                    let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                    #[allow(unused_mut)]
                                    let (mut variants, i) = variants.quote_into_iter();
                                    let has_iter = has_iter | i;
                                    let _: ::quote::__private::HasIterator = has_iter;
                                    while true {
                                        let variants = match variants.next() {
                                            Some(_x) => ::quote::__private::RepInterp(_x),
                                            None => break,
                                        };
                                        if _i > 0 {
                                            ::quote::__private::push_comma(&mut _s);
                                        }
                                        _i += 1;
                                        ::quote::ToTokens::to_tokens(&variants, &mut _s);
                                    }
                                }
                                _s
                            },
                        );
                        _s
                    },
                );
                _s
            }
        }
        fn generate_system(
            system: &QSystemSer,
            quantities: &Vec<QuantitySer>,
        ) -> TokenStream {
            let q_dimensions = quantities
                .iter()
                .map(|quantity| {
                    let dimension_struct = generate_dimension_struct(
                        &system.get_name(),
                        &quantity.dimension,
                    );
                    let quantity_name_option = get_name_from_dimensions_system_op(
                        &quantities,
                        &quantity.dimension,
                        system,
                    );
                    let quantity_name_tokens = match quantity_name_option {
                        Some(q_name) => {
                            let q_name = q_name.to_tokens();
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::ToTokens::to_tokens(&q_name, &mut _s);
                                _s
                            }
                        }
                        None => {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "QName");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "None");
                            _s
                        }
                    };
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "const");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::ToTokens::to_tokens(&dimension_struct, &mut _s);
                                _s
                            },
                        );
                        ::quote::__private::push_fat_arrow(&mut _s);
                        ::quote::ToTokens::to_tokens(&quantity_name_tokens, &mut _s);
                        _s
                    }
                });
            let systemname = system.get_name();
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::ToTokens::to_tokens(&systemname, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "dimensions");
                        _s
                    },
                );
                ::quote::__private::push_fat_arrow(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "match");
                        ::quote::__private::push_ident(&mut _s, "dimensions");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                {
                                    use ::quote::__private::ext::*;
                                    let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                    #[allow(unused_mut)]
                                    let (mut q_dimensions, i) = q_dimensions.quote_into_iter();
                                    let has_iter = has_iter | i;
                                    let _: ::quote::__private::HasIterator = has_iter;
                                    while true {
                                        let q_dimensions = match q_dimensions.next() {
                                            Some(_x) => ::quote::__private::RepInterp(_x),
                                            None => break,
                                        };
                                        ::quote::ToTokens::to_tokens(&q_dimensions, &mut _s);
                                    }
                                }
                                _s
                            },
                        );
                        _s
                    },
                );
                _s
            }
        }
        fn get_name_from_dimensions_system_op(
            quantities: &Vec<QuantitySer>,
            dimension: &HashMap<String, i8>,
            system: &QSystemSer,
        ) -> Option<QName> {
            let mut candidates = quantities
                .iter()
                .filter(|quantity| quantity.dimension == *dimension);
            match candidates.next() {
                None => None,
                Some(first) => {
                    match candidates.next() {
                        None => {
                            QName::from_str(&first.name.to_case(Case::UpperCamel)).ok()
                        }
                        Some(_second) => None,
                    }
                }
            }
        }
    }
    pub(crate) mod prefix {
        use crate::{global_types::factor::Factor, parsing::{PrefixPars, PrefixSer}};
        use convert_case::Casing;
        use itertools::Itertools;
        use proc_macro2::TokenStream;
        use quote::quote;
        pub(crate) fn generate_p_name_enum(prefixes: Vec<PrefixSer>) -> TokenStream {
            let prefixes = get_p_names_w_aliases(prefixes);
            let names = prefixes
                .iter()
                .map(|prefix| -> syn::Ident {
                    let name = prefix.name.to_case(convert_case::Case::UpperCamel);
                    syn::parse_str(&name)
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("parsing {0} to a Ident failed", name),
                                );
                                res
                            },
                        )
                });
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_pound(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "derive");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "Debug");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Clone");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "ConstParamTy");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "PartialEq");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Eq");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "FromStr");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "SelfRustTokenize");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Display");
                                _s
                            },
                        );
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "enum");
                ::quote::__private::push_ident(&mut _s, "PName");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut names, i) = names.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let names = match names.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&names, &mut _s);
                            }
                        }
                        _s
                    },
                );
                _s
            }
        }
        pub(crate) fn generate_p_name_enum_eq() {}
        fn get_p_names_w_aliases(prefixes: Vec<PrefixSer>) -> Vec<PrefixPars> {
            prefixes
                .iter()
                .map(|prefix| prefix.clone().into_iter())
                .flatten()
                .collect_vec()
        }
    }
    pub(crate) mod quantities {
        use convert_case::{Case, Casing};
        use proc_macro2::TokenStream;
        use quote::quote;
        use std::{collections::HashMap, str::FromStr};
        use crate::parsing::QuantitySer;
        pub(crate) fn generate_quantities(
            input: Vec<QuantitySer>,
            systemname: String,
        ) -> TokenStream {
            let all = input.iter().map(|q| generate_quantity(&systemname, q));
            TokenStream::from_iter(all)
        }
        fn generate_quantity(
            systemname: &String,
            quantity: &QuantitySer,
        ) -> TokenStream {
            let name: syn::Expr = syn::parse_str(&quantity.name)
                .expect("failed to parse quantity name");
            let systemname_expr: syn::Expr = syn::parse_str(
                    &systemname.to_case(Case::UpperCamel),
                )
                .expect("failed to parse systemname");
            let dim_struct = generate_dimension_struct(&systemname, &quantity.dimension);
            match &quantity.description {
                Some(description) => {
                    TokenStream::from_str(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "///{0}\n            {1}", description, { let mut _s =
                                        ::quote::__private::TokenStream::new();
                                        ::quote::__private::push_ident(& mut _s, "pub");
                                        ::quote::__private::push_ident(& mut _s, "const");
                                        ::quote::ToTokens::to_tokens(& name, & mut _s);
                                        ::quote::__private::push_colon(& mut _s);
                                        ::quote::ToTokens::to_tokens(& systemname_expr, & mut _s);
                                        ::quote::__private::push_eq(& mut _s);
                                        ::quote::ToTokens::to_tokens(& dim_struct, & mut _s); _s }
                                    ),
                                );
                                res
                            },
                        )
                        .expect("error")
                }
                None => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "pub");
                    ::quote::__private::push_ident(&mut _s, "const");
                    ::quote::ToTokens::to_tokens(&name, &mut _s);
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::ToTokens::to_tokens(&systemname_expr, &mut _s);
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::ToTokens::to_tokens(&dim_struct, &mut _s);
                    _s
                }
            }
        }
        pub(crate) fn generate_dimension_fields(
            dimension: &HashMap<String, i8>,
        ) -> Vec<TokenStream> {
            dimension
                .iter()
                .map(|(dimension_name, power)| {
                    let dimension_name: syn::Expr = syn::parse_str(&dimension_name)
                        .expect("parsing failed");
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::ToTokens::to_tokens(&dimension_name, &mut _s);
                        ::quote::__private::push_colon(&mut _s);
                        ::quote::ToTokens::to_tokens(&power, &mut _s);
                        ::quote::__private::push_comma(&mut _s);
                        _s
                    }
                })
                .collect()
        }
        pub(crate) fn generate_dimension_struct(
            systemname: &String,
            dimension: &HashMap<String, i8>,
        ) -> TokenStream {
            let systemname: syn::Expr = syn::parse_str(
                    &systemname.to_case(Case::UpperCamel),
                )
                .expect("failed to parse systemname");
            let fields = generate_dimension_fields(dimension);
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::ToTokens::to_tokens(&systemname, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut fields, i) = fields.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let fields = match fields.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&fields, &mut _s);
                            }
                        }
                        _s
                    },
                );
                _s
            }
        }
    }
    pub(crate) mod systems {
        use std::path::Path;
        use convert_case::{Case, Casing};
        use proc_macro2::TokenStream;
        use quote::quote;
        use crate::parsing::{parse_quantities, Dimensions, QSystemSer};
        use super::generate_quantities;
        const CODE_PATH: &str = "code";
        pub(crate) fn generate_systems_base(systems: Vec<QSystemSer>) -> TokenStream {
            let dimensions_code = generate_sys_dim_types(&systems);
            let dim_structs_code = systems
                .iter()
                .map(|system| generate_system_dim_struct(
                    system.get_name(),
                    system.dimensions.clone(),
                ));
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "use");
                ::quote::__private::push_ident(&mut _s, "super");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_star(&mut _s);
                ::quote::__private::push_semi(&mut _s);
                ::quote::ToTokens::to_tokens(&dimensions_code, &mut _s);
                {
                    use ::quote::__private::ext::*;
                    let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                    #[allow(unused_mut)]
                    let (mut dim_structs_code, i) = dim_structs_code.quote_into_iter();
                    let has_iter = has_iter | i;
                    let _: ::quote::__private::HasIterator = has_iter;
                    while true {
                        let dim_structs_code = match dim_structs_code.next() {
                            Some(_x) => ::quote::__private::RepInterp(_x),
                            None => break,
                        };
                        ::quote::ToTokens::to_tokens(&dim_structs_code, &mut _s);
                    }
                }
                _s
            }
        }
        fn generate_system(system: &QSystemSer) -> TokenStream {
            let dimensions = generate_system_dim_struct(
                system.get_name(),
                system.dimensions.clone(),
            );
            let quantities = generate_quantities(
                parse_quantities(Path::new(&system.get_path())),
                system.get_name(),
            );
            ::core::panicking::panic("not yet implemented")
        }
        /// generates the systems corresponding Dimensions and NONE and implements Default
        fn generate_system_dim_struct(
            systemname: String,
            dimension: Dimensions,
        ) -> TokenStream {
            let fields_struct = dimension
                .iter()
                .map(|(name, description)| {
                    let name: syn::Ident = syn::parse_str(name)
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("parsing {0} to Ident failed", name),
                                );
                                res
                            },
                        );
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_pound(&mut _s);
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Bracket,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "doc");
                                ::quote::__private::push_eq(&mut _s);
                                ::quote::ToTokens::to_tokens(&description, &mut _s);
                                _s
                            },
                        );
                        ::quote::ToTokens::to_tokens(&name, &mut _s);
                        ::quote::__private::push_colon(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "i8");
                        _s
                    }
                });
            let fields_names = dimension
                .iter()
                .map(|(name, _)| -> syn::Ident {
                    syn::parse_str(name).expect("parsing failed")
                });
            let systemname: syn::Ident = syn::parse_str(&systemname)
                .expect(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("parsing {0} to Ident failed", systemname),
                        );
                        res
                    },
                );
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "struct");
                ::quote::ToTokens::to_tokens(&systemname, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut fields_struct, i) = fields_struct.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let fields_struct = match fields_struct.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&fields_struct, &mut _s);
                            }
                        }
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "const");
                ::quote::__private::push_ident(&mut _s, "NONE");
                ::quote::__private::push_colon(&mut _s);
                ::quote::ToTokens::to_tokens(&systemname, &mut _s);
                ::quote::__private::push_eq(&mut _s);
                ::quote::ToTokens::to_tokens(&systemname, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut fields_names, i) = fields_names.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let fields_names = match fields_names.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&fields_names, &mut _s);
                                ::quote::__private::push_colon(&mut _s);
                                ::quote::__private::parse(&mut _s, "0");
                            }
                        }
                        _s
                    },
                );
                ::quote::__private::push_semi(&mut _s);
                ::quote::__private::push_ident(&mut _s, "impl");
                ::quote::__private::push_ident(&mut _s, "const");
                ::quote::__private::push_ident(&mut _s, "Default");
                ::quote::__private::push_ident(&mut _s, "for");
                ::quote::ToTokens::to_tokens(&systemname, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "fn");
                        ::quote::__private::push_ident(&mut _s, "default");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            ::quote::__private::TokenStream::new(),
                        );
                        ::quote::__private::push_rarrow(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Self");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "NONE");
                                _s
                            },
                        );
                        _s
                    },
                );
                _s
            }
        }
        /// generates all the Dimension-related types that are generic over the systems
        fn generate_sys_dim_types(systems: &Vec<QSystemSer>) -> TokenStream {
            let names = systems
                .iter()
                .map(|system| -> syn::Ident {
                    syn::parse_str(&system.get_name().to_case(Case::UpperCamel))
                        .expect("parsing failed")
                });
            let names_clone = names.clone();
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_pound(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "derive");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "PartialEq");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Eq");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Display");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "SelfRustTokenize");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Clone");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Copy");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "FromStr");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "ConstParamTy");
                                _s
                            },
                        );
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "enum");
                ::quote::__private::push_ident(&mut _s, "System");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut names, i) = names.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let names = match names.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                ::quote::ToTokens::to_tokens(&names, &mut _s);
                            }
                        }
                        _s
                    },
                );
                ::quote::__private::push_pound(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "derive");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "PartialEq");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Eq");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Display");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "SelfRustTokenize");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Clone");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Copy");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Neg");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Mul");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Div");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "ConstParamTy");
                                _s
                            },
                        );
                        _s
                    },
                );
                ::quote::__private::push_pound(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "display");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::parse(&mut _s, "\"{}\"");
                                _s
                            },
                        );
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "enum");
                ::quote::__private::push_ident(&mut _s, "SystemDim");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut names_clone, i) = names_clone.quote_into_iter();
                            let has_iter = has_iter | i;
                            #[allow(unused_mut)]
                            let (mut names_clone, i) = names_clone.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let names_clone = match names_clone.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                let names_clone = match names_clone.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                ::quote::ToTokens::to_tokens(&names_clone, &mut _s);
                                ::quote::__private::push_group(
                                    &mut _s,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        ::quote::ToTokens::to_tokens(&names_clone, &mut _s);
                                        _s
                                    },
                                );
                            }
                        }
                        _s
                    },
                );
                _s
            }
        }
    }
    pub(crate) mod unit {
        use crate::generated::prefix::PName;
        use crate::global_types::quantity::Quantity;
        use crate::parsing::{UnitNameSer, UnitSer};
        use convert_case::{Case, Casing};
        use proc_macro2::TokenStream;
        use self_rust_tokenize::SelfRustTokenize;
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
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("QName  {0} not found ", q_name),
                                );
                                res
                            },
                        ),
                    dim_type.parse().expect("DimType not found"),
                )
                .to_tokens();
            let prefix = prefix.to_tokens();
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "const");
                ::quote::ToTokens::to_tokens(&singular, &mut _s);
                ::quote::__private::push_colon(&mut _s);
                ::quote::__private::push_ident(&mut _s, "Unit");
                ::quote::__private::push_lt(&mut _s);
                ::quote::ToTokens::to_tokens(&data_type, &mut _s);
                ::quote::__private::push_comma(&mut _s);
                ::quote::ToTokens::to_tokens(&q_const, &mut _s);
                ::quote::__private::push_comma(&mut _s);
                ::quote::ToTokens::to_tokens(&prefix, &mut _s);
                ::quote::__private::push_gt(&mut _s);
                ::quote::__private::push_eq(&mut _s);
                ::quote::__private::push_ident(&mut _s, "Unit");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "new");
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    ::quote::__private::TokenStream::new(),
                );
                ::quote::__private::push_semi(&mut _s);
                _s
            }
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
                                            [
                                                generate_unit_code_symbol(
                                                    unit,
                                                    data_type.clone(),
                                                    dim_type.clone(),
                                                    q_name.clone(),
                                                    prefix.name.clone(),
                                                ),
                                            ]
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
    }
    pub(crate) use quantities::*;
}
mod expected_tests {
    use crate::{
        generated::PName::UNKNOWN,
        global_types::{factor::Factor::Ratio, factor::RatioConst, prefix::Prefix},
        parsing::{FactorSer, PrefixSer, PrefixSerSer, QSystemSerVec, QuantitySer},
    };
    use hashmap_macro::hashmap;
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::{collections::HashMap, string::String};
    #[allow(non_snake_case)]
    pub(crate) fn CURRENT_SYSTEMS() -> Vec<QSystemSerVec> {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                QSystemSerVec {
                    name: "si_extended".to_string(),
                    path: "data/si_extended".to_string(),
                    dimensions: {
                        let mut vec = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ("L".to_string(), "length".to_string()).into(),
                                ("M".to_string(), "mass".to_string()).into(),
                                ("T".to_string(), "time".to_string()).into(),
                                ("I".to_string(), "electrical current".to_string()).into(),
                                ("Θ".to_string(), "thermodynamic temprature".to_string())
                                    .into(),
                                ("N".to_string(), "amount of substance".to_string()).into(),
                                ("J".to_string(), "luminous intensity".to_string()).into(),
                                ("A".to_string(), "Angle".to_string()).into(),
                                ("ΔΘ".to_string(), "temperature interval".to_string())
                                    .into(),
                                ("INFO".to_string(), "Information".to_string()).into(),
                            ]),
                        );
                        vec.sort();
                        vec
                    },
                },
            ]),
        )
    }
    #[allow(non_snake_case)]
    pub(crate) fn EXPECTED_DIMS() -> TokenStream {
        {
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::__private::push_pound(&mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Bracket,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "derive");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "PartialEq");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Eq");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Display");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "SelfRustTokenize");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Clone");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Copy");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "FromStr");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "ConstParamTy");
                            _s
                        },
                    );
                    _s
                },
            );
            ::quote::__private::push_ident(&mut _s, "pub");
            ::quote::__private::push_ident(&mut _s, "enum");
            ::quote::__private::push_ident(&mut _s, "System");
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "SiExtended");
                    ::quote::__private::push_comma(&mut _s);
                    _s
                },
            );
            ::quote::__private::push_pound(&mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Bracket,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "derive");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "PartialEq");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Eq");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Display");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "SelfRustTokenize");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Clone");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Copy");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Neg");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Mul");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Div");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "ConstParamTy");
                            ::quote::__private::push_comma(&mut _s);
                            _s
                        },
                    );
                    _s
                },
            );
            ::quote::__private::push_pound(&mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Bracket,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "display");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::parse(&mut _s, "\"{}\"");
                            ::quote::__private::push_dot(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "to_string");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                ::quote::__private::TokenStream::new(),
                            );
                            _s
                        },
                    );
                    _s
                },
            );
            ::quote::__private::push_ident(&mut _s, "pub");
            ::quote::__private::push_ident(&mut _s, "enum");
            ::quote::__private::push_ident(&mut _s, "SystemDim");
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "SiExtended");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "SiExtended");
                            _s
                        },
                    );
                    ::quote::__private::push_comma(&mut _s);
                    _s
                },
            );
            _s
        }
    }
    #[allow(non_snake_case)]
    pub(crate) fn EXPECTED_QUANTITIES_SI() -> TokenStream {
        {
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::__private::push_pound(&mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Bracket,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "doc");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::__private::parse(&mut _s, "\"time\"");
                    _s
                },
            );
            ::quote::__private::push_ident(&mut _s, "pub");
            ::quote::__private::push_ident(&mut _s, "const");
            ::quote::__private::push_ident(&mut _s, "time");
            ::quote::__private::push_colon(&mut _s);
            ::quote::__private::parse(&mut _s, "\"si_extended\"");
            ::quote::__private::push_eq(&mut _s);
            ::quote::__private::push_ident(&mut _s, "SiExtended");
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "T");
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::parse(&mut _s, "1i8");
                    ::quote::__private::push_comma(&mut _s);
                    _s
                },
            );
            ::quote::__private::push_pound(&mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Bracket,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "doc");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::__private::parse(&mut _s, "\"measure of distance\"");
                    _s
                },
            );
            ::quote::__private::push_ident(&mut _s, "pub");
            ::quote::__private::push_ident(&mut _s, "const");
            ::quote::__private::push_ident(&mut _s, "length");
            ::quote::__private::push_colon(&mut _s);
            ::quote::__private::parse(&mut _s, "\"si_extended\"");
            ::quote::__private::push_eq(&mut _s);
            ::quote::__private::push_ident(&mut _s, "SiExtended");
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "L");
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::parse(&mut _s, "1i8");
                    ::quote::__private::push_comma(&mut _s);
                    _s
                },
            );
            _s
        }
    }
    #[allow(non_snake_case)]
    pub(crate) fn EXPECTED_QUANTITIES() -> Vec<QuantitySer> {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                QuantitySer {
                    name: "time".to_string(),
                    description: Some("time".to_string()).into(),
                    reference_unit: "second".to_string(),
                    dimension: {
                        ::std::collections::HashMap::from([("T".to_string(), 1)])
                    },
                },
                QuantitySer {
                    name: "length".to_string(),
                    description: Some("measure of distance".to_string()).into(),
                    reference_unit: "meter".to_string(),
                    dimension: {
                        ::std::collections::HashMap::from([("L".to_string(), 1)])
                    },
                },
            ]),
        )
    }
    #[allow(non_snake_case)]
    pub(crate) fn EXPECTED_PREFIXES() -> Vec<(String, Vec<PrefixSer>)> {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                (
                    "binary".to_string(),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            PrefixSer {
                                name: "exbi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^6/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "gibi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^3/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "kibi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^1/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "mebi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^2/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "pebi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^5/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "tebi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^4/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "yobi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^8/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "zebi".to_string(),
                                alias: None,
                                factor: FactorSer("1024^7/1".to_string()).into(),
                            },
                        ]),
                    ),
                ),
                (
                    "metric".to_string(),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            PrefixSer {
                                name: "atto".to_string(),
                                alias: Some("a".to_string()).into(),
                                factor: FactorSer("1/10^18".to_string()).into(),
                            },
                            PrefixSer {
                                name: "centi".to_string(),
                                alias: Some("c".to_string()).into(),
                                factor: FactorSer("1/10^2".to_string()).into(),
                            },
                            PrefixSer {
                                name: "deca".to_string(),
                                alias: Some("da".to_string()).into(),
                                factor: FactorSer("10^1/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "deci".to_string(),
                                alias: Some("d".to_string()).into(),
                                factor: FactorSer("1/10^1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "exa".to_string(),
                                alias: Some("E".to_string()).into(),
                                factor: FactorSer("10^18/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "femto".to_string(),
                                alias: Some("f".to_string()).into(),
                                factor: FactorSer("1/10^15".to_string()).into(),
                            },
                            PrefixSer {
                                name: "giga".to_string(),
                                alias: Some("G".to_string()).into(),
                                factor: FactorSer("10^9/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "hecto".to_string(),
                                alias: Some("h".to_string()).into(),
                                factor: FactorSer("10^2/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "kilo".to_string(),
                                alias: Some("k".to_string()).into(),
                                factor: FactorSer("10^3/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "mega".to_string(),
                                alias: Some("M".to_string()).into(),
                                factor: FactorSer("10^6/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "micro".to_string(),
                                alias: Some("μ".to_string()).into(),
                                factor: FactorSer("1/10^6".to_string()).into(),
                            },
                            PrefixSer {
                                name: "milli".to_string(),
                                alias: Some("m".to_string()).into(),
                                factor: FactorSer("1/10^3".to_string()).into(),
                            },
                            PrefixSer {
                                name: "nano".to_string(),
                                alias: Some("n".to_string()).into(),
                                factor: FactorSer("1/10^9".to_string()).into(),
                            },
                            PrefixSer {
                                name: "peta".to_string(),
                                alias: Some("P".to_string()).into(),
                                factor: FactorSer("10^15/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "pico".to_string(),
                                alias: Some("p".to_string()).into(),
                                factor: FactorSer("1/10^12".to_string()).into(),
                            },
                            PrefixSer {
                                name: "quecto".to_string(),
                                alias: Some("q".to_string()).into(),
                                factor: FactorSer("1/10^30".to_string()).into(),
                            },
                            PrefixSer {
                                name: "quetta".to_string(),
                                alias: Some("Q".to_string()).into(),
                                factor: FactorSer("10^30/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "ronna".to_string(),
                                alias: Some("R".to_string()).into(),
                                factor: FactorSer("10^27/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "ronto".to_string(),
                                alias: Some("r".to_string()).into(),
                                factor: FactorSer("1/10^27".to_string()).into(),
                            },
                            PrefixSer {
                                name: "tera".to_string(),
                                alias: Some("T".to_string()).into(),
                                factor: FactorSer("10^12/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "yocto".to_string(),
                                alias: Some("y".to_string()).into(),
                                factor: FactorSer("1/10^24".to_string()).into(),
                            },
                            PrefixSer {
                                name: "yotta".to_string(),
                                alias: Some("Y".to_string()).into(),
                                factor: FactorSer("10^24/1".to_string()).into(),
                            },
                            PrefixSer {
                                name: "zepto".to_string(),
                                alias: Some("z".to_string()).into(),
                                factor: FactorSer("1/10^21".to_string()).into(),
                            },
                            PrefixSer {
                                name: "zetta".to_string(),
                                alias: Some("Z".to_string()).into(),
                                factor: FactorSer("10^21/1".to_string()).into(),
                            },
                        ]),
                    ),
                ),
            ]),
        )
    }
}
mod generated {
    #![allow(unused_imports)]
    pub(crate) mod dim_type {
        use const_units_macros::{Div, Mul, Neg};
        use core::marker::ConstParamTy;
        use parse_display::{Display, FromStr};
        use self_rust_tokenize::SelfRustTokenize;
        use std::ops::{Div, Mul, Neg};
        use crate::global_types::SiExtended;
        pub enum System {
            SiExtended,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for System {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for System {
            #[inline]
            fn eq(&self, other: &System) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for System {}
        #[automatically_derived]
        impl ::core::cmp::Eq for System {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::fmt::Display for System {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    &Self::SiExtended => f.write_fmt(format_args!("{0}", "SiExtended")),
                }
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for System {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                match self {
                    System::SiExtended => {
                        ::self_rust_tokenize::_private::add_unit_constructor_body(
                            token_stream,
                            &["System", "SiExtended"],
                        );
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for System {
            #[inline]
            fn clone(&self) -> System {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for System {}
        #[automatically_derived]
        impl ::core::str::FromStr for System {
            type Err = ::parse_display::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    "SiExtended" => {
                        return ::core::result::Result::Ok(Self::SiExtended);
                    }
                    _ => {}
                }
                ::core::result::Result::Err(::parse_display::ParseError::new())
            }
        }
        #[display("{0}")]
        pub enum SystemDim {
            SiExtended(SiExtended),
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SystemDim {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SystemDim {
            #[inline]
            fn eq(&self, other: &SystemDim) -> bool {
                match (self, other) {
                    (
                        SystemDim::SiExtended(__self_0),
                        SystemDim::SiExtended(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for SystemDim {}
        #[automatically_derived]
        impl ::core::cmp::Eq for SystemDim {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<SiExtended>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Display for SystemDim {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    &Self::SiExtended(ref _value_0) => {
                        f.write_fmt(format_args!("{0}", _value_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for SystemDim {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                match self {
                    SystemDim::SiExtended(ref _0) => {
                        ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                            token_stream,
                            &["SystemDim", "SiExtended"],
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                                ]),
                            ),
                        );
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SystemDim {
            #[inline]
            fn clone(&self) -> SystemDim {
                let _: ::core::clone::AssertParamIsClone<SiExtended>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for SystemDim {}
        impl Neg for SystemDim {
            type Output = Self;
            fn neg(self) -> Self::Output {
                match self {
                    SystemDim::SiExtended(arg0) => SystemDim::SiExtended(-arg0),
                }
            }
        }
        impl Mul for SystemDim {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                match self {
                    SystemDim::SiExtended(a_arg0) => {
                        if let SystemDim::SiExtended(b_arg0) = rhs {
                            SystemDim::SiExtended {
                                0: a_arg0 * b_arg0,
                            }
                        } else {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("can only multiply if the DimType is the same"),
                                );
                            }
                        }
                    }
                }
            }
        }
        impl Div for SystemDim {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                match self {
                    SystemDim::SiExtended(a_arg0) => {
                        if let SystemDim::SiExtended(b_arg0) = rhs {
                            SystemDim::SiExtended {
                                0: a_arg0 / b_arg0,
                            }
                        } else {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("can only multiply if the DimType is the same"),
                                );
                            }
                        }
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::str::FromStr for SystemDim {
            type Err = ::parse_display::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                {
                    let parse_variant = |
                        s: &str,
                    | -> ::core::result::Result<Self, ::parse_display::ParseError> {
                        #[allow(clippy::trivial_regex)]
                        static RE: ::parse_display::helpers::once_cell::sync::Lazy<
                            ::parse_display::helpers::regex::Regex,
                        > = ::parse_display::helpers::once_cell::sync::Lazy::new(|| {
                            ::parse_display::helpers::regex::Regex::new(
                                    "\\A(?P<value_1>(?:[\0-\u{10ffff}]*?))\\z",
                                )
                                .unwrap()
                        });
                        if let Some(c) = RE.captures(&s) {
                            return ::core::result::Result::Ok(
                                Self::SiExtended(
                                    c
                                        .name("value_1")
                                        .map_or("", |m| m.as_str())
                                        .parse()
                                        .map_err(|e| ::parse_display::ParseError::with_message(
                                            "field `0` parse failed.",
                                        ))?,
                                ),
                            );
                        }
                        ::core::result::Result::Err(::parse_display::ParseError::new())
                    };
                    if let ::core::result::Result::Ok(value) = parse_variant(s) {
                        return ::core::result::Result::Ok(value);
                    }
                }
                ::core::result::Result::Err(::parse_display::ParseError::new())
            }
        }
        #[automatically_derived]
        impl ::core::marker::ConstParamTy for SystemDim {}
    }
    pub mod generic_quantities {}
    pub(crate) mod get_name_from_dimensions_and_op {
        use crate::generated::dim_type::SystemDim;
        use crate::global_types::{Operation, QName};
        pub(crate) fn get_name_from_dimensions_and_op(
            result: SystemDim,
            operation: Operation,
        ) -> QName {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub(crate) mod prefix {
        use core::marker::ConstParamTy;
        use parse_display::{Display, FromStr};
        use self_rust_tokenize::SelfRustTokenize;
        pub enum PName {
            UNKNOWN,
            NONE,
            CENTI,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PName {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        PName::UNKNOWN => "UNKNOWN",
                        PName::NONE => "NONE",
                        PName::CENTI => "CENTI",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PName {
            #[inline]
            fn clone(&self) -> PName {
                match self {
                    PName::UNKNOWN => PName::UNKNOWN,
                    PName::NONE => PName::NONE,
                    PName::CENTI => PName::CENTI,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::ConstParamTy for PName {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PName {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PName {
            #[inline]
            fn eq(&self, other: &PName) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for PName {}
        #[automatically_derived]
        impl ::core::cmp::Eq for PName {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::str::FromStr for PName {
            type Err = ::parse_display::ParseError;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    "UNKNOWN" => {
                        return ::core::result::Result::Ok(Self::UNKNOWN);
                    }
                    "NONE" => {
                        return ::core::result::Result::Ok(Self::NONE);
                    }
                    "CENTI" => {
                        return ::core::result::Result::Ok(Self::CENTI);
                    }
                    _ => {}
                }
                ::core::result::Result::Err(::parse_display::ParseError::new())
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for PName {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                match self {
                    PName::UNKNOWN => {
                        ::self_rust_tokenize::_private::add_unit_constructor_body(
                            token_stream,
                            &["PName", "UNKNOWN"],
                        );
                    }
                    PName::NONE => {
                        ::self_rust_tokenize::_private::add_unit_constructor_body(
                            token_stream,
                            &["PName", "NONE"],
                        );
                    }
                    PName::CENTI => {
                        ::self_rust_tokenize::_private::add_unit_constructor_body(
                            token_stream,
                            &["PName", "CENTI"],
                        );
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Display for PName {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    &Self::UNKNOWN => f.write_fmt(format_args!("{0}", "UNKNOWN")),
                    &Self::NONE => f.write_fmt(format_args!("{0}", "NONE")),
                    &Self::CENTI => f.write_fmt(format_args!("{0}", "CENTI")),
                }
            }
        }
    }
    pub(crate) mod quantity_from_name {
        use super::{dim_type::System, SystemDim};
        use crate::global_types::{quantity::Quantity, QName, SiExtended};
        impl Quantity {
            pub const fn from_name(name: QName, dim_type: System) -> Self {
                match dim_type {
                    System::SiExtended => {
                        match name {
                            QName::Velocity => {
                                Self {
                                    name,
                                    dimensions: SystemDim::SiExtended(SiExtended { a: 0 }),
                                }
                            }
                            QName::Length => {
                                Self {
                                    name,
                                    dimensions: SystemDim::SiExtended(SiExtended { a: 0 }),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub(crate) use prefix::*;
    pub(crate) use crate::generated::dim_type::SystemDim;
    pub(crate) use crate::global_types::{Operation, QName};
    pub(crate) use const_units_macros::{Div, Mul, Neg};
    pub(crate) use core::marker::ConstParamTy;
    pub(crate) use parse_display::{Display, FromStr};
    pub(crate) use std::ops::{Div, Mul, Neg};
}
mod global_types {
    use self::prefix::Prefix;
    use self::quantity::QuantityDataTraits;
    use const_units_macros::{AddingMul, Neg, SubbingDiv};
    use parse_display::{Display, FromStr};
    use self_rust_tokenize::SelfRustTokenize;
    use std::marker::ConstParamTy;
    use std::ops::{Div, Mul, Neg};
    pub(crate) mod factor {
        use core::marker::ConstParamTy;
        use core::ops::Mul;
        use num_rational::Ratio as RatioNonConst;
        use num_traits::FromPrimitive;
        use self_rust_tokenize::SelfRustTokenize;
        use std::{num::ParseFloatError, str::FromStr};
        pub struct RatioConst<
            T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens,
        > {
            pub(crate) numerator: T,
            pub(crate) denominator: T,
        }
        #[automatically_derived]
        impl<
            T: ::core::fmt::Debug + Copy + Eq + Ord + num_integer::Integer
                + quote::ToTokens,
        > ::core::fmt::Debug for RatioConst<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RatioConst",
                    "numerator",
                    &self.numerator,
                    "denominator",
                    &&self.denominator,
                )
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::marker::ConstParamTy + Copy + Eq + Ord + num_integer::Integer
                + quote::ToTokens,
        > ::core::marker::ConstParamTy for RatioConst<T> {}
        #[automatically_derived]
        impl<
            T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens,
        > ::core::marker::StructuralPartialEq for RatioConst<T> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::PartialEq + Copy + Eq + Ord + num_integer::Integer
                + quote::ToTokens,
        > ::core::cmp::PartialEq for RatioConst<T> {
            #[inline]
            fn eq(&self, other: &RatioConst<T>) -> bool {
                self.numerator == other.numerator
                    && self.denominator == other.denominator
            }
        }
        #[automatically_derived]
        impl<
            T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens,
        > ::core::marker::StructuralEq for RatioConst<T> {}
        #[automatically_derived]
        impl<
            T: ::core::cmp::Eq + Copy + Eq + Ord + num_integer::Integer + quote::ToTokens,
        > ::core::cmp::Eq for RatioConst<T> {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<T>;
            }
        }
        #[automatically_derived]
        impl<
            T: ::core::marker::Copy + Copy + Eq + Ord + num_integer::Integer
                + quote::ToTokens,
        > ::core::marker::Copy for RatioConst<T> {}
        #[automatically_derived]
        impl<
            T: ::core::clone::Clone + Copy + Eq + Ord + num_integer::Integer
                + quote::ToTokens,
        > ::core::clone::Clone for RatioConst<T> {
            #[inline]
            fn clone(&self) -> RatioConst<T> {
                RatioConst {
                    numerator: ::core::clone::Clone::clone(&self.numerator),
                    denominator: ::core::clone::Clone::clone(&self.denominator),
                }
            }
        }
        #[automatically_derived]
        impl<
            T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens,
        > ::self_rust_tokenize::SelfRustTokenize for RatioConst<T>
        where
            T: ::self_rust_tokenize::SelfRustTokenize,
            T: ::self_rust_tokenize::SelfRustTokenize,
        {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                let RatioConst { numerator: ref _0, denominator: ref _1 } = self;
                ::self_rust_tokenize::_private::add_named_constructor_body(
                    token_stream,
                    &["RatioConst"],
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            (
                                "numerator",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                            ),
                            (
                                "denominator",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_1),
                            ),
                        ]),
                    ),
                );
            }
        }
        impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> RatioConst<T> {
            pub fn new_raw(numerator: T, denominator: T) -> Self {
                Self { numerator, denominator }
            }
            pub fn new_ratio(ratio: RatioNonConst<T>) -> Self {
                let ratio = ratio.reduced();
                Self {
                    numerator: ratio.numer().clone(),
                    denominator: ratio.denom().clone(),
                }
            }
        }
        impl<
            T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens,
        > Into<RatioNonConst<T>> for RatioConst<T> {
            fn into(self) -> RatioNonConst<T> {
                RatioNonConst::new(self.numerator, self.denominator)
            }
        }
        impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Mul
        for RatioConst<T> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let a: RatioNonConst<T> = self.into();
                let b: RatioNonConst<T> = rhs.into();
                Self::new_ratio(a * b)
            }
        }
        impl<T: Copy + Eq + Ord + num_integer::Integer + quote::ToTokens> Mul<F64>
        for RatioConst<T>
        where
            RatioNonConst<T>: FromPrimitive,
        {
            type Output = Self;
            fn mul(self, rhs: F64) -> Self::Output {
                let a: RatioNonConst<T> = self.into();
                let b: f64 = rhs.into();
                RatioConst::new_ratio(a * RatioNonConst::from_f64(b).unwrap())
            }
        }
        pub struct F64([u8; 8]);
        #[automatically_derived]
        impl ::core::marker::ConstParamTy for F64 {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for F64 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for F64 {
            #[inline]
            fn eq(&self, other: &F64) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for F64 {}
        #[automatically_derived]
        impl ::core::cmp::Eq for F64 {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<[u8; 8]>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for F64 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F64", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for F64 {}
        #[automatically_derived]
        impl ::core::clone::Clone for F64 {
            #[inline]
            fn clone(&self) -> F64 {
                let _: ::core::clone::AssertParamIsClone<[u8; 8]>;
                *self
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for F64 {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                let F64(ref _0) = self;
                ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                    token_stream,
                    &["F64"],
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                        ]),
                    ),
                );
            }
        }
        impl FromStr for F64 {
            type Err = ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self {
                    0: f64::from_str(s)?.to_be_bytes(),
                })
            }
        }
        impl Into<f64> for F64 {
            fn into(self) -> f64 {
                f64::from_be_bytes(self.0)
            }
        }
        impl From<f64> for F64 {
            fn from(value: f64) -> Self {
                Self(value.to_be_bytes())
            }
        }
        pub enum Factor {
            Ratio(RatioConst<i128>),
            Float(F64),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Factor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Factor::Ratio(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ratio",
                            &__self_0,
                        )
                    }
                    Factor::Float(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Float",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Factor {}
        #[automatically_derived]
        impl ::core::clone::Clone for Factor {
            #[inline]
            fn clone(&self) -> Factor {
                let _: ::core::clone::AssertParamIsClone<RatioConst<i128>>;
                let _: ::core::clone::AssertParamIsClone<F64>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::ConstParamTy for Factor {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Factor {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Factor {
            #[inline]
            fn eq(&self, other: &Factor) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Factor::Ratio(__self_0), Factor::Ratio(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Factor::Float(__self_0), Factor::Float(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Factor {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Factor {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<RatioConst<i128>>;
                let _: ::core::cmp::AssertParamIsEq<F64>;
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for Factor {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                match self {
                    Factor::Ratio(ref _0) => {
                        ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                            token_stream,
                            &["Factor", "Ratio"],
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                                ]),
                            ),
                        );
                    }
                    Factor::Float(ref _0) => {
                        ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                            token_stream,
                            &["Factor", "Float"],
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                                ]),
                            ),
                        );
                    }
                }
            }
        }
        impl Mul<Factor> for Factor {
            type Output = Self;
            fn mul(self, rhs: Factor) -> Self::Output {
                match self {
                    Factor::Ratio(a) => {
                        match rhs {
                            Factor::Ratio(b) => Factor::Ratio(a * b),
                            Factor::Float(b) => Factor::Ratio(a * b),
                        }
                    }
                    Factor::Float(a) => {
                        match rhs {
                            Factor::Ratio(b) => Factor::Ratio(b * a),
                            Factor::Float(b) => {
                                Factor::Float({
                                    let a: f64 = a.into();
                                    let b: f64 = b.into();
                                    (a * b).into()
                                })
                            }
                        }
                    }
                }
            }
        }
    }
    pub(crate) mod prefix {
        use core::marker::ConstParamTy;
        use core::ops::Mul;
        use std::str::FromStr;
        use self_rust_tokenize::SelfRustTokenize;
        use crate::generated::PName;
        use crate::parsing::{PrefixSer, PrefixSerSer};
        use super::factor::Factor;
        impl From<Option<PName>> for PName {
            fn from(value: Option<PName>) -> Self {
                match value {
                    Some(panme) => panme,
                    None => PName::UNKNOWN,
                }
            }
        }
        /// (name, alias)
        impl From<Factor> for (PName, PName) {
            fn from(value: Factor) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl From<Factor> for Prefix {
            fn from(value: Factor) -> Self {
                let (name, alias) = value.into();
                Self { name, alias, factor: value }
            }
        }
        pub struct Prefix {
            pub name: PName,
            pub alias: PName,
            pub factor: Factor,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Prefix {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Prefix",
                    "name",
                    &self.name,
                    "alias",
                    &self.alias,
                    "factor",
                    &&self.factor,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Prefix {
            #[inline]
            fn clone(&self) -> Prefix {
                Prefix {
                    name: ::core::clone::Clone::clone(&self.name),
                    alias: ::core::clone::Clone::clone(&self.alias),
                    factor: ::core::clone::Clone::clone(&self.factor),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::ConstParamTy for Prefix {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Prefix {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Prefix {
            #[inline]
            fn eq(&self, other: &Prefix) -> bool {
                self.name == other.name && self.alias == other.alias
                    && self.factor == other.factor
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Prefix {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Prefix {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<PName>;
                let _: ::core::cmp::AssertParamIsEq<Factor>;
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for Prefix {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                let Prefix { name: ref _0, alias: ref _1, factor: ref _2 } = self;
                ::self_rust_tokenize::_private::add_named_constructor_body(
                    token_stream,
                    &["Prefix"],
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            (
                                "name",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                            ),
                            (
                                "alias",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_1),
                            ),
                            (
                                "factor",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_2),
                            ),
                        ]),
                    ),
                );
            }
        }
        impl From<PrefixSer> for Prefix {
            fn from(val: PrefixSer) -> Self {
                Self {
                    name: PName::from_str(&val.name)
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("failed to parse {0} to a PName", val.name),
                                );
                                res
                            },
                        ),
                    alias: match val.alias {
                        Some(alias) => {
                            PName::from_str(&alias)
                                .expect(
                                    &{
                                        let res = ::alloc::fmt::format(
                                            format_args!("failed to parse {0} to a PName", alias),
                                        );
                                        res
                                    },
                                )
                        }
                        None => PName::UNKNOWN,
                    },
                    factor: val.factor.into(),
                }
            }
        }
        impl Mul for Prefix {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let factor = self.factor * rhs.factor;
                let (name, alias) = factor.clone().into();
                Prefix { name, alias, factor }
            }
        }
        impl Into<Prefix> for (String, PrefixSerSer) {
            fn into(self) -> Prefix {
                Prefix {
                    name: PName::from_str(&self.0).ok().into(),
                    alias: {
                        match self.1.alias {
                            Some(str) => PName::from_str(&str).ok().into(),
                            None => None.into(),
                        }
                    },
                    factor: self.1.factor.into(),
                }
            }
        }
    }
    pub(crate) mod quantity {
        use std;
        use std::fmt::Display;
        use std::ops::Div;
        use std::ops::Mul;
        use num_traits::NumAssignRef;
        use self_rust_tokenize::SelfRustTokenize;
        use super::Operation;
        use crate::generated::get_name_from_dimensions_and_op::get_name_from_dimensions_and_op;
        use std::ops::Neg;
        use crate::generated::dim_type::SystemDim;
        use core::marker::ConstParamTy;
        use super::QName;
        pub struct Quantity {
            pub(crate) name: QName,
            pub(crate) dimensions: SystemDim,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Quantity {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Quantity {
            #[inline]
            fn eq(&self, other: &Quantity) -> bool {
                self.name == other.name && self.dimensions == other.dimensions
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Quantity {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Quantity {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<QName>;
                let _: ::core::cmp::AssertParamIsEq<SystemDim>;
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for Quantity {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                let Quantity { name: ref _0, dimensions: ref _1 } = self;
                ::self_rust_tokenize::_private::add_named_constructor_body(
                    token_stream,
                    &["Quantity"],
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            (
                                "name",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                            ),
                            (
                                "dimensions",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_1),
                            ),
                        ]),
                    ),
                );
            }
        }
        #[automatically_derived]
        impl ::core::marker::ConstParamTy for Quantity {}
        impl Neg for Quantity {
            type Output = Self;
            fn neg(self) -> Self::Output {
                let result = self.dimensions.neg();
                Self {
                    name: get_name_from_dimensions_and_op(
                        result,
                        Operation::Neg(self.name),
                    ),
                    dimensions: result,
                }
            }
        }
        impl Mul for Quantity {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let result = self.dimensions.mul(rhs.dimensions);
                Self {
                    name: get_name_from_dimensions_and_op(
                        result,
                        Operation::Mul(self.name, rhs.name),
                    ),
                    dimensions: result,
                }
            }
        }
        impl Div for Quantity {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let result = self.dimensions.div(rhs.dimensions);
                Self {
                    name: get_name_from_dimensions_and_op(
                        result,
                        Operation::Div(self.name, rhs.name),
                    ),
                    dimensions: result,
                }
            }
        }
        impl Display for Quantity {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    f.write_fmt(format_args!("{0} ({1})", self.name, self.dimensions))
                } else {
                    f.write_fmt(format_args!("{0}", self.name))
                }
            }
        }
        pub trait QuantityDataTraits: NumAssignRef + One + Copy {}
        impl<DT> QuantityDataTraits for DT
        where
            DT: NumAssignRef + One + Display + Copy,
        {}
        pub trait One {
            const ONE: Self;
        }
    }
    pub(crate) enum Operation {
        Mul(QName, QName),
        Div(QName, QName),
        Neg(QName),
    }
    pub struct SiExtended {
        pub(crate) a: i16,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SiExtended {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SiExtended {
        #[inline]
        fn eq(&self, other: &SiExtended) -> bool {
            self.a == other.a
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SiExtended {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SiExtended {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i16>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display for SiExtended {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            f.write_fmt(format_args!("{0}", self.a))
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for SiExtended {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            let SiExtended { a: ref _0 } = self;
            ::self_rust_tokenize::_private::add_named_constructor_body(
                token_stream,
                &["SiExtended"],
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ("a", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0)),
                    ]),
                ),
            );
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SiExtended {
        #[inline]
        fn clone(&self) -> SiExtended {
            let _: ::core::clone::AssertParamIsClone<i16>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SiExtended {}
    impl Neg for SiExtended {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self { a: -self.a }
        }
    }
    impl Mul for SiExtended {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self { a: self.a + rhs.a }
        }
    }
    impl Div for SiExtended {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            Self { a: self.a - rhs.a }
        }
    }
    #[automatically_derived]
    impl ::core::str::FromStr for SiExtended {
        type Err = ::parse_display::ParseError;
        fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
            #[allow(clippy::trivial_regex)]
            static RE: ::parse_display::helpers::once_cell::sync::Lazy<
                ::parse_display::helpers::regex::Regex,
            > = ::parse_display::helpers::once_cell::sync::Lazy::new(|| {
                ::parse_display::helpers::regex::Regex::new(
                        "\\A(?P<value_1>(?:[\0-\u{10ffff}]*?))\\z",
                    )
                    .unwrap()
            });
            if let Some(c) = RE.captures(&s) {
                return ::core::result::Result::Ok(Self {
                    a: c
                        .name("value_1")
                        .map_or("", |m| m.as_str())
                        .parse()
                        .map_err(|e| ::parse_display::ParseError::with_message(
                            "field `a` parse failed.",
                        ))?,
                });
            }
            ::core::result::Result::Err(::parse_display::ParseError::new())
        }
    }
    #[automatically_derived]
    impl ::core::marker::ConstParamTy for SiExtended {}
    pub enum QName {
        Velocity,
        Length,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for QName {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for QName {
        #[inline]
        fn eq(&self, other: &QName) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for QName {}
    #[automatically_derived]
    impl ::core::cmp::Eq for QName {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::fmt::Display for QName {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                &Self::Velocity => f.write_fmt(format_args!("{0}", "Velocity")),
                &Self::Length => f.write_fmt(format_args!("{0}", "Length")),
            }
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for QName {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            match self {
                QName::Velocity => {
                    ::self_rust_tokenize::_private::add_unit_constructor_body(
                        token_stream,
                        &["QName", "Velocity"],
                    );
                }
                QName::Length => {
                    ::self_rust_tokenize::_private::add_unit_constructor_body(
                        token_stream,
                        &["QName", "Length"],
                    );
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::str::FromStr for QName {
        type Err = ::parse_display::ParseError;
        fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
            match s {
                "Velocity" => {
                    return ::core::result::Result::Ok(Self::Velocity);
                }
                "Length" => {
                    return ::core::result::Result::Ok(Self::Length);
                }
                _ => {}
            }
            ::core::result::Result::Err(::parse_display::ParseError::new())
        }
    }
    #[automatically_derived]
    impl ::core::marker::ConstParamTy for QName {}
    pub struct Unit<
        StorageDt: QuantityDataTraits,
        const QUANTITY: quantity::Quantity,
        const PREFIX: Prefix,
        const INITIALIZED: bool = false,
    > {
        value: StorageDt,
    }
    impl<
        StorageDt: QuantityDataTraits,
        const QUANTITY: quantity::Quantity,
        const PREFIX: Prefix,
    > Unit<StorageDt, QUANTITY, PREFIX, false>
    where
        StorageDt: QuantityDataTraits,
    {
        pub const fn new() -> Self {
            Self {
                value: <StorageDt as quantity::One>::ONE,
            }
        }
    }
    impl<
        StorageDt: QuantityDataTraits,
        const QUANTITY: quantity::Quantity,
        const PREFIX: Prefix,
        const INITIALIZED: bool,
    > Unit<StorageDt, QUANTITY, PREFIX, INITIALIZED>
    where
        StorageDt: QuantityDataTraits,
    {
        /// declared as unsafe as you are leaving the dimension-checking realm
        pub const unsafe fn raw_value(&self) -> StorageDt {
            self.value
        }
    }
}
mod parsing {
    use crate::global_types::{
        factor::{Factor, RatioConst},
        prefix::Prefix,
    };
    use convert_case::{Case, Casing};
    use either::Either;
    use itertools::Itertools;
    use self_rust_tokenize::SelfRustTokenize;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::HashMap, fs::{self, File},
        io::Read, path::Path,
    };
    const DATA_PATH: &str = "data";
    const DIMENSIONS_PATH: &str = "dimensions.toml";
    const PREFIXES_PATH: &str = "prefixes";
    const QUANTITIES_PATH: &str = "quantities";
    const QUANTITIES_FILE_NAME: &str = "quantity.toml";
    pub(crate) struct QuantitySer {
        pub(crate) name: String,
        pub(crate) description: Option<String>,
        pub(crate) reference_unit: String,
        pub(crate) dimension: HashMap<String, i8>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for QuantitySer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "QuantitySer",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "reference_unit",
                    &self.reference_unit,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dimension",
                    &self.dimension,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for QuantitySer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "description" => _serde::__private::Ok(__Field::__field1),
                            "reference_unit" => _serde::__private::Ok(__Field::__field2),
                            "dimension" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"description" => _serde::__private::Ok(__Field::__field1),
                            b"reference_unit" => _serde::__private::Ok(__Field::__field2),
                            b"dimension" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<QuantitySer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = QuantitySer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct QuantitySer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            HashMap<String, i8>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(QuantitySer {
                            name: __field0,
                            description: __field1,
                            reference_unit: __field2,
                            dimension: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            HashMap<String, i8>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "reference_unit",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "dimension",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            HashMap<String, i8>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("description") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "reference_unit",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("dimension") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(QuantitySer {
                            name: __field0,
                            description: __field1,
                            reference_unit: __field2,
                            dimension: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "name",
                    "description",
                    "reference_unit",
                    "dimension",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "QuantitySer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<QuantitySer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for QuantitySer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "QuantitySer",
                "name",
                &self.name,
                "description",
                &self.description,
                "reference_unit",
                &self.reference_unit,
                "dimension",
                &&self.dimension,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for QuantitySer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for QuantitySer {
        #[inline]
        fn eq(&self, other: &QuantitySer) -> bool {
            self.name == other.name && self.description == other.description
                && self.reference_unit == other.reference_unit
                && self.dimension == other.dimension
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for QuantitySer {}
    #[automatically_derived]
    impl ::core::cmp::Eq for QuantitySer {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<HashMap<String, i8>>;
        }
    }
    pub struct PrefixSerSer {
        pub alias: Option<String>,
        pub factor: FactorSer,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PrefixSerSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "PrefixSerSer",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "alias",
                    &self.alias,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "factor",
                    &self.factor,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PrefixSerSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "alias" => _serde::__private::Ok(__Field::__field0),
                            "factor" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"alias" => _serde::__private::Ok(__Field::__field0),
                            b"factor" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PrefixSerSer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PrefixSerSer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct PrefixSerSer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct PrefixSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            FactorSer,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct PrefixSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(PrefixSerSer {
                            alias: __field0,
                            factor: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<FactorSer> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("alias"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("factor"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            FactorSer,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("alias") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("factor") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(PrefixSerSer {
                            alias: __field0,
                            factor: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["alias", "factor"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PrefixSerSer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PrefixSerSer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for PrefixSerSer {
        #[inline]
        fn clone(&self) -> PrefixSerSer {
            PrefixSerSer {
                alias: ::core::clone::Clone::clone(&self.alias),
                factor: ::core::clone::Clone::clone(&self.factor),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PrefixSerSer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "PrefixSerSer",
                "alias",
                &self.alias,
                "factor",
                &&self.factor,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PrefixSerSer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PrefixSerSer {
        #[inline]
        fn eq(&self, other: &PrefixSerSer) -> bool {
            self.alias == other.alias && self.factor == other.factor
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PrefixSerSer {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PrefixSerSer {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<FactorSer>;
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for PrefixSerSer {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            let PrefixSerSer { alias: ref _0, factor: ref _1 } = self;
            ::self_rust_tokenize::_private::add_named_constructor_body(
                token_stream,
                &["PrefixSerSer"],
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ("alias", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0)),
                        ("factor", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_1)),
                    ]),
                ),
            );
        }
    }
    pub struct PrefixSer {
        pub name: String,
        pub alias: Option<String>,
        pub factor: Factor,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PrefixSer {
        #[inline]
        fn clone(&self) -> PrefixSer {
            PrefixSer {
                name: ::core::clone::Clone::clone(&self.name),
                alias: ::core::clone::Clone::clone(&self.alias),
                factor: ::core::clone::Clone::clone(&self.factor),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PrefixSer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "PrefixSer",
                "name",
                &self.name,
                "alias",
                &self.alias,
                "factor",
                &&self.factor,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PrefixSer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PrefixSer {
        #[inline]
        fn eq(&self, other: &PrefixSer) -> bool {
            self.name == other.name && self.alias == other.alias
                && self.factor == other.factor
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PrefixSer {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PrefixSer {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<Factor>;
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for PrefixSer {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            let PrefixSer { name: ref _0, alias: ref _1, factor: ref _2 } = self;
            ::self_rust_tokenize::_private::add_named_constructor_body(
                token_stream,
                &["PrefixSer"],
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ("name", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0)),
                        ("alias", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_1)),
                        ("factor", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_2)),
                    ]),
                ),
            );
        }
    }
    pub struct PrefixPars {
        pub(crate) name: String,
        pub(crate) factor: Factor,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PrefixPars {
        #[inline]
        fn clone(&self) -> PrefixPars {
            PrefixPars {
                name: ::core::clone::Clone::clone(&self.name),
                factor: ::core::clone::Clone::clone(&self.factor),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PrefixPars {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "PrefixPars",
                "name",
                &self.name,
                "factor",
                &&self.factor,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PrefixPars {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PrefixPars {
        #[inline]
        fn eq(&self, other: &PrefixPars) -> bool {
            self.name == other.name && self.factor == other.factor
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PrefixPars {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PrefixPars {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Factor>;
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for PrefixPars {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            let PrefixPars { name: ref _0, factor: ref _1 } = self;
            ::self_rust_tokenize::_private::add_named_constructor_body(
                token_stream,
                &["PrefixPars"],
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ("name", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0)),
                        ("factor", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_1)),
                    ]),
                ),
            );
        }
    }
    impl IntoIterator for PrefixSer {
        type Item = PrefixPars;
        type IntoIter = std::vec::IntoIter<PrefixPars>;
        /// yields PrefiSer´s with the alias field empty instead having a name field
        fn into_iter(self) -> Self::IntoIter {
            self.alias
                .iter()
                .chain([self.name].iter())
                .map(|name| PrefixPars {
                    name: name.clone(),
                    factor: self.factor.into(),
                })
                .collect_vec()
                .into_iter()
        }
    }
    pub(crate) type Dimensions = HashMap<String, String>;
    #[serde(transparent)]
    pub(crate) struct UnitNameSerSer {
        #[serde(with = "either::serde_untagged")]
        inner: Either<String, (String, String)>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UnitNameSerSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                either::serde_untagged::serialize(&self.inner, __serializer)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UnitNameSerSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                _serde::__private::Result::map(
                    either::serde_untagged::deserialize(__deserializer),
                    |__transparent| UnitNameSerSer {
                        inner: __transparent,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for UnitNameSerSer {
        #[inline]
        fn clone(&self) -> UnitNameSerSer {
            UnitNameSerSer {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    impl UnitNameSerSer {
        pub fn new(either: Either<String, (String, String)>) -> Self {
            Self { inner: either }
        }
    }
    pub(crate) struct UnitSerSer {
        pub(crate) symbol: String,
        pub(crate) names: HashMap<String, UnitNameSerSer>,
        pub(crate) derive_prefixes: Option<Vec<String>>,
        pub(crate) conversions: Option<HashMap<String, ConversionSerSer>>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UnitSerSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UnitSerSer",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "symbol",
                    &self.symbol,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "names",
                    &self.names,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "derive_prefixes",
                    &self.derive_prefixes,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "conversions",
                    &self.conversions,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UnitSerSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "symbol" => _serde::__private::Ok(__Field::__field0),
                            "names" => _serde::__private::Ok(__Field::__field1),
                            "derive_prefixes" => _serde::__private::Ok(__Field::__field2),
                            "conversions" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"symbol" => _serde::__private::Ok(__Field::__field0),
                            b"names" => _serde::__private::Ok(__Field::__field1),
                            b"derive_prefixes" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            b"conversions" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UnitSerSer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UnitSerSer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UnitSerSer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            HashMap<String, UnitNameSerSer>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<Vec<String>>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<HashMap<String, ConversionSerSer>>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UnitSerSer {
                            symbol: __field0,
                            names: __field1,
                            derive_prefixes: __field2,
                            conversions: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            HashMap<String, UnitNameSerSer>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<Vec<String>>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            Option<HashMap<String, ConversionSerSer>>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("symbol"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("names"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            HashMap<String, UnitNameSerSer>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "derive_prefixes",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<Vec<String>>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "conversions",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<HashMap<String, ConversionSerSer>>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("symbol") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("names") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "derive_prefixes",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("conversions") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(UnitSerSer {
                            symbol: __field0,
                            names: __field1,
                            derive_prefixes: __field2,
                            conversions: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "symbol",
                    "names",
                    "derive_prefixes",
                    "conversions",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UnitSerSer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UnitSerSer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub(crate) struct UnitSer {
        pub(crate) symbol: String,
        pub(crate) names: HashMap<String, UnitNameSer>,
        pub(crate) prefixes: Vec<Prefix>,
        pub(crate) conversions: HashMap<String, ConversionSer>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnitSer {
        #[inline]
        fn clone(&self) -> UnitSer {
            UnitSer {
                symbol: ::core::clone::Clone::clone(&self.symbol),
                names: ::core::clone::Clone::clone(&self.names),
                prefixes: ::core::clone::Clone::clone(&self.prefixes),
                conversions: ::core::clone::Clone::clone(&self.conversions),
            }
        }
    }
    pub struct FactorSer(pub(crate) String);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for FactorSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "FactorSer",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for FactorSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<FactorSer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = FactorSer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct FactorSer",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: String = match <String as _serde::Deserialize>::deserialize(
                            __e,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(FactorSer(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct FactorSer with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(FactorSer(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "FactorSer",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<FactorSer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for FactorSer {
        #[inline]
        fn clone(&self) -> FactorSer {
            FactorSer(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FactorSer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FactorSer", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FactorSer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FactorSer {
        #[inline]
        fn eq(&self, other: &FactorSer) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for FactorSer {}
    #[automatically_derived]
    impl ::core::cmp::Eq for FactorSer {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for FactorSer {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            let FactorSer(ref _0) = self;
            ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                token_stream,
                &["FactorSer"],
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                    ]),
                ),
            );
        }
    }
    pub(crate) struct ConversionSerSer {
        factor: FactorSer,
        accuracy: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ConversionSerSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ConversionSerSer",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "factor",
                    &self.factor,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "accuracy",
                    &self.accuracy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ConversionSerSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "factor" => _serde::__private::Ok(__Field::__field0),
                            "accuracy" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"factor" => _serde::__private::Ok(__Field::__field0),
                            b"accuracy" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ConversionSerSer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ConversionSerSer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ConversionSerSer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            FactorSer,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ConversionSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ConversionSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ConversionSerSer {
                            factor: __field0,
                            accuracy: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<FactorSer> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("factor"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            FactorSer,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "accuracy",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("factor") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("accuracy") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ConversionSerSer {
                            factor: __field0,
                            accuracy: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["factor", "accuracy"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ConversionSerSer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ConversionSerSer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub(crate) struct ConversionSer {
        factor: Factor,
        accuracy: AccuracySer,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConversionSer {
        #[inline]
        fn clone(&self) -> ConversionSer {
            ConversionSer {
                factor: ::core::clone::Clone::clone(&self.factor),
                accuracy: ::core::clone::Clone::clone(&self.accuracy),
            }
        }
    }
    pub(crate) struct UnitNameSer {
        pub(crate) singular: String,
        pub(crate) plural: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnitNameSer {
        #[inline]
        fn clone(&self) -> UnitNameSer {
            UnitNameSer {
                singular: ::core::clone::Clone::clone(&self.singular),
                plural: ::core::clone::Clone::clone(&self.plural),
            }
        }
    }
    pub(crate) enum AccuracySer {
        Exact,
        Inaccurate(i32),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AccuracySer {
        #[inline]
        fn clone(&self) -> AccuracySer {
            match self {
                AccuracySer::Exact => AccuracySer::Exact,
                AccuracySer::Inaccurate(__self_0) => {
                    AccuracySer::Inaccurate(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    pub(crate) struct QSystemSerVec {
        pub(crate) name: String,
        pub(crate) path: String,
        pub(crate) dimensions: Vec<(String, String)>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for QSystemSerVec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "QSystemSerVec",
                "name",
                &self.name,
                "path",
                &self.path,
                "dimensions",
                &&self.dimensions,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for QSystemSerVec {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for QSystemSerVec {
        #[inline]
        fn eq(&self, other: &QSystemSerVec) -> bool {
            self.name == other.name && self.path == other.path
                && self.dimensions == other.dimensions
        }
    }
    pub(crate) struct QSystemSer {
        name: String,
        path: String,
        pub(crate) dimensions: Dimensions,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for QSystemSer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "QSystemSer",
                "name",
                &self.name,
                "path",
                &self.path,
                "dimensions",
                &&self.dimensions,
            )
        }
    }
    impl QSystemSer {
        pub(crate) fn get_name(&self) -> String {
            self.name.to_case(Case::UpperCamel)
        }
        pub(crate) fn get_name_raw(&self) -> String {
            self.name.clone()
        }
        pub(crate) fn get_path(&self) -> String {
            self.path.clone()
        }
        pub(crate) fn new(name: String, path: String, dimensions: Dimensions) -> Self {
            Self { name, path, dimensions }
        }
    }
    impl IntoIterator for UnitNameSer {
        type Item = String;
        type IntoIter = std::vec::IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            if self.singular == self.plural {
                <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([self.singular]))
                    .into_iter()
            } else {
                <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([self.singular, self.plural]),
                    )
                    .into_iter()
            }
        }
    }
    impl Into<Factor> for FactorSer {
        fn into(self) -> Factor {
            match self.0.split_once("/") {
                Some((num, denom)) => {
                    let (num, denom): (i128, i128) = (
                        parse_to_int_pow(num),
                        parse_to_int_pow(denom),
                    );
                    Factor::Ratio(RatioConst::new_raw(num, denom))
                }
                None => {
                    match self.0.split_once(".") {
                        Some(_) => {
                            Factor::Float(
                                self
                                    .0
                                    .parse()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!("failed to parse {0} to a float", self.0),
                                            );
                                            res
                                        },
                                    ),
                            )
                        }
                        None => {
                            Factor::Ratio(RatioConst::new_raw(parse_to_int(&self.0), 1))
                        }
                    }
                }
            }
        }
    }
    impl From<(String, Option<String>)> for UnitNameSer {
        fn from(value: (String, Option<String>)) -> Self {
            UnitNameSer {
                singular: value.0.clone(),
                plural: {
                    match value.1 {
                        Some(plural) => plural,
                        None => value.0,
                    }
                },
            }
        }
    }
    impl From<UnitNameSerSer> for UnitNameSer {
        fn from(value: UnitNameSerSer) -> Self {
            match value.inner {
                Either::Left(string) => {
                    UnitNameSer {
                        singular: string.clone(),
                        plural: string,
                    }
                }
                Either::Right((singular, plural)) => UnitNameSer { singular, plural },
            }
        }
    }
    impl UnitSer {
        fn new(
            unit: UnitSerSer,
            prefix_map: HashMap<String, Vec<PrefixSer>>,
        ) -> UnitSer {
            UnitSer {
                symbol: unit.symbol,
                names: unit
                    .names
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone().into()))
                    .collect::<HashMap<String, UnitNameSer>>(),
                prefixes: {
                    unit.derive_prefixes
                        .unwrap_or_default()
                        .iter()
                        .map(|group_name| {
                            prefix_map
                                .get(group_name)
                                .expect(
                                    &{
                                        let res = ::alloc::fmt::format(
                                            format_args!("could not find prefix-group {0}", group_name),
                                        );
                                        res
                                    },
                                )
                                .clone()
                        })
                        .flatten()
                        .map(|prefix| prefix.into())
                        .collect_vec()
                },
                conversions: unit
                    .conversions
                    .unwrap_or_default()
                    .iter()
                    .map(|(key, value)| (key.clone(), (value.clone()).into()))
                    .collect::<HashMap<String, ConversionSer>>(),
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
    impl From<String> for UnitNameSer {
        fn from(value: String) -> Self {
            UnitNameSer {
                singular: value.clone(),
                plural: value,
            }
        }
    }
    impl Into<QSystemSerVec> for &QSystemSer {
        fn into(self) -> QSystemSerVec {
            QSystemSerVec {
                name: self.get_name_raw(),
                path: self.get_path(),
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
    pub(crate) fn parse_dimensions(systempath: &Path) -> Dimensions {
        struct DimensionsSer {
            dimensions: HashMap<String, String>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DimensionsSer {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DimensionsSer",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "dimensions",
                        &self.dimensions,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DimensionsSer {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "dimensions" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"dimensions" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<DimensionsSer>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DimensionsSer;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DimensionsSer",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                HashMap<String, String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DimensionsSer with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DimensionsSer {
                                dimensions: __field0,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                HashMap<String, String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "dimensions",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                HashMap<String, String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("dimensions") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DimensionsSer {
                                dimensions: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["dimensions"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DimensionsSer",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<DimensionsSer>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        let path = systempath.join(DIMENSIONS_PATH);
        let contents = fs::read_to_string(path.clone())
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("failed to read file {0}", path.display()),
                    );
                    res
                },
            );
        let dim: DimensionsSer = toml::de::from_str(&contents)
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "failed to parse file {0} \n contents: {1}", path.display(),
                            contents
                        ),
                    );
                    res
                },
            );
        dim.dimensions
    }
    pub(crate) struct PrefixGroup(Vec<PrefixSer>);
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for PrefixGroup {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            let PrefixGroup(ref _0) = self;
            ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                token_stream,
                &["PrefixGroup"],
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                    ]),
                ),
            );
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PrefixGroup {
        #[inline]
        fn clone(&self) -> PrefixGroup {
            PrefixGroup(::core::clone::Clone::clone(&self.0))
        }
    }
    impl IntoIterator for PrefixGroup {
        type Item = PrefixSer;
        type IntoIter = std::vec::IntoIter<PrefixSer>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
    impl From<(String, PrefixSerSer)> for PrefixSer {
        fn from((name, prefix): (String, PrefixSerSer)) -> Self {
            Self {
                name,
                alias: prefix.alias,
                factor: prefix.factor.into(),
            }
        }
    }
    impl From<Vec<(String, PrefixSerSer)>> for PrefixGroup {
        fn from(value: Vec<(String, PrefixSerSer)>) -> Self {
            Self(
                value
                    .iter()
                    .map(|(name_prefix)| name_prefix.clone().into())
                    .collect_vec(),
            )
        }
    }
    pub(crate) fn parse_prefixes(systempath: &Path) -> HashMap<String, PrefixGroup> {
        let folder = systempath.join(PREFIXES_PATH);
        folder
            .read_dir()
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("wrong prefixes-dir: {0}", folder.display()),
                    );
                    res
                },
            )
            .into_iter()
            .filter_map(|file| {
                let path = file
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!("could not read {0}", folder.display()),
                            );
                            res
                        },
                    )
                    .path();
                if path.extension().unwrap_or_default() != "toml" {
                    None
                } else {
                    let mut file = File::open(path.clone())
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("could not open {0}", path.display()),
                                );
                                res
                            },
                        );
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("could not read file {0}", path.display()),
                                );
                                res
                            },
                        );
                    let out = {
                        let contents_clone = contents.clone();
                        toml::de::from_str::<HashMap<String, PrefixSerSer>>(&contents)
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "could not desiralize file {0} \n with these contents \n {1}",
                                            path.display(), contents_clone
                                        ),
                                    );
                                    res
                                },
                            )
                            .into_iter()
                            .collect_vec()
                    };
                    Some((
                        path
                            .file_name()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!("invalid Path {0}", path.display()),
                                    );
                                    res
                                },
                            )
                            .to_str()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "failed to parse {0} to a String", path.display()
                                        ),
                                    );
                                    res
                                },
                            )
                            .split_once(".toml")
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "invalid filename (missing .toml) {0}", path.display()
                                        ),
                                    );
                                    res
                                },
                            )
                            .0
                            .to_string(),
                        out.into(),
                    ))
                }
            })
            .collect()
    }
    pub(crate) fn parse_to_int_pow(num: &str) -> i128 {
        match num.split_once("^") {
            Some((str1, str2)) => {
                if str2 == "" {
                    parse_to_int(str1)
                } else {
                    let base = parse_to_int(str1);
                    let pow = parse_to_int(str2);
                    base.pow(pow as u32)
                }
            }
            None => parse_to_int(num),
        }
    }
    pub(crate) fn parse_to_int(num: &str) -> i128 {
        num.parse()
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("failed to parse {0} to a string", num),
                    );
                    res
                },
            )
    }
    pub(crate) fn parse_quantities(systempath: &Path) -> Vec<QuantitySer> {
        struct QuantitySerSer {
            description: Option<String>,
            reference_unit: String,
            dimension: HashMap<String, i8>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for QuantitySerSer {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "QuantitySerSer",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "reference_unit",
                        &self.reference_unit,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "dimension",
                        &self.dimension,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for QuantitySerSer {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "description" => _serde::__private::Ok(__Field::__field0),
                                "reference_unit" => _serde::__private::Ok(__Field::__field1),
                                "dimension" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"description" => _serde::__private::Ok(__Field::__field0),
                                b"reference_unit" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"dimension" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<QuantitySerSer>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = QuantitySerSer;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct QuantitySerSer",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct QuantitySerSer with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct QuantitySerSer with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                HashMap<String, i8>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct QuantitySerSer with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(QuantitySerSer {
                                description: __field0,
                                reference_unit: __field1,
                                dimension: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                HashMap<String, i8>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "reference_unit",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "dimension",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                HashMap<String, i8>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("description") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "reference_unit",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("dimension") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(QuantitySerSer {
                                description: __field0,
                                reference_unit: __field1,
                                dimension: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "description",
                        "reference_unit",
                        "dimension",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "QuantitySerSer",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<QuantitySerSer>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        let path = systempath.join(QUANTITIES_PATH);
        path.read_dir()
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("failed to open {0}", path.display()),
                    );
                    res
                },
            )
            .into_iter()
            .filter_map(|folder_or_file| {
                let folder_or_file = folder_or_file.expect("could not  read folder");
                if folder_or_file.path().is_dir() {
                    let mut children = folder_or_file
                        .path()
                        .read_dir()
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "could not read folder {0}", folder_or_file.path().display()
                                    ),
                                );
                                res
                            },
                        );
                    let children_names = children
                        .find_map(|readdir| {
                            let readdir = readdir
                                .expect(
                                    &{
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "could not read child of {0}", folder_or_file.path()
                                                .display()
                                            ),
                                        );
                                        res
                                    },
                                );
                            if readdir.path().ends_with(QUANTITIES_FILE_NAME) {
                                Some((
                                    readdir,
                                    folder_or_file
                                        .path()
                                        .file_name()
                                        .expect(
                                            &{
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "invalid path {0}", folder_or_file.path().display()
                                                    ),
                                                );
                                                res
                                            },
                                        )
                                        .to_string_lossy()
                                        .into_owned(),
                                ))
                            } else {
                                None
                            }
                        });
                    Some(children_names.unwrap())
                } else {
                    None
                }
            })
            .map(|(quantity, folder_name)| {
                let mut file = File::open(quantity.path())
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "could not open file: {0}", quantity.path().display()
                                ),
                            );
                            res
                        },
                    );
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "could not read from: {0}", quantity.path().display()
                                ),
                            );
                            res
                        },
                    );
                let mut q: QuantitySerSer = toml::de::from_str(&contents)
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "could not parse file: {0} \n contents: {1}", quantity
                                    .path().display(), contents
                                ),
                            );
                            res
                        },
                    );
                QuantitySer {
                    name: folder_name,
                    description: q.description,
                    reference_unit: q.reference_unit,
                    dimension: q.dimension,
                }
            })
            .collect()
    }
    pub(crate) fn parse_systems() -> Vec<QSystemSer> {
        Path::new(DATA_PATH)
            .read_dir()
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("cant open data-folder {0}", DATA_PATH),
                    );
                    res
                },
            )
            .into_iter()
            .filter_map(|folder_or_file| {
                let folder_or_file = folder_or_file
                    .expect("could not read folder")
                    .path();
                let is_dir = folder_or_file.is_dir();
                if is_dir {
                    let target = &mut Default::default();
                    folder_or_file
                        .components()
                        .last()
                        .expect("failed")
                        .as_os_str()
                        .clone_into(target);
                    Some((target.clone(), folder_or_file))
                } else {
                    None
                }
            })
            .map(|(os_string, folder)| {
                let name: String = os_string
                    .clone()
                    .into_string()
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "cannot convert dir {0:?} to String", os_string
                                ),
                            );
                            res
                        },
                    );
                let dimensions = parse_dimensions(&folder.clone());
                QSystemSer::new(
                    name,
                    (&folder.as_os_str().to_string_lossy()).to_string(),
                    dimensions,
                )
            })
            .collect()
    }
    pub(crate) fn parse_units(
        quantity_path: &Path,
        prefix_map: HashMap<String, Vec<PrefixSer>>,
    ) -> Vec<UnitSer> {
        quantity_path
            .read_dir()
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("failed to read dir {0}", quantity_path.display()),
                    );
                    res
                },
            )
            .into_iter()
            .filter_map(|folder_or_file| {
                let folder_or_file = folder_or_file
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "could not read folder {0}", quantity_path.display()
                                ),
                            );
                            res
                        },
                    )
                    .path();
                if folder_or_file.is_file() {
                    if folder_or_file.ends_with(QUANTITIES_FILE_NAME) {
                        None
                    } else {
                        let folder_or_file_display = folder_or_file.display();
                        let unit: UnitSerSer = {
                            let mut file = File::open(folder_or_file.clone())
                                .expect(
                                    &{
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "could not open file: {0}", folder_or_file_display
                                            ),
                                        );
                                        res
                                    },
                                );
                            let mut contents = String::new();
                            file.read_to_string(&mut contents)
                                .expect(
                                    &{
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "could not read from: {0}", folder_or_file_display
                                            ),
                                        );
                                        res
                                    },
                                );
                            toml::de::from_str(&contents)
                                .expect(
                                    &{
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "could not parse file: {0} \n contents: {1}",
                                                folder_or_file_display, contents
                                            ),
                                        );
                                        res
                                    },
                                )
                        };
                        Some(unit)
                    }
                } else {
                    None
                }
            })
            .map(|unit| UnitSer::new(unit, prefix_map.clone()))
            .collect()
    }
}
mod uuse {
    use proc_macro2::TokenStream;
    use syn::{parse::Parse, Error, Ident, Token};
    pub(crate) struct UUse(Vec<UnitImpl>);
    pub(crate) struct UnitImpl {
        name: String,
        data_type: String,
    }
    pub(crate) fn generate_uuse(uuse: UUse) -> TokenStream {
        uuse.0.iter().map(|unit_impl| generate_unit(unit_impl)).collect()
    }
    fn generate_unit(unit_impl: &UnitImpl) -> TokenStream {
        ::core::panicking::panic("not yet implemented")
    }
    impl Parse for UUse {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let mut idents = Vec::new();
            idents.push(input.parse::<Ident>()?);
            let mut lookahead = input.lookahead1();
            while lookahead.peek(::syn::token::Comma) {
                input.parse::<::syn::token::Comma>()?;
                idents.push(input.parse()?);
                lookahead = input.lookahead1();
            }
            Ok(UUse {
                0: idents
                    .iter()
                    .map(|ident| match ident.to_string().split_once("_") {
                        Some((name, data_type)) => {
                            Ok(UnitImpl {
                                name: name.to_owned(),
                                data_type: data_type.to_owned(),
                            })
                        }
                        None => return Err(Error::new(ident.span(), "_ not found")),
                    })
                    .try_collect()?,
            })
        }
    }
}
