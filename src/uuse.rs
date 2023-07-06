use proc_macro2::TokenStream;
use syn::{parse::Parse, Error, Ident, Token};

pub(crate) struct UUse(Vec<UnitImpl>);

pub(crate) struct UnitImpl {
    name: String,
    data_type: String,
}

pub(crate) fn generate_uuse(uuse: UUse) -> TokenStream {
    uuse.0
        .iter()
        .map(|unit_impl| generate_unit(unit_impl))
        .collect()
}

fn generate_unit(unit_impl: &UnitImpl) -> TokenStream {
    todo!()
    // optional_create_dbs();
    // let db = get_db_units();
    // let unit = db.get(unit_impl.name);
}

impl Parse for UUse {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut idents = Vec::new();
        idents.push(input.parse::<Ident>()?);
        let mut lookahead = input.lookahead1();
        while lookahead.peek(Token!(,)) {
            input.parse::<Token!(,)>()?;
            idents.push(input.parse()?);
            lookahead = input.lookahead1();
        }
        Ok(UUse {
            0: idents
                .iter()
                .map(|ident| match ident.to_string().split_once("_") {
                    Some((name, data_type)) => Ok(UnitImpl {
                        name: name.to_owned(),
                        data_type: data_type.to_owned(),
                    }),
                    None => return Err(Error::new(ident.span(), "_ not found")),
                })
                .try_collect()?,
        })
    }
}
