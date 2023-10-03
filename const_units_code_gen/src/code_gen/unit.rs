use std::rc::Rc;

use crate::parsing::{BaseUNamePowSer, QuantitySer, UnitSer};
use const_units_global_types::str_to_ident;
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
    default_lang: &String,
    unit: UnitSer,
    dim_type: Encased<{ Case::UpperCamel }>,
) -> TokenStream {
    let quantity = unit.quantity();
    let unit_name_lang = unit.get_name(&default_lang);
    let u_name = unit_name_lang.singular();
    let dim_type = str_to_ident(dim_type);
    let q_name = str_to_ident(quantity.name());

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
    let binding = unit.clone().base_units_sorted(default_lang.clone());
    let base_units_sorted = binding.iter().map(|BaseUNamePowSer { name, pow }| {
        quote!(
            BaseUNamePow{
                name: BaseUName::#name,
                pow: #pow
            }
        )
    });
    let uname = quote!(
        UName::new_arr([#(#base_units_sorted),*])
    );
    unit.prefixes
        .iter()
        .map(|a| Some(a))
        .chain([None].iter().cloned())
        .map(|prefix| {
            let (u_name, prefix) = match prefix {
                Some(prefix) => {
                    let prefix_id = str_to_ident(prefix.name().to_case(Case::UpperCamel));
                    let u_name = str_to_ident(&format!("{}{}", prefix.name(), u_name));
                    (u_name, quote!(PName::#prefix_id))
                }
                None => (str_to_ident(u_name), quote!(PName::None)),
            };
            let prefix = quote!(Prefix :: from( #prefix ));
            //FIXME allow dead_code
            //IMPORANT!! do not convert u_name to UpperCamel as it causes name collisions
            quote::quote!(
                #[allow(dead_code,non_camel_case_types)]
                pub type #u_name<DT> = Unit<DT,{#uname},  {#q_const}, { #prefix }>;
            )
        })
        .collect()
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
    default_lang: impl ToString,
    units: Vec<Rc<UnitSer>>,
    dim_type: Encased<{ Case::UpperCamel }>,
) -> TokenStream {
    units
        .iter()
        .map(|unit| -> TokenStream {
            println!(
                "generating {}",
                unit.get_name(&default_lang.to_string()).singular()
            );
            generate_unit_code_name(
                &default_lang.to_string(),
                Rc::unwrap_or_clone((unit.clone())),
                dim_type.clone(),
            )
            // generate_unit_code_name(
            //     quantity.clone(),
            //     UnitNameGen::new(
            //         format!("{}{}", p_name, unit.get_name(&default_lang).singular),
            //         unit.get_name(&default_lang).clone().singular,
            //     ),
            //     unit.clone(),
            //     dim_type.clone(),
            //     Some(prefix.name().encased()),
            // )
        })
        .collect()
}

pub(crate) fn generate_generic_units(quantities: Vec<QuantitySer>) -> TokenStream {
    quantities.iter().map(|quantity| {
        let name: Ident = syn::parse_str(&quantity.name().to_case(Case::UpperCamel))
            .expect(&format!("failed to parse {} to an Iden", quantity.name()));
        quote!(
            pub type #name <DT,const NAME: UName, const PREFIX: Prefix> = Unit<DT, NAME, {crate::quantity::Quantity::from_name(QName::#name)}, PREFIX>;
        )
    }).collect()
}

pub(crate) fn generate_uname_enum(
    units: Vec<Rc<UnitSer>>,
    default_lang: impl ToString,
) -> TokenStream {
    let names = units.iter().map(|unit| -> Ident {
        str_to_ident(
            &unit
                .get_name(&default_lang.to_string())
                .singular()
                .to_case(Case::UpperCamel)
                .clone(),
        )
    });

    //FIXME implement Display (languages....) instead of deriving it.
    quote!(
        #[derive(PartialEq, Eq, core::marker::ConstParamTy, parse_display::Display, Copy, Clone)]
        pub enum BaseUName{
            #(#names),*
        }
    )
}

// pub(crate) fn generate_uname_inv_mul(
//     units: Vec<UnitSer>,
//     default_lang: impl ToString,
// ) -> TokenStream {
//     quote!(
//         impl const const_ops::Neg for BaseUName {
//             type Output = BaseUName;

//             //#[inline]
//             fn neg(self) -> Self::Output {
//                 //TODO
//                 self
//             }
//         }

//         impl const const_ops::Mul for BaseUName {
//             type Output = BaseUName;

//             //#[inline]
//             fn mul(self, rhs: Self) -> Self::Output {
//                 //TODO
//                 self
//             }
//         }

//         impl const const_ops::Div for UName {
//             type Output = BaseUName;

//             //#[inline]
//             fn div(self, rhs: Self) -> Self::Output {
//                 //TODO
//                 self
//             }
//         }
//     )
// }
