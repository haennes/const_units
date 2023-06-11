#![feature(prelude_import)]
#![feature(iterator_try_collect, adt_const_params, const_trait_impl)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod code_gen {
    pub(crate) mod dimension {
        use std::{collections::HashMap, fs, path::Path, str::FromStr};
        use proc_macro::TokenStream;
        const DIMENSIONS_PATH: &str = "dimensions.toml";
        const DIMENSIONS_DEF: &str = "";
        pub(crate) type Dimensions = HashMap<String, String>;
        pub(crate) fn parse_dimensions(systempath: &Path) -> Dimensions {
            let path = systempath.join(DIMENSIONS_PATH);
            let contents = fs::read_to_string(path.clone())
                .expect(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("failed to read file {0}", path.display()),
                        );
                        res
                    },
                );
            toml::de::from_str(&contents)
                .expect(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "failed to parse file {0} \n contents: {1}", path.display(),
                                contents
                            ),
                        );
                        res
                    },
                )
        }
        pub(crate) fn generate_dimensions(
            systemname: String,
            dimensions: Dimensions,
        ) -> TokenStream {
            let fields: String = dimensions
                .iter()
                .map(|(name, description)| {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("///{0} \n {1}: i8, \n", description, name),
                        );
                        res
                    }
                })
                .collect();
            let mut out = {
                let res = ::alloc::fmt::format(
                    format_args!("pub struct {0} {{ \n {1} \n }};", systemname, fields),
                );
                res
            };
            out += "\n";
            let fields_zero: String = dimensions
                .iter()
                .map(|(name, _)| {
                    let res = ::alloc::fmt::format(format_args!("{0} = 0 \n", name));
                    res
                })
                .collect();
            out
                += &{
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "pub const NONE: {0} {{ \n {1} \n }}", systemname,
                            fields_zero
                        ),
                    );
                    res
                };
            out += "\n";
            out
                += &{
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "impl const Default for {0} {{ \n fn default() -> Self {{ NONE }} \n }}",
                            systemname
                        ),
                    );
                    res
                };
            TokenStream::from_str(&out)
                .expect(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "formatting error while building TokenStream: {0}", out
                            ),
                        );
                        res
                    },
                )
        }
    }
    pub(crate) mod factor {
        use num_rational::Ratio;
        use serde::{Deserialize, Serialize};
        pub enum FactorSer {
            Ratio(String),
            Float(f64),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for FactorSer {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        FactorSer::Ratio(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "FactorSer",
                                0u32,
                                "Ratio",
                                __field0,
                            )
                        }
                        FactorSer::Float(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "FactorSer",
                                1u32,
                                "Float",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for FactorSer {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Ratio" => _serde::__private::Ok(__Field::__field0),
                                "Float" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Ratio" => _serde::__private::Ok(__Field::__field0),
                                b"Float" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<FactorSer>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = FactorSer;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum FactorSer",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match match _serde::de::EnumAccess::variant(__data) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            String,
                                        >(__variant),
                                        FactorSer::Ratio,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            f64,
                                        >(__variant),
                                        FactorSer::Float,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Ratio", "Float"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "FactorSer",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<FactorSer>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for FactorSer {
            #[inline]
            fn clone(&self) -> FactorSer {
                match self {
                    FactorSer::Ratio(__self_0) => {
                        FactorSer::Ratio(::core::clone::Clone::clone(__self_0))
                    }
                    FactorSer::Float(__self_0) => {
                        FactorSer::Float(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        pub enum Factor {
            Ratio(Ratio<i128>),
            Float(f64),
        }
        impl Into<Factor> for FactorSer {
            fn into(self) -> Factor {
                match self {
                    FactorSer::Ratio(str) => {
                        Factor::Ratio({
                            match str.split_once("/") {
                                Some((num, denom)) => {
                                    let (num, denom): (i128, i128) = (
                                        parse_to_int_pow(num),
                                        parse_to_int_pow(denom),
                                    );
                                    Ratio::new(num, denom)
                                }
                                None => Ratio::new(parse_to_int(&str), 1),
                            }
                        })
                    }
                    FactorSer::Float(f) => Factor::Float(f),
                }
            }
        }
        fn parse_to_int_pow(num: &str) -> i128 {
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
                .expect(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("failed to parse {0} to a string", num),
                        );
                        res
                    },
                )
        }
    }
    pub(crate) mod prefix {
        use super::factor::{Factor, FactorSer};
        use serde::{Deserialize, Serialize};
        use std::{fs::File, io::Read, path::Path};
        const PREFIXES_PATH: &str = "prefixes";
        pub struct PrefixSer {
            pub alias: Option<String>,
            pub factor: FactorSer,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for PrefixSer {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "PrefixSer",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "alias",
                        &self.alias,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "factor",
                        &self.factor,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PrefixSer {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "alias" => _serde::__private::Ok(__Field::__field0),
                                "factor" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"alias" => _serde::__private::Ok(__Field::__field0),
                                b"factor" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<PrefixSer>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PrefixSer;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct PrefixSer",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct PrefixSer with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                FactorSer,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct PrefixSer with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(PrefixSer {
                                alias: __field0,
                                factor: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<FactorSer> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("alias"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("factor"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                FactorSer,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("alias") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("factor") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(PrefixSer {
                                alias: __field0,
                                factor: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["alias", "factor"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "PrefixSer",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<PrefixSer>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
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
                .expect(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("wrong prefixes-dir: {0}", folder.display()),
                        );
                        res
                    },
                )
                .into_iter()
                .filter_map(|file| {
                    let path = file
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("could not read {0}", folder.display()),
                                );
                                res
                            },
                        )
                        .path();
                    if path.extension().unwrap_or_default() != ".toml" {
                        None
                    } else {
                        let mut file = File::open(path.clone())
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!("could not open {0}", path.display()),
                                    );
                                    res
                                },
                            );
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!("could not read file {0}", path.display()),
                                    );
                                    res
                                },
                            );
                        Some(
                            PrefixSer::into(
                                toml::de::from_str(&contents)
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "could not desiralize file {0} \n with these contents \n {1}",
                                                    path.display(), contents
                                                ),
                                            );
                                            res
                                        },
                                    ),
                            ),
                        )
                    }
                })
                .collect()
        }
    }
    pub(crate) use dimension::*;
    pub(crate) use factor::*;
    pub(crate) use prefix::*;
}
mod generated {
    pub(crate) mod quantity_from_name {
        use crate::global_types::{quantity::Quantity, QName};
        impl Quantity {
            pub const fn from_name(name: QName) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
}
mod global_types {
    use self::quantity::QuantityDataTraits;
    use crate::generated::quantity_from_name;
    use const_units_macros::{AddingMul, Mul, Neg};
    use num_traits::{Num, NumAssign, NumAssignRef, One};
    use parse_display::{Display, FromStr};
    use quote::ToTokens;
    use self_rust_tokenize::SelfRustTokenize;
    use std::{fmt::Display, marker::PhantomData, ops::{self, Div, Mul, Neg}};
    pub(crate) mod quantity {
        use std;
        use std::fmt::Display;
        use std::ops::Div;
        use std::ops::Mul;
        use num_traits::NumAssignRef;
        use num_traits::One;
        use self_rust_tokenize::SelfRustTokenize;
        use super::Operation;
        use super::get_name_from_dimensions_and_op;
        use std::ops::Neg;
        use super::DimType;
        use super::QName;
        pub struct Quantity {
            pub(crate) name: QName,
            pub(crate) dimensions: DimType,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Quantity {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Quantity {
            #[inline]
            fn eq(&self, other: &Quantity) -> bool {
                self.name == other.name && self.dimensions == other.dimensions
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Quantity {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Quantity {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<QName>;
                let _: ::core::cmp::AssertParamIsEq<DimType>;
            }
        }
        #[automatically_derived]
        impl ::self_rust_tokenize::SelfRustTokenize for Quantity {
            fn append_to_token_stream(
                &self,
                token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
            ) {
                let Quantity { name: ref _0, dimensions: ref _1 } = self;
                ::self_rust_tokenize::_private::add_named_constructor_body(
                    token_stream,
                    &["Quantity"],
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            (
                                "name",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                            ),
                            (
                                "dimensions",
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_1),
                            ),
                        ]),
                    ),
                );
            }
        }
        impl Neg for Quantity {
            type Output = Self;
            fn neg(self) -> Self::Output {
                let result = self.dimensions.neg();
                Self {
                    name: get_name_from_dimensions_and_op(
                        result,
                        Operation::Neg(self.name),
                    ),
                    dimensions: result,
                }
            }
        }
        impl Mul for Quantity {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let result = self.dimensions.mul(rhs.dimensions);
                Self {
                    name: get_name_from_dimensions_and_op(
                        result,
                        Operation::Mul(self.name, rhs.name),
                    ),
                    dimensions: result,
                }
            }
        }
        impl Div for Quantity {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let result = self.dimensions.div(rhs.dimensions);
                Self {
                    name: get_name_from_dimensions_and_op(
                        result,
                        Operation::Div(self.name, rhs.name),
                    ),
                    dimensions: result,
                }
            }
        }
        impl Display for Quantity {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    f.write_fmt(format_args!("{0} ({1})", self.name, self.dimensions))
                } else {
                    f.write_fmt(format_args!("{0}", self.name))
                }
            }
        }
        pub trait QuantityDataTraits: NumAssignRef + One + Copy {}
        impl<DT> QuantityDataTraits for DT
        where
            DT: NumAssignRef + One + Display + Copy,
        {}
    }
    enum Operation {
        Mul(QName, QName),
        Div(QName, QName),
        Neg(QName),
    }
    pub struct SiExtended_struct {
        a: i16,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SiExtended_struct {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SiExtended_struct {
        #[inline]
        fn eq(&self, other: &SiExtended_struct) -> bool {
            self.a == other.a
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SiExtended_struct {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SiExtended_struct {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i16>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display for SiExtended_struct {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            f.write_fmt(format_args!("{0}", self.a))
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for SiExtended_struct {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            let SiExtended_struct { a: ref _0 } = self;
            ::self_rust_tokenize::_private::add_named_constructor_body(
                token_stream,
                &["SiExtended_struct"],
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ("a", ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0)),
                    ]),
                ),
            );
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SiExtended_struct {
        #[inline]
        fn clone(&self) -> SiExtended_struct {
            let _: ::core::clone::AssertParamIsClone<i16>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SiExtended_struct {}
    impl Neg for SiExtended_struct {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self { a: -self.a }
        }
    }
    impl Mul for SiExtended_struct {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self { a: self.a + rhs.a }
        }
    }
    #[display("{}")]
    pub enum DimType {
        SiExtended(SiExtended_struct),
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DimType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DimType {
        #[inline]
        fn eq(&self, other: &DimType) -> bool {
            match (self, other) {
                (DimType::SiExtended(__self_0), DimType::SiExtended(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DimType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DimType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<SiExtended_struct>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display for DimType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                &Self::SiExtended(ref _value_0) => {
                    f.write_fmt(format_args!("{0}", "SiExtended"))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for DimType {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            match self {
                DimType::SiExtended(ref _0) => {
                    ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                        token_stream,
                        &["DimType", "SiExtended"],
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::self_rust_tokenize::SelfRustTokenize::to_tokens(_0),
                            ]),
                        ),
                    );
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DimType {
        #[inline]
        fn clone(&self) -> DimType {
            let _: ::core::clone::AssertParamIsClone<SiExtended_struct>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DimType {}
    impl Neg for DimType {
        type Output = Self;
        fn neg(self) -> Self::Output {
            match self {
                DimType::SiExtended(arg0) => DimType::SiExtended(-arg0),
            }
        }
    }
    impl Mul for DimType {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            match self {
                DimType::SiExtended(a_arg0) => {
                    if let DimType::SiExtended(b_arg0) = rhs {
                        DimType::SiExtended {
                            0: a_arg0 * b_arg0,
                        }
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("can only multiply if the DimType is the same"),
                            );
                        }
                    }
                }
            }
        }
    }
    impl Div for DimType {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub(crate) enum QName {
        Velocity,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for QName {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for QName {
        #[inline]
        fn eq(&self, other: &QName) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for QName {}
    #[automatically_derived]
    impl ::core::cmp::Eq for QName {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::fmt::Display for QName {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                &Self::Velocity => f.write_fmt(format_args!("{0}", "Velocity")),
            }
        }
    }
    #[automatically_derived]
    impl ::self_rust_tokenize::SelfRustTokenize for QName {
        fn append_to_token_stream(
            &self,
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream,
        ) {
            match self {
                QName::Velocity => {
                    ::self_rust_tokenize::_private::add_unit_constructor_body(
                        token_stream,
                        &["QName", "Velocity"],
                    );
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::str::FromStr for QName {
        type Err = ::parse_display::ParseError;
        fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
            match s {
                "Velocity" => {
                    return ::core::result::Result::Ok(Self::Velocity);
                }
                _ => {}
            }
            ::core::result::Result::Err(::parse_display::ParseError::new())
        }
    }
    pub type Velocity<DT> = Unit<DT, { quantity::Quantity::from_name(QName::Velocity) }>;
    pub struct Unit<
        StorageDt: QuantityDataTraits,
        const QUANTITY: quantity::Quantity,
        const INITIALIZED: bool = false,
    > {
        value: StorageDt,
    }
    impl<
        StorageDt: QuantityDataTraits,
        const QUANTITY: quantity::Quantity,
    > Unit<StorageDt, QUANTITY, false>
    where
        StorageDt: QuantityDataTraits,
    {
        pub const fn new() -> Self {
            Self { value: StorageDt::one() }
        }
    }
    impl<
        StorageDt: QuantityDataTraits,
        const QUANTITY: quantity::Quantity,
        const INITIALIZED: bool,
    > Unit<StorageDt, QUANTITY, INITIALIZED>
    where
        StorageDt: QuantityDataTraits,
    {
        /// declared as unsafe as you are leaving the dimension-checking realm
        pub const unsafe fn raw_value(&self) -> StorageDt {
            self.value
        }
    }
    fn get_name_from_dimensions_and_op(result: DimType, neg: Operation) -> QName {
        ::core::panicking::panic("not yet implemented")
    }
}
mod quantities {
    use std::{collections::HashMap, fs::File, io::Read, path::Path, str::FromStr};
    use proc_macro::TokenStream;
    use serde::{Deserialize, Serialize};
    const QUANTITIES_PATH: &str = "quantities";
    const QUANTITIES_FILE_NAME: &str = "quantity.toml";
    pub(crate) struct QuantitySer {
        name: String,
        description: Option<String>,
        reference_unit: String,
        dimension: HashMap<String, i8>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for QuantitySer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "QuantitySer",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "reference_unit",
                    &self.reference_unit,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dimension",
                    &self.dimension,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for QuantitySer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "description" => _serde::__private::Ok(__Field::__field1),
                            "reference_unit" => _serde::__private::Ok(__Field::__field2),
                            "dimension" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"description" => _serde::__private::Ok(__Field::__field1),
                            b"reference_unit" => _serde::__private::Ok(__Field::__field2),
                            b"dimension" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<QuantitySer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = QuantitySer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct QuantitySer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            HashMap<String, i8>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct QuantitySer with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(QuantitySer {
                            name: __field0,
                            description: __field1,
                            reference_unit: __field2,
                            dimension: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            HashMap<String, i8>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "reference_unit",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "dimension",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            HashMap<String, i8>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("description") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "reference_unit",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("dimension") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(QuantitySer {
                            name: __field0,
                            description: __field1,
                            reference_unit: __field2,
                            dimension: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "name",
                    "description",
                    "reference_unit",
                    "dimension",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "QuantitySer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<QuantitySer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub(crate) fn parse_quantities(systempath: &Path) -> Vec<QuantitySer> {
        let path = systempath.join(QUANTITIES_PATH);
        path.read_dir()
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("failed to open {0}", path.display()),
                    );
                    res
                },
            )
            .into_iter()
            .filter_map(|folder_or_file| {
                let folder_or_file = folder_or_file.expect("could not  read folder");
                if folder_or_file.path().is_dir() {
                    let mut children = folder_or_file
                        .path()
                        .read_dir()
                        .expect(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "could not read folder {0}", folder_or_file.path().display()
                                    ),
                                );
                                res
                            },
                        );
                    let children_names = children
                        .find_map(|readdir| {
                            let readdir = readdir
                                .expect(
                                    &{
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "could not read child of {0}", folder_or_file.path()
                                                .display()
                                            ),
                                        );
                                        res
                                    },
                                );
                            if readdir.path().ends_with(QUANTITIES_FILE_NAME) {
                                Some(readdir)
                            } else {
                                None
                            }
                        });
                    Some(children_names.unwrap())
                } else {
                    None
                }
            })
            .map(|quantity| {
                let mut file = File::open(quantity.path())
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "could not open file: {0}", quantity.path().display()
                                ),
                            );
                            res
                        },
                    );
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "could not read from: {0}", quantity.path().display()
                                ),
                            );
                            res
                        },
                    );
                toml::de::from_str(&contents)
                    .expect(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "could not parse file: {0} \n contents: {1}", quantity
                                    .path().display(), contents
                                ),
                            );
                            res
                        },
                    )
            })
            .collect()
    }
    pub(crate) fn generate_quantities(
        input: Vec<QuantitySer>,
        systemname: String,
    ) -> TokenStream {
        let all = input.iter().map(|q| generate_quantity(&systemname, q));
        TokenStream::from_iter(all)
    }
    fn generate_quantity(systemname: &String, quantity: &QuantitySer) -> TokenStream {
        let mut string = String::new();
        let mut optional_description = "".to_string();
        if let Some(description) = &quantity.description {
            optional_description += &("/**".to_owned() + &description + "*/");
            optional_description += "\n";
        }
        let fields: String = quantity
            .dimension
            .iter()
            .map(|(dimension_name, power)| {
                {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}:{1}, ", dimension_name, power),
                    );
                    res
                }
            })
            .collect();
        string
            += &{
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0} pub const {1}_dim: {2} = {2}{{ {3} }}",
                        optional_description, quantity.name, systemname, fields
                    ),
                );
                res
            };
        TokenStream::from_str(&string)
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!("failed to parse string {0}", string),
                    );
                    res
                },
            )
    }
}
mod unit {
    use std::collections::HashMap;
    use crate::global_types::QName;
    use itertools::Itertools;
    use num_rational::Ratio;
    use proc_macro2::TokenStream;
    use self_rust_tokenize::SelfRustTokenize;
    use serde::{Deserialize, Serialize};
    use std::{fs::File, io::Read, path::Path};
    use crate::{
        code_gen::factor::{parse_to_int, Factor, FactorSer},
        global_types::quantity::Quantity, quantities::QuantitySer,
    };
    const UNIT_PATH: &str = "units";
    pub(crate) struct UnitSerSer {
        symbol: String,
        names: HashMap<String, UnitNameSerSer>,
        derive_prefixes: Vec<String>,
        conversions: HashMap<String, ConversionSerSer>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UnitSerSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UnitSerSer",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "symbol",
                    &self.symbol,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "names",
                    &self.names,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "derive_prefixes",
                    &self.derive_prefixes,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "conversions",
                    &self.conversions,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UnitSerSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "symbol" => _serde::__private::Ok(__Field::__field0),
                            "names" => _serde::__private::Ok(__Field::__field1),
                            "derive_prefixes" => _serde::__private::Ok(__Field::__field2),
                            "conversions" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"symbol" => _serde::__private::Ok(__Field::__field0),
                            b"names" => _serde::__private::Ok(__Field::__field1),
                            b"derive_prefixes" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            b"conversions" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UnitSerSer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UnitSerSer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UnitSerSer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            HashMap<String, UnitNameSerSer>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            HashMap<String, ConversionSerSer>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UnitSerSer with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UnitSerSer {
                            symbol: __field0,
                            names: __field1,
                            derive_prefixes: __field2,
                            conversions: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            HashMap<String, UnitNameSerSer>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            HashMap<String, ConversionSerSer>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("symbol"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("names"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            HashMap<String, UnitNameSerSer>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "derive_prefixes",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "conversions",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            HashMap<String, ConversionSerSer>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("symbol") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("names") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "derive_prefixes",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("conversions") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(UnitSerSer {
                            symbol: __field0,
                            names: __field1,
                            derive_prefixes: __field2,
                            conversions: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "symbol",
                    "names",
                    "derive_prefixes",
                    "conversions",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UnitSerSer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UnitSerSer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub(crate) struct ConversionSerSer {
        factor: FactorSer,
        accuracy: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ConversionSerSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ConversionSerSer",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "factor",
                    &self.factor,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "accuracy",
                    &self.accuracy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ConversionSerSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "factor" => _serde::__private::Ok(__Field::__field0),
                            "accuracy" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"factor" => _serde::__private::Ok(__Field::__field0),
                            b"accuracy" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ConversionSerSer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ConversionSerSer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ConversionSerSer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            FactorSer,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ConversionSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ConversionSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ConversionSerSer {
                            factor: __field0,
                            accuracy: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<FactorSer> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("factor"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            FactorSer,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "accuracy",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("factor") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("accuracy") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ConversionSerSer {
                            factor: __field0,
                            accuracy: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["factor", "accuracy"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ConversionSerSer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ConversionSerSer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub(crate) struct UnitNameSerSer {
        singular: String,
        plural: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UnitNameSerSer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "UnitNameSerSer",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "singular",
                    &self.singular,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "plural",
                    &self.plural,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UnitNameSerSer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "singular" => _serde::__private::Ok(__Field::__field0),
                            "plural" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"singular" => _serde::__private::Ok(__Field::__field0),
                            b"plural" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UnitNameSerSer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UnitNameSerSer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UnitNameSerSer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UnitNameSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UnitNameSerSer with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UnitNameSerSer {
                            singular: __field0,
                            plural: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "singular",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("plural"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("singular") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("plural") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(UnitNameSerSer {
                            singular: __field0,
                            plural: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["singular", "plural"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UnitNameSerSer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UnitNameSerSer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for UnitNameSerSer {
        #[inline]
        fn clone(&self) -> UnitNameSerSer {
            UnitNameSerSer {
                singular: ::core::clone::Clone::clone(&self.singular),
                plural: ::core::clone::Clone::clone(&self.plural),
            }
        }
    }
    pub(crate) struct UnitSer {
        symbol: String,
        names: HashMap<String, UnitNameSer>,
        derive_prefixes: Vec<String>,
        conversions: HashMap<String, ConversionSer>,
    }
    pub(crate) struct ConversionSer {
        factor: Factor,
        accuracy: AccuracySer,
    }
    pub(crate) struct UnitNameSer {
        singular: String,
        plural: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnitNameSer {
        #[inline]
        fn clone(&self) -> UnitNameSer {
            UnitNameSer {
                singular: ::core::clone::Clone::clone(&self.singular),
                plural: ::core::clone::Clone::clone(&self.plural),
            }
        }
    }
    impl IntoIterator for UnitNameSer {
        type Item = String;
        type IntoIter = std::vec::IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            if self.singular == self.plural {
                <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([self.singular]))
                    .into_iter()
            } else {
                <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([self.singular, self.plural]),
                    )
                    .into_iter()
            }
        }
    }
    pub(crate) enum AccuracySer {
        Exact,
        Inaccurate(i32),
    }
    impl Into<UnitSer> for UnitSerSer {
        fn into(self) -> UnitSer {
            UnitSer {
                symbol: self.symbol,
                names: self
                    .names
                    .iter()
                    .map(|(key, value)| (key.clone(), (value.clone()).into()))
                    .collect::<HashMap<String, UnitNameSer>>(),
                derive_prefixes: self.derive_prefixes,
                conversions: self
                    .conversions
                    .iter()
                    .map(|(key, value)| (key.clone(), (value.clone()).into()))
                    .collect::<HashMap<String, ConversionSer>>(),
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
    impl Into<UnitNameSer> for UnitNameSerSer {
        fn into(self) -> UnitNameSer {
            UnitNameSer {
                singular: self.singular.clone(),
                plural: {
                    match self.plural {
                        Some(plural) => plural,
                        None => self.singular.clone(),
                    }
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
    pub(crate) fn generate_unit_code_name(
        q_name: String,
        name: UnitNameSer,
        data_type: String,
        dim_type: String,
    ) -> TokenStream {
        let singular = name.singular;
        let q_const = Quantity::from_name(q_name.parse().expect("UnitName not found"))
            .to_tokens();
        {
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::__private::push_ident(&mut _s, "pub");
            ::quote::__private::push_ident(&mut _s, "const");
            ::quote::ToTokens::to_tokens(&singular, &mut _s);
            ::quote::__private::push_colon(&mut _s);
            ::quote::__private::push_ident(&mut _s, "Unit");
            ::quote::__private::push_lt(&mut _s);
            ::quote::ToTokens::to_tokens(&data_type, &mut _s);
            ::quote::__private::push_comma(&mut _s);
            ::quote::ToTokens::to_tokens(&q_name, &mut _s);
            ::quote::__private::push_comma(&mut _s);
            ::quote::ToTokens::to_tokens(&q_const, &mut _s);
            ::quote::__private::push_gt(&mut _s);
            ::quote::__private::push_eq(&mut _s);
            ::quote::__private::push_ident(&mut _s, "Unit");
            ::quote::__private::push_colon2(&mut _s);
            ::quote::__private::push_ident(&mut _s, "new");
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Parenthesis,
                ::quote::__private::TokenStream::new(),
            );
            ::quote::__private::push_semi(&mut _s);
            _s
        }
    }
    pub(crate) fn generate_unit_code_lang(
        q_name: String,
        unit: &UnitSer,
        language: &String,
        data_type: String,
        dim_type: String,
    ) -> TokenStream {
        generate_unit_code_name(
            q_name,
            unit.names[language].clone(),
            data_type,
            dim_type,
        )
    }
    pub(crate) fn generate_unit_code_symbol(
        unit: UnitSer,
        data_type: String,
        dim_type: String,
        q_name: String,
    ) -> TokenStream {
        generate_unit_code_name(q_name, unit.symbol.into(), data_type, dim_type)
    }
    pub(crate) fn generate_units(
        languages: Vec<String>,
        units: Vec<UnitSer>,
        data_type: String,
        dim_type: String,
        q_name: String,
    ) -> TokenStream {
        units
            .iter()
            .map(|unit| -> TokenStream {
                languages
                    .iter()
                    .map(|language| {
                        generate_unit_code_lang(
                            q_name.clone(),
                            unit,
                            language,
                            data_type.clone(),
                            dim_type.clone(),
                        )
                    })
                    .collect()
            })
            .collect()
    }
}
mod uuse {
    use proc_macro::TokenStream;
    use syn::{parse::Parse, Error, Ident, Token};
    pub(crate) struct UUse(Vec<UnitImpl>);
    pub(crate) struct UnitImpl {
        name: String,
        data_type: String,
    }
    pub(crate) fn generate_uuse(uuse: UUse) -> TokenStream {
        uuse.0.iter().map(|unit_impl| generate_unit(unit_impl)).collect()
    }
    fn generate_unit(unit_impl: &UnitImpl) -> TokenStream {
        ::core::panicking::panic("not yet implemented")
    }
    impl Parse for UUse {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let mut idents = Vec::new();
            idents.push(input.parse::<Ident>()?);
            let mut lookahead = input.lookahead1();
            while lookahead.peek(::syn::token::Comma) {
                input.parse::<::syn::token::Comma>()?;
                idents.push(input.parse()?);
                lookahead = input.lookahead1();
            }
            Ok(UUse {
                0: idents
                    .iter()
                    .map(|ident| match ident.to_string().split_once("_") {
                        Some((name, data_type)) => {
                            Ok(UnitImpl {
                                name: name.to_owned(),
                                data_type: data_type.to_owned(),
                            })
                        }
                        None => return Err(Error::new(ident.span(), "_ not found")),
                    })
                    .try_collect()?,
            })
        }
    }
}
pub(crate) use code_gen::*;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, parse_macro_input, DeriveInput, Ident, LitStr, Token};
use uuse::{generate_uuse, UUse};
#[proc_macro]
pub fn uuse(ts: TokenStream) -> TokenStream {
    let uuse = match ::syn::parse::<UUse>(ts) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    generate_uuse(uuse)
}
const _: () = {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    #[used]
    #[allow(deprecated)]
    static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[
        proc_macro::bridge::client::ProcMacro::bang("uuse", uuse),
    ];
};
