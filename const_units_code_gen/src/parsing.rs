use const_units_global_types::{Factor, RatioConst, F64};
// use crate::global_types::{
//     factor::{Factor, RatioConst},
//     prefix::Prefix,
// };
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

const DATA_PATH: &str = "./data";
const DIMENSIONS_PATH: &str = "dimensions.toml";
const PREFIXES_PATH: &str = "prefixes";
const QUANTITIES_PATH: &str = "quantities";
const QUANTITIES_FILE_NAME: &str = "quantity.toml";
//const UNIT_PATH: &str = "units";

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

pub(crate) type Dimensions = HashMap<String, String>;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct FactorSer {
    #[serde(with = "either::serde_untagged")]
    inner: Either<f64, String>,
}

impl FactorSer {
    pub fn new<T: ToString>(expr: T) -> Self {
        Self {
            inner: Either::Right(expr.to_string()),
        }
    }
}

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
pub(crate) struct UnitNameSer {
    pub(crate) singular: String,
    pub(crate) plural: String,
}

#[derive(Clone, Debug)]
pub(crate) enum AccuracySer {
    Exact,
    Inaccurate(i32),
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

impl Into<Factor> for FactorSer {
    fn into(self) -> Factor {
        match self.inner {
            Either::Left(float) => Factor::Float(F64::from_f64(float)),
            Either::Right(string) => {
                match string.split_once("/") {
                    Some((num, denom)) => {
                        let (num, denom): (i128, i128) =
                            (parse_to_int_pow(num), parse_to_int_pow(denom));
                        Factor::Ratio(RatioConst::new_raw(num, denom))
                    }
                    None => {
                        //Ratio::new(parse_to_int(&str), 1)
                        match string.split_once(".") {
                            Some(_) => Factor::Float(
                                string
                                    .parse()
                                    .expect(&format!("failed to parse {} to a float", string)),
                            ),
                            None => Factor::Ratio(RatioConst::new_raw(parse_to_int(&string), 1)),
                        }
                    }
                }
            }
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
    fn new(unit: UnitSerSer, prefix_map: HashMap<String, PrefixGroup>) -> UnitSer {
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
                .map(|(key, value)| (key.clone(), (value.clone()).into()))
                .collect::<HashMap<String, ConversionSer>>(), //.into(),
        }
    }
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

impl From<String> for UnitNameSer {
    fn from(value: String) -> Self {
        UnitNameSer {
            singular: value.clone(),
            plural: value,
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
