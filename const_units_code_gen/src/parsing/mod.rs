use const_units_global_types::{Factor, RatioConst};
// use crate::global_types::{
//     factor::{Factor, RatioConst},
//     prefix::Prefix,
// };

use either::Either;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

mod factor;
mod prefix;
mod quantity;
mod unit;
pub use factor::*;
pub use prefix::*;
pub use quantity::*;
pub use unit::*;

const DATA_PATH: &str = "./data";
const DIMENSIONS_PATH: &str = "dimensions.toml";
const PREFIXES_PATH: &str = "prefixes";
const QUANTITIES_PATH: &str = "quantities";
const QUANTITIES_FILE_NAME: &str = "quantity.toml";
//const UNIT_PATH: &str = "units";

pub(crate) type Dimensions = HashMap<String, String>;

#[derive(Serialize, Deserialize)]
pub(crate) struct ConversionSerSer {
    factor: FactorSer,
    accuracy: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ConversionSer {
    factor: Factor,
    accuracy: AccuracySer,
}

#[derive(Clone, Debug)]
pub(crate) enum AccuracySer {
    Exact,
    Inaccurate(i32),
}

impl Into<ConversionSer> for &ConversionSerSer {
    fn into(self) -> ConversionSer {
        ConversionSer {
            factor: self.factor.clone().into(),
            accuracy: if self.accuracy.to_lowercase() == "e" {
                AccuracySer::Exact
            } else {
                AccuracySer::Inaccurate(parse_to_int(&self.accuracy) as i32)
            },
        }
    }
}

pub(crate) fn parse_dimensions(systempath: &Path) -> Dimensions {
    #[derive(Serialize, Deserialize)]
    struct DimensionsSer {
        dimensions: HashMap<String, String>,
    }

    let path = systempath.join(DIMENSIONS_PATH);
    let contents =
        fs::read_to_string(path.clone()).expect(&format!("failed to read file {}", path.display()));
    let dim: DimensionsSer = toml::de::from_str(&contents).expect(&format!(
        "failed to parse file {} \n contents: {}",
        path.display(),
        contents
    ));
    dim.dimensions
}

pub(crate) fn parse_to_int_pow(num: &str) -> i128 {
    match num.split_once("^") {
        Some((str1, str2)) => {
            if str2 == "" {
                parse_to_int(str1)
            } else {
                let base = parse_to_int(str1);
                let pow = parse_to_int(str2);
                base.pow(pow as u32)
            }
        }
        None => parse_to_int(num),
    }
}

pub(crate) fn parse_to_int(num: &str) -> i128 {
    num.parse()
        .expect(&format!("failed to parse {} to a string", num))
}

pub(crate) fn parse_systems() -> Vec<QSystemSer> {
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
            QSystemSer::new(
                name,
                (&folder.as_os_str().to_string_lossy()).to_string(),
                dimensions,
            )
        })
        .collect()
}
