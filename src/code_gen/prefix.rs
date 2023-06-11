use super::factor::{Factor, FactorSer};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

const PREFIXES_PATH: &str = "prefixes";

#[derive(Serialize, Deserialize)]
pub struct PrefixSer {
    pub alias: Option<String>,
    pub factor: FactorSer,
}

#[derive(Debug)]
pub struct Prefix {
    pub alias: Option<String>,
    pub factor: Factor,
}

impl Into<Prefix> for PrefixSer {
    fn into(self) -> Prefix {
        Prefix {
            alias: self.alias,
            factor: self.factor.into(),
        }
    }
}

pub(crate) fn parse_prefixes(systempath: &Path) -> Vec<Prefix> {
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
                Some(PrefixSer::into(toml::de::from_str(&contents).expect(
                    &format!(
                        "could not desiralize file {} \n with these contents \n {}",
                        path.display(),
                        contents,
                    ),
                )))
            }
        })
        .collect()
}
