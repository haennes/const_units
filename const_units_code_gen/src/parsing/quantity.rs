use convert_case::{Case, Casing, Encased};
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use super::QUANTITIES_FILE_NAME;

use super::QUANTITIES_PATH;

use super::Dimensions;

#[derive(Debug, PartialEq, Eq, Getters, CopyGetters, Clone)]
#[getset(get = "pub")]
pub(crate) struct QuantitySer {
    name: String,
    path: PathBuf,
    description: Option<String>,
    reference_unit: String,
    dimension: HashMap<String, i8>,
}

impl QuantitySer {
    pub fn new(
        name: impl ToString,
        path: PathBuf,
        description: Option<impl ToString>,
        reference_unit: impl ToString,
        dimension: HashMap<impl ToString, i8>,
    ) -> Self {
        Self {
            name: name.to_string().to_case(Case::UpperCamel),
            path: path,
            description: description.map(|value| value.to_string()),
            reference_unit: reference_unit.to_string(),
            dimension: dimension
                .iter()
                .map(|(key, value)| (key.to_string(), *value))
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Getters, Clone)]
#[getset(get = "pub")]
pub(crate) struct QSystemSer {
    name: Encased<{ Case::UpperCamel }>,
    path: String,
    dimensions: Dimensions,
}

//TODO QSystemSer + rename of oldf
// additional field: quantities

impl QSystemSer {
    pub(crate) fn get_name_raw(&self) -> String {
        self.name.raw().clone()
    }

    pub(crate) fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn new<N: ToString, O: ToString>(
        name: impl ToString,
        path: impl ToString,
        dimensions: HashMap<N, O>,
    ) -> Self {
        Self {
            name: name.to_string().encased(),
            path: path.to_string(),
            dimensions: dimensions
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }
}

pub(crate) fn parse_quantities(systempath: &Path) -> Vec<QuantitySer> {
    #[derive(Serialize, Deserialize)]
    struct QuantitySerSer {
        description: Option<String>,
        reference_unit: String,
        dimension: HashMap<String, i8>,
    }

    let path = systempath.join(QUANTITIES_PATH);
    path.read_dir()
        .expect(&format!("failed to open {}", path.display()))
        .into_iter()
        .filter_map(|folder_or_file| {
            let folder_or_file = folder_or_file.expect("could not  read folder");
            if folder_or_file.path().is_dir() {
                let mut children = folder_or_file.path().read_dir().expect(&format!(
                    "could not read folder {}",
                    folder_or_file.path().display(),
                ));
                let children_names = children.find_map(|readdir| {
                    let readdir = readdir.expect(&format!(
                        "could not read child of {}",
                        folder_or_file.path().display()
                    ));
                    if readdir.path().ends_with(QUANTITIES_FILE_NAME) {
                        Some((
                            readdir,
                            folder_or_file
                                .path()
                                .file_name()
                                .expect(&format!(
                                    "invalid path {}",
                                    folder_or_file.path().display()
                                ))
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
            let mut file = File::open(quantity.path()).expect(&format!(
                "could not open file: {}",
                quantity.path().display()
            ));
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect(&format!(
                "could not read from: {}",
                quantity.path().display()
            ));
            let mut q: QuantitySerSer = toml::de::from_str(&contents).expect(&format!(
                "could not parse file: {} \n contents: {}",
                quantity.path().display(),
                contents
            ));
            QuantitySer::new(
                folder_name,
                quantity.path(),
                q.description,
                q.reference_unit,
                q.dimension,
            )
        })
        .collect()
}
