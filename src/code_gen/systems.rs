use std::path::Path;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quantities::parse_quantities;
use quote::quote;

use crate::code_gen::{generate_dim_types, generate_dimension, parse_dimensions};

use super::{generate_quantities, quantities, Dimensions};

const DATA_PATH: &str = "data";

const CODE_PATH: &str = "code";

#[derive(Debug)]
pub(crate) struct QSystemSer {
    name: String,
    path: String,
    pub(crate) dimensions: Dimensions,
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

    pub(crate) fn get_path_raw(&self) -> String {
        self.path.clone()
    }

    pub(crate) fn new(name: String, path: String, dimensions: Dimensions) -> Self {
        Self {
            name,
            path,
            dimensions,
        }
    }
}

pub(crate) fn get_systems() -> Vec<QSystemSer> {
    Path::new(DATA_PATH)
        .read_dir()
        .expect(&format!("cant open data-folder {}", DATA_PATH))
        .into_iter()
        .filter_map(|folder_or_file| {
            let folder_or_file = folder_or_file.expect("could not read folder").path();
            let is_dir = folder_or_file.is_dir();

            if is_dir {
                //confusion strikes... TODO: clear this up better... just fought with the compiler in this location
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
                .expect(&format!("cannot convert dir {:?} to String", os_string));
            let dimensions = parse_dimensions(&folder.clone());
            QSystemSer {
                name,
                dimensions,
                path: (&folder.as_os_str().to_string_lossy()).to_string(),
            }
        })
        .collect()
}

pub(crate) fn generate_systems(systems: Vec<QSystemSer>) -> TokenStream {
    // Systems-Enums
    let dimensions_code = generate_dim_types(&systems);

    // generate the system-specific stuff
    let systems_code = systems.iter().map(|system| generate_system(system));

    quote!(
        #dimensions_code

        #(#systems_code)

        //------------------NEXT SYSTEM------------------
        *
    )
}

fn generate_system(system: &QSystemSer) -> TokenStream {
    let dimensions = generate_dimension(system.get_name(), system.dimensions.clone());

    let quantities = generate_quantities(
        parse_quantities(Path::new(&system.path)),
        system.name.clone(),
    );
    todo!()
}
