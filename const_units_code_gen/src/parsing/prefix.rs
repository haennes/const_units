use const_units_global_types::{Factor, RatioConst};
// use crate::global_types::{
//     factor::{Factor, RatioConst},
//     prefix::Prefix,
// };
use super::{FactorSer, PREFIXES_PATH};
use convert_case::{Case, Casing, Encased};
use either::Either;
use getset::{CopyGetters, Getters};
use itertools::Itertools;
use self_rust_tokenize::SelfRustTokenize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrefixSerSer {
    pub alias: Option<String>,
    pub factor: FactorSer,
}

#[derive(Clone, Debug, PartialEq, Eq, SelfRustTokenize, Getters, CopyGetters)]
#[getset(get = "pub")]
pub struct PrefixSer {
    name: String,
    alias: Option<String>,
    factor: Factor,
}

impl PrefixSer {
    pub fn new<N, A>(name: N, alias: Option<A>, factor: Factor) -> Self
    where
        N: ToString,
        A: ToString,
    {
        Self {
            name: name.to_string(),
            alias: alias.map(|alias| alias.to_string()),
            factor,
        }
    }
}

#[derive(SelfRustTokenize, Clone, Debug)]
pub(crate) struct PrefixGroup(Vec<PrefixSer>);

impl IntoIterator for PrefixGroup {
    type Item = PrefixSer;

    type IntoIter = std::vec::IntoIter<PrefixSer>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<(String, PrefixSerSer)> for PrefixSer {
    fn from((name, prefix): (String, PrefixSerSer)) -> Self {
        Self::new(name, prefix.alias, prefix.factor.into())
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
        .expect(&format!("wrong prefixes-dir: {}", folder.display()))
        .into_iter()
        .filter_map(|file| {
            let path = file
                .expect(&format!("could not read {}", folder.display()))
                .path();
            if path.extension().unwrap_or_default() != "toml" {
                None
            } else {
                let mut file =
                    File::open(path.clone()).expect(&format!("could not open {}", path.display()));
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect(&format!("could not read file {}", path.display()));
                let out = {
                    let contents_clone = contents.clone();
                    toml::de::from_str::<HashMap<String, PrefixSerSer>>(&contents)
                        .expect(&format!(
                            "could not desiralize file {} \n with these contents \n {}",
                            path.display(),
                            contents_clone,
                        ))
                        .into_iter()
                        .collect_vec()
                };
                Some((
                    path.file_name()
                        .expect(&format!("invalid Path {}", path.display()))
                        .to_str()
                        .expect(&format!("failed to parse {} to a String", path.display()))
                        .split_once(".toml")
                        .expect(&format!(
                            "invalid filename (missing .toml) {}",
                            path.display()
                        ))
                        .0
                        .to_string(),
                    out.into(),
                ))
            }
        })
        .collect()
}
