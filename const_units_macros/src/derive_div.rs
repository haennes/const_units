use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;
use syn::{Data, Ident};

pub(crate) fn generate_derive_div(item: Ident, data: Data, path: Path) -> TokenStream {
    match data {
        Data::Struct(struct_data) => {
            let fields = struct_data.fields.iter().enumerate().map(|(idx, field)| {
                //TODO make this better...
                match field.ident.clone() {
                    Some(ident) => quote!(#ident: #path::Div::div(self.#ident, rhs.#ident)),
                    None => {
                        let ident = Ident::new(&(&(idx as u32).to_string()), Span::call_site());
                        quote!(#ident: #path::Div::div(self.#ident, rhs.#ident))
                    }
                }
            });

            quote!(
                impl const const_ops::Div for #item {
                type Output = Self;
                fn div(self, rhs: Self) -> Self::Output{
                    Self{
                        #(#fields),*
                    }
                }
            })
        }
        Data::Enum(enum_data) => {
            let enum_variants = enum_data.variants.iter().map(|variant| {
                let fields = variant.fields.iter().enumerate().map(|(idx, field)| {
                    match field.ident.clone() {
                        Some(ident) => (ident, true),
                        None => (
                            Ident::new(
                                &("arg".to_owned() + &(idx as u32).to_string()),
                                Span::call_site(),
                            ),
                            false,
                        ),
                    }
                });
                let first_fields = fields.clone().map(|(ident, original)| {
                    let span = ident.span();
                    (
                        Ident::new(&("a_".to_owned() + &ident.to_string()), span),
                        original,
                    )
                });
                let second_fields = fields.map(|(ident, original)| {
                    let span = ident.span();
                    (
                        Ident::new(&("b_".to_owned() + &ident.to_string()), span),
                        original,
                    )
                });

                let fields_set = first_fields
                    .clone()
                    .zip(second_fields.clone())
                    .enumerate()
                    .map(|(idx, (first_field, second_field))| {
                        let first_field_ident = first_field.0;
                        let second_field_ident = second_field.0;
                        if first_field.1 {
                            quote!( #first_field_ident: #path::Div::div(#first_field_ident, #second_field_ident))
                        } else {
                            let idx_str: syn::Expr = syn::parse_str(&(idx as u32).to_string())
                                .expect("int to expr failed!");
                            quote!(#idx_str: #path::Div::div(#first_field_ident, #second_field_ident))
                        }
                    });
                let first_fields_names = first_fields.map(|(ident, _)| ident);
                let second_fields_names = second_fields.map(|(ident, _)| ident);

                let ident = variant.ident.clone();
                let ident_cp = ident.clone();
                let ident_cp_cp = ident.clone();
                quote!(
                    #item::#ident(#(#first_fields_names),*) => {
                        if let #item::#ident_cp(#(#second_fields_names),*) = rhs {
                            #item::#ident_cp_cp{ #(#fields_set),* }
                        } else{
                            panic!("can only multiply if the DimType is the same")
                        }
                    }
                )
            });
            let res = quote!(

                impl const const_ops::Div for #item {
                    //TESTCOMMENT
                    type Output = Self;
                    fn div(self, rhs: Self) -> Self::Output {
                        match self{
                            #(#enum_variants),*
                        }
                    }
                }
            );
            //panic!("{}", res.to_string());
            res
        }
        Data::Union(_) => panic!("only available for enums"),
    }
}
