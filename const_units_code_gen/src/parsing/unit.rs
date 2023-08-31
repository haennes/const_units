use either::Either;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, path::Path};

use super::{ConversionSer, ConversionSerSer, PrefixGroup, PrefixSer, QUANTITIES_FILE_NAME};

pub(crate) fn parse_units(
    quantity_dir: &Path,
    prefix_map: HashMap<String, PrefixGroup>,
) -> Vec<UnitSer> {
    quantity_dir
        .read_dir()
        .expect(&format!("failed to read dir {}", quantity_dir.display()))
        .into_iter()
        .filter_map(|folder_or_file| {
            let folder_or_file = folder_or_file
                .expect(&format!("could not read folder {}", quantity_dir.display()))
                .path();
            if folder_or_file.is_file() {
                if folder_or_file.ends_with(QUANTITIES_FILE_NAME) {
                    None
                } else {
                    let folder_or_file_display = folder_or_file.display();
                    let unit: UnitSerSer = {
                        let mut file = File::open(folder_or_file.clone())
                            .expect(&format!("could not open file: {}", folder_or_file_display));
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)
                            .expect(&format!("could not read from: {}", folder_or_file_display));
                        toml::de::from_str(&contents).expect(&format!(
                            "could not parse file: {} \n contents: {}",
                            folder_or_file_display, contents
                        ))
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub(crate) struct UnitNameSerSer {
    #[serde(with = "either::serde_untagged")]
    inner: Either<String, (String, String)>,
}

impl UnitNameSerSer {
    pub fn new(either: Either<String, (String, String)>) -> Self {
        Self { inner: either }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UnitSerSer {
    pub(crate) symbol: String,
    pub(crate) names: HashMap<String, UnitNameSerSer>,
    // for example metric, bin ...
    pub(crate) derive_prefixes: Option<Vec<String>>,
    pub(crate) conversions: Option<HashMap<String, ConversionSerSer>>,
}

#[derive(Clone, Debug)]
pub(crate) struct UnitSer {
    pub(crate) symbol: String,
    pub(crate) names: HashMap<String, UnitNameSer>,
    pub(crate) prefixes: Vec<PrefixSer>,
    pub(crate) conversions: HashMap<String, ConversionSer>,
}

impl From<UnitNameSerSer> for UnitNameSer {
    fn from(value: UnitNameSerSer) -> Self {
        match value.inner {
            Either::Left(string) => UnitNameSer {
                singular: string.clone(),
                plural: string,
            },
            Either::Right((singular, plural)) => UnitNameSer { singular, plural },
        }
    }
}

impl UnitSer {
    // prefix_map: example: ("bin", "peta, exa, tera")
    pub(crate) fn new(unit: UnitSerSer, prefix_map: HashMap<String, PrefixGroup>) -> UnitSer {
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
                            .expect(&format!("could not find prefix-group {}", group_name))
                            .clone()
                    })
                    .flatten()
                    .collect_vec()
            },
            conversions: unit
                .conversions
                .unwrap_or_default()
                .iter()
                .map(|(key, value)| (key.clone(), value.into()))
                .collect::<HashMap<String, ConversionSer>>(), //.into(),
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

impl IntoIterator for UnitNameSer {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        if self.singular == self.plural {
            vec![self.singular].into_iter()
        } else {
            vec![self.singular, self.plural].into_iter()
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UnitNameSer {
    pub(crate) singular: String,
    pub(crate) plural: String,
}
