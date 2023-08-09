use crate::parsing::{QuantitySer, UnitSer};
use convert_case::Encased;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) struct UnitNameGen {
    u_name: String,
    base_u_name: String,
}

impl UnitNameGen {
    pub(crate) fn new(u_name: String, base_u_name: String) -> Self {
        Self {
            u_name,
            base_u_name,
        }
    }
    pub(crate) fn new_base(u_name: String) -> Self {
        Self {
            base_u_name: u_name.clone(),
            u_name,
        }
    }
}

pub(crate) struct PrefixGen {
    name: Encased<{ Case::UpperCamel }>,
    base_name: Encased<{ Case::UpperCamel }>,
}

impl PrefixGen {
    fn new(name: impl ToString, base_name: impl ToString) -> Self {
        Self {
            name: name.to_string().encased(),
            base_name: base_name.to_string().encased(),
        }
    }
}

pub(crate) fn generate_unit_code_name(
    quantity: QuantitySer,
    name: UnitNameGen,
    dim_type: Encased<{ Case::UpperCamel }>,
    prefix: Option<Encased<{ Case::UpperCamel }>>,
) -> TokenStream {
    let u_name: Ident = syn::parse_str(&name.u_name).unwrap();
    let base_u_name: Ident = syn::parse_str(&name.base_u_name.to_case(Case::UpperCamel)).unwrap();
    let dim_type: Ident = syn::parse_str(&dim_type.raw()).unwrap();
    let q_name: Ident = syn::parse_str(quantity.name()).unwrap();

    let dimensions = quantity.dimension().iter().map(|(name, power)| {
        let name: Ident = syn::parse_str(name).unwrap();
        quote!( #name: #power)
    });

    let dim_type_twice = quote!(#dim_type :: #dim_type);

    let q_const = quote!(
        Quantity{
            name: QName::#q_name,
            dimensions: SystemDim::#dim_type( #dim_type_twice{
                #(#dimensions),*,..#dim_type::NONE
            })
        }
    );
    let prefix: Option<Ident> = match prefix {
        Some(prefix) => Some(syn::parse_str(&prefix.raw()).unwrap()),
        None => None,
    };
    let prefix = match prefix {
        Some(prefix) => quote!(PName::#prefix),
        None => quote!(PName::None),
    };
    let uname = quote!(UName::#base_u_name);
    let prefix = quote!(Prefix :: from( #prefix ));
    //FIXME allow dead_code
    //IMPORANT!! do not convert u_name to UpperCamel as it causes name collisions
    quote::quote!(
        #[allow(dead_code,non_camel_case_types)]
        pub type #u_name<DT> = Unit<DT,{#uname},  {#q_const}, { #prefix }>;
    )
}

// Will only be added once Display will be implemented with translation support
// pub(crate) fn generate_unit_code_lang(
//     q_name: QuantitySer,
//     unit: &UnitSer,
//     language: &String,
//     data_type: String,
//     dim_type: String,
//     prefix: String,
// ) -> TokenStream {
//     generate_unit_code_name(
//         q_name,
//         UnitNameGen::new(unit.names[language].singular.clone(),),
//         data_type,
//         dim_type,
//         prefix,
//     )
// }

// pub(crate) fn generate_unit_code_symbol(
//     name: UnitNameGen,
//     data_type: String,
//     dim_type: String,
//     q_name: QuantitySer,
//     prefix: String,
// ) -> TokenStream {
//     generate_unit_code_name(
//         q_name,
//         name,
//         data_type,
//         dim_type.encased(),
//         prefix.encased(),
//     )
// }

pub(crate) fn generate_units(
    default_lang: String,
    units: Vec<UnitSer>,
    dim_type: Encased<{ Case::UpperCamel }>,
    quantity: QuantitySer,
) -> TokenStream {
    units
        .iter()
        .map(|unit| -> TokenStream {
            vec![
                unit.prefixes
                    .iter()
                    .map(|prefix| -> TokenStream {
                        [Some(prefix.name().clone()), prefix.alias().clone()]
                            .iter()
                            .filter_map(|item| item.clone())
                            .map(|p_name| -> TokenStream {
                                //PREFIX NAME NORMALISATION µ -> micro
                                //TODO
                                generate_unit_code_name(
                                    quantity.clone(),
                                    UnitNameGen::new(
                                        format!("{}{}", p_name, unit.names[&default_lang].singular),
                                        unit.names[&default_lang].clone().singular,
                                    ),
                                    dim_type.clone(),
                                    Some(prefix.name().encased()),
                                )
                            })
                            .collect()
                    })
                    .collect(),
                generate_unit_code_name(
                    quantity.clone(),
                    UnitNameGen::new_base(unit.names[&default_lang].clone().singular),
                    dim_type.clone(),
                    None,
                ),
            ]
            .iter()
            .cloned()
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

pub(crate) fn generate_uname_enum(units: Vec<UnitSer>, default_lang: impl ToString) -> TokenStream {
    let names = units.iter().map(|unit| -> Ident {
        syn::parse_str(
            &unit.names[&default_lang.to_string()]
                .singular
                .to_case(Case::UpperCamel)
                .clone(),
        )
        .expect(&format!(
            "failed to parse {} to an Ident",
            unit.symbol.clone()
        ))
    });

    //FIXME implement Display (languages....) instead of deriving it.
    quote!(
        #[derive(PartialEq, Eq, core::marker::ConstParamTy, parse_display::Display)]
        pub enum UName{
            #(#names),*
        }
    )
}

pub(crate) fn generate_uname_inv_mul(
    units: Vec<UnitSer>,
    default_lang: impl ToString,
) -> TokenStream {
    quote!(
        impl const num_traits::Inv for UName {
            type Output = UName;

            #[inline]
            fn inv(self) -> Self::Output {
                //TODO
                todo!()
            }
        }

        impl const const_ops::Mul for UName {
            type Output = UName;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                //TODO
                todo!()
            }
        }
    )
}
