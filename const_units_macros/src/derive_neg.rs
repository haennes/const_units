use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;
use syn::{Data, Ident};

/// fix the as conversion in line 17
pub(crate) fn generate_derive_neg(item: Ident, data: Data, path: Path) -> TokenStream {
    match data {
        Data::Struct(struct_data) => {
            let fields_expanded = struct_data.fields.iter().enumerate().map(|(idx, field)| {
                match field.ident.clone() {
                    Some(ident) => quote!(#ident),
                    None => {
                        let idx_str: syn::Expr =
                            syn::parse_str(&(idx as u32).to_string()).expect("int to expr failed!");
                        quote!(#idx_str)
                        //Ident::new(&(idx as u32).to_string(), Span::call_site())
                    }
                }
            });
            quote!(
                impl const const_ops::Neg for #item {
                    type Output = Self;

                    fn neg(self) -> Self::Output {
                        Self{ #(#fields_expanded: #path::Neg::neg(self. #fields_expanded)),*}
                    }
                }

            )
        }
        Data::Enum(enum_data) => {
            let items = enum_data.variants.iter().map(|var| {
                let var_ident = var.ident.clone();
                let fields =
                    var.fields
                        .iter()
                        .enumerate()
                        .map(|(idx, field)| match field.ident.clone() {
                            Some(ident) => ident,
                            None => Ident::new(&("arg".to_owned() + &(idx as u32).to_string()), Span::call_site()),
                        });
                let fields_clone = fields.clone();
                quote!(
                    #item :: #var_ident ( #(#fields)*, ) => #item :: #var_ident( #( #path::Neg::neg(#fields_clone) )*, )
                )
            });
            quote!(
                impl const const_ops::Neg for #item {
                    type Output = Self;
                    fn neg(self) -> Self::Output {
                        match self {
                            #(#items),*
                        }
                    }
                }
            )
        }
        Data::Union(_) => panic!("not able to use this in an union"),
    }
}

// impl Neg for DimType {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         match self {
//             DimType::SiExtended(q) => DimType::SiExtended(q.neg()),
//         }
//     }
// }
