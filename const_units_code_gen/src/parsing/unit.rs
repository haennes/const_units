use super::{ConversionSer, ConversionSerSer, PrefixGroup, PrefixSer, QUANTITIES_FILE_NAME, QuantitySer};
use const_units_global_types::str_to_ident;
use convert_case::{Casing, Case};
use either::Either;
use getset::{CopyGetters, Getters};
use itertools::Itertools;
use petgraph::{prelude::DiGraph, visit::IntoNodeReferences, Direction};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, fs::File, hash::Hash, io::Read, num::NonZeroI8, path::Path, rc::Rc, todo, cell::RefCell, println,
};
use syn::Ident;
/// keys: Name of file
/// values: Unit
pub(crate) fn parse_units(
    quantities_parent_dir: &Path,
    prefix_map: HashMap<String, PrefixGroup>,
    quantity_map: HashMap<String, QuantitySer>,
) -> HashMap<String, Rc<UnitSer>> {
    let unit_ser_input_map: HashMap<_, _> = quantities_parent_dir.read_dir().expect("failed to read...").into_iter().filter_map(|f| f.ok().filter(|f| f.path().is_dir())).map( |quantity_dir| {
        let quantity_path = quantity_dir.path();
        let quantity_map_clone = quantity_map.clone();
        let quantity_path_clone = quantity_path.clone();
        quantity_path_clone
        .read_dir()
        .expect(&format!("failed to read dir {}", quantity_dir.path().display()))
        .into_iter()
        .filter_map(move |folder_or_file| {
            let folder_or_file = folder_or_file
                //.expect(&format!("could not read folder {}", quantity_dir.path().display()))
                .unwrap()
                .path();
            if folder_or_file.is_file() {
                if folder_or_file.ends_with(QUANTITIES_FILE_NAME) {
                    None
                } else {
                    let folder_or_file_display = folder_or_file.display();
                    let unit: UnitSerSerOrd = {
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
                    Some((
                        folder_or_file
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string().clone(),
                        (quantity_map_clone.get(quantity_path.file_stem().unwrap().to_str().unwrap()).unwrap().clone(), unit.clone())
                    ))
                }
            } else {
                None
            }
        })})
        .flatten()
        //.map(|(file_name, unit)| UnitSerSer::new(file_name.to_string(), unit, prefix_map.clone()))
        .collect();

    let mut graph = DiGraph::<String, ()>::new();
    println!("ParsingHashMap: {:?}", unit_ser_input_map.keys());
    unit_ser_input_map
        .iter()
        .for_each(
            |(uname, (quantity, unit_ser_ser_ser))| match &unit_ser_ser_ser.composite {
                Some(composite) => composite.iter().for_each(|(name, _)| {
                    let uname_idx = graph
                        .node_references()
                        .find_map(|(idx, data)| if data == uname { Some(idx) } else { 
                            None
                        }).unwrap_or_else(||{
                            graph.add_node(name.clone())
                        });
                    let name_idx = graph
                        .node_references()
                        .find_map(|(idx, data)| if data.clone() == name.clone() { Some(idx) } else { None })
                        .unwrap();

                    let _ = graph.add_edge(uname_idx, name_idx, ());
                }),
                None => {
                    let _ = graph.add_node(uname.to_string());
                }
            },
        );
    //let mut output_hashmap_mut : HashMap<String, Rc<UnitSer>> = HashMap::new();
    let mut output_hashmap: Rc<RefCell<HashMap<String, Rc<UnitSer>>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut count = 0;
    while graph.node_count() > 0 {
        println!("iteration: {}", count);
        count += 1;
        let mut remove_buffer = Vec::new();
        graph.externals(Direction::Outgoing).for_each(|idx| {
            let name = &graph.clone()[idx];
            let other_out = Rc::clone(&output_hashmap);
            let unit = unit_ser_input_map[name].clone();
            output_hashmap.borrow_mut().insert(
                name.clone(),
                Rc::new(UnitSer::new(unit.1, prefix_map.clone(), Rc::clone(&other_out),unit.0)),
            );
            remove_buffer.push(idx);
        });
        remove_buffer.iter().for_each(|idx|{let _ = graph.remove_node(*idx);})
    }
    let out=Rc::try_unwrap(output_hashmap);
    out.unwrap().into_inner()
}

#[derive(Serialize, Deserialize, Ord, Hash, PartialEq, Eq, PartialOrd, Clone)]
#[serde(transparent)]
pub(crate) struct UnitNameSerSer {
    // Left: singular only,
    // Right: adds plural
    #[serde(with = "either::serde_untagged")]
    inner: Either<String, (String, String)>,
}

impl UnitNameSerSer {
    pub fn new(either: Either<String, (String, String)>) -> Self {
        Self { inner: either }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UnitSerSerOrd {
    pub(crate) symbol: String,
    pub(crate) description: Option<String>,
    pub(crate) names: Option<HashMap<String, UnitNameSerSer>>,
    // for example metric, bin ...
    pub(crate) derive_prefixes: Option<Vec<String>>,
    pub(crate) conversions: Option<HashMap<String, ConversionSerSer>>,
    // used in conjuction with composite
    // if true -> names are automatically generated -> {UNIT1}_per_{UNIT2}
    // else names have to be given
    // Default = true
    pub(crate) inherit_names: Option<bool>,
    // used for composite units
    pub(crate) composite: Option<HashMap<String, i8>>,
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct UnitSer {
    //pub(crate) symbol: String,
    //pub(crate) names: HashMap<String, UnitNameSer>,
    //pub(crate) prefixes: Vec<PrefixSer>,
    pub(crate) conversions: HashMap<String, ConversionSer>,
    //pub(crate) composite: Vec<(UnitSerSer, i8)>,
    pub(crate) prefixes: Vec<PrefixSer>,
    pub(crate) ty: UnitTypeSer,
    pub(crate) description: Option<String>,
    pub(crate) quantity: QuantitySer
}

#[derive(Clone, Debug)]
pub(crate) enum UnitTypeSer {
    Composed(TypeComposed),
    Simple(UNameSer),
}
impl UnitTypeSer {
    fn get_name(&self, lang: &String) -> UNamePars {
        match self {
            Self::Composed(com) => com.get_name(lang),
            Self::Simple(sim) => sim.get_name(lang),
        }
    }
}
#[derive(Getters, CopyGetters, Clone)]
#[getset(get = "pub")]
pub(crate) struct UNamePars {
    symbol: String,
    singular: String,
    plural: String,
}
impl UNamePars {
    fn new(lang: String, arg: &UNameSer) -> UNamePars {
        Self {
            symbol: arg.symbol.clone(),
            singular: arg.names[&lang].singular().clone(),
            plural: arg.names[&lang].plural().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UNameSer {
    symbol: String,
    names: HashMap<String, UnitNameSer>,
}
impl UNameSer {
    fn get_name(&self, lang: &String) -> UNamePars {
        UNamePars::new(lang.clone(), self)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum TypeComposed {
    Derived(Vec<(Rc<UnitSer>, i8)>),
    Supplied(Vec<(Rc<UnitSer>, i8)>, UNameSer),
}
impl TypeComposed {
    fn get_name(&self, lang: &String) -> UNamePars {
        match self {
            TypeComposed::Derived(derived) => fully_expand_derived_name(derived.iter().cloned()),
            TypeComposed::Supplied(_, name) => name.get_name(lang),
        }
    }
}
#[derive(Clone)]
pub(crate) enum Steps {
    Fully,
    N(i8),
}

impl Iterator for Steps {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self.clone() {
            Steps::Fully => Some(Self::Fully),
            Steps::N(n) => {
                if n > 0 {
                    *self = Self::N(n - 1);
                    Some(Self::N(n - 1))
                } else {
                    None
                }
            }
        }
    }
}

fn fully_expand_derived_name<T>(derived: T) -> UNamePars
where
    T: Iterator<Item = (Rc<UnitSer>, i8)>,
{
    expand_derived_name(derived, Steps::Fully)
}

fn expand_derived_name<T>(derived: T, fully: Steps) -> UNamePars
where
    T: Iterator<Item = (Rc<UnitSer>, i8)>,
{
    todo!()
}

#[derive(Clone)]
pub(crate) struct BaseUNamePowSer {
    pub(crate) name: Ident,
    pub(crate) pow: i8,
}

impl UnitSer {
    fn new(
        unit: UnitSerSerOrd,
        prefix: HashMap<String, PrefixGroup>,
        other_units: Rc<RefCell<HashMap<String, Rc<UnitSer>>>>,
        quantity: QuantitySer
    ) -> Self {
        Self {
            quantity,
            conversions: unit
                .conversions
                .unwrap_or_default()
                .iter()
                .map(|(key, value)| (key.clone(), value.into()))
                .collect(),
            prefixes: unit
                .derive_prefixes
                .unwrap_or_default()
                .iter()
                .map(|group_name| prefix[group_name].0.clone())
                .flatten()
                .collect(),
            description: unit.description,
            ty: match unit.composite {
                Some(inherited_units) => {
                    UnitTypeSer::Composed(if !unit.inherit_names.unwrap_or(true) {
                        TypeComposed::Supplied(
                            inherited_units
                                .iter()
                                .map(|(key, value)| (Rc::clone(&other_units.borrow()[key]), *value))
                                .collect_vec(),
                            UNameSer {
                                symbol: unit.symbol,
                                names: unit
                                    .names
                                    .unwrap()
                                    .iter()
                                    .map(|(key, value)| (key.clone(), value.clone().into()))
                                    .collect(),
                            },
                        )
                    } else {
                        TypeComposed::Derived(
                            inherited_units
                                .iter()
                                .map(|(key, value)| (Rc::clone(&other_units.borrow()[key]), *value))
                                .collect_vec(),
                        )
                    })
                }
                // TODO Errorreporting
                None => UnitTypeSer::Simple(UNameSer {
                    symbol: unit.symbol,
                    names: unit
                        .names
                        .unwrap()
                        .iter()
                        .map(|(key, value)| (key.clone(), value.clone().into()))
                        .collect(),
                }),
            },
        }
    }

    pub fn get_name(&self, lang: &String) -> UNamePars {
        self.ty.get_name(lang)
    }

    pub(crate) fn base_units_sorted(self, lang: String) -> Vec<BaseUNamePowSer> {
        match self.ty.clone() {
            UnitTypeSer::Simple(simple) => [BaseUNamePowSer {
                name: str_to_ident(simple.names[&lang].singular.clone().to_case(Case::UpperCamel)).clone(),
                pow: 1,
            }]
            .iter()
            .cloned()
            .collect(),
            UnitTypeSer::Composed(composed) => match composed {
                TypeComposed::Derived(iter) => iter,
                TypeComposed::Supplied(iter, _) => iter,
            }
            .iter()
            .map(|(unit, pow)| BaseUNamePowSer {
                name: str_to_ident(unit.get_name(&lang).singular.to_case(Case::UpperCamel)),
                pow: *pow,
            }).collect(),
        }
    }

    // fn new(unit_serser: UnitSerSer, base_units: Vec<UnitSerSer>) -> Self{
    //     Self {
    //         symbol: unit_serser.symbol,
    //         prefixes: unit_serser.prefixes,
    //         conversions: unit_serser.conversions,
    //         composite: unit_serser.composite.map(|(_, map)| map).map(|composite|{
    //             composite.iter().map(|(u_name, pow)|{
    //                 let unit = base_units.iter().find(|UnitSerSer{file_name_wo_ending, ..}| file_name_wo_ending).unwrap();
    //                 (*unit, *pow)
    //             }).collect()
    //         }).unwrap_or(Vec::new()),
    //         names: match unit_serser.names{
    //             UnitSerNames::Supplied(names) => names,
    //             UnitSerNames::Derive(derived) => {

    //                 let num_str = generate_string_repr(num);
    //                 let denom_str = generate_string_repr(denom);
    //                 let whole_str = format!("{} per {}", num_str, denom_str);
    //                 // higher powers will be named first

    //                 todo!()
    //             },
    //         },
    //     }
    //}
    // / base_units do not need to be base units. They can however be some
    // / returns names of supplied only though
    // fn derived_to_supplied_only(
    //     derived: HashMap<String, i8>,
    //     base_units: Vec<UnitSerSer>,
    // ) -> (HashMap<String, UnitNameSer>, i8) {
    //     // LEFT: Top
    //     // Right: Bottom
    //     let (num, denom): (Vec<_>, Vec<_>) = derived.iter().partition_map(|(name, pow)| {
    //         let u_name = base_units
    //             .iter()
    //             .find(
    //                 |UnitSerSer {
    //                      file_name_wo_ending,
    //                      ..
    //                  }| file_name_wo_ending == name,
    //             )
    //             .map(|UnitSerSer { names, .. }| names)
    //             .unwrap();
    //         if *pow > 0 {
    //             Either::Left((u_name, pow))
    //         } else {
    //             Either::Right((u_name, pow))
    //         }
    //     });
    //     let nums_finished = num.iter().map(|(name, pow)| match name {
    //         UnitSerNames::Supplied(sup) => (sup, pow),
    //         UnitSerNames::Derive(der) => todo!(),
    //     });

    //     let denoms_finished = denom.iter().map(|(name, pow)| match name {
    //         UnitSerNames::Supplied(sup) => sup,
    //         UnitSerNames::Derive(der) => todo!(),
    //     });
    //     todo!()
    // }
}

///
fn generate_string_repr(
    unit_pow: Vec<(HashMap<String, UnitNameSer>, NonZeroI8)>,
) -> HashMap<String, UnitNameSer> {
    // // UnitSerSer -> lang
    // // .1 -> pow
    // let first = unit_pow.first();
    // let all_langs_intersect = unit_pow
    //     .iter()
    //     .filter(|elem| elem.0.keys().all(|key| key.contains(first)))
    //     .chain((first).iter());
    // let (num, denom): (Vec<_>, Vec<_>) = unit_pow.iter().partition_map(|(unit, pow)| {
    //     if pow.get().is_positive() {
    //         Either::Left((unit, pow))
    //     } else {
    //         Either::Right((unit, pow))
    //     }
    // });
    todo!()
}

#[derive(Clone, Debug)]
pub(crate) struct UnitSerSer {
    //EMITTING .toml
    pub(crate) file_name_wo_ending: String,
    pub(crate) symbol: String,
    pub(crate) names: UnitSerNames,
    pub(crate) prefixes: Vec<PrefixSer>,
    pub(crate) conversions: HashMap<String, ConversionSer>,
    // bool = inherit_names -> only possible in composite units
    pub(crate) composite: Option<(bool, HashMap<String, i8>)>,
}
#[derive(Clone, Debug)]
pub enum UnitSerNames {
    // Lang -> Name
    Supplied(HashMap<String, UnitNameSer>),
    // Name of toml file -> pow
    Derive(HashMap<String, i8>),
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

impl UnitSerSer {
    // prefix_map: example: ("bin", "peta, exa, tera")
    pub(crate) fn new(
        file_name_wo_ending: String,
        unit: UnitSerSerOrd,
        prefix_map: HashMap<String, PrefixGroup>,
    ) -> UnitSerSer {
        let UnitSerSerOrd{ symbol, description, names, derive_prefixes, conversions, inherit_names, composite } = unit;
        UnitSerSer {
            file_name_wo_ending,
            symbol: symbol,
            names: match composite.clone() {
                Some(composite) => {
                    if inherit_names.unwrap_or(true) {
                        UnitSerNames::Derive(composite.iter().map(|(k,v)|(k.clone(), v.clone())).collect())
                    } else {
                        UnitSerNames::Supplied(names.expect("you must supply [names] if you deactivate inheritance (active by default)").iter().map(|(lang, UnitNameSerSer { inner })|{
                            (lang.clone(), inner.clone().into())
                        }).collect())
                    }
                }
                None => UnitSerNames::Supplied(
                    names
                        .expect("you either need to supply [composite] or  [names]")
                        .iter()
                        .map(|(lang, UnitNameSerSer { inner })| {
                            (lang.clone(), inner.clone().into())
                        })
                        .collect(),
                ),
            },
            // unit
            //     .names
            //     .iter()
            //     .map(|(key, value)| (key.clone(), value.clone().into()))
            //     .collect::<HashMap<String, UnitNameSer>>(),
            prefixes: {
                derive_prefixes
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
            conversions: 
                conversions
                .unwrap_or_default()
                .iter()
                .map(|(key, value)| (key.clone(), value.into()))
                .collect::<HashMap<String, ConversionSer>>(), //.into(),
            composite: composite.map(|composite| {
                (
                    inherit_names.unwrap_or(true),
                    composite.iter().map(|(k,v)|(k.clone(), v.clone())).collect(),
                )
            }),
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

impl From<Either<String, (String, String)>> for UnitNameSer {
    fn from(value: Either<String, (String, String)>) -> Self {
        match value {
            Either::Left(singular) => Self {
                singular: singular.clone(),
                plural: singular,
            },
            Either::Right((singular, plural)) => Self { singular, plural },
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

#[derive(Clone, Debug, CopyGetters, Getters)]
#[getset(get = "pub")]
pub(crate) struct UnitNameSer {
    pub(crate) singular: String,
    pub(crate) plural: String,
}
