use const_units_global_types::{Factor, RatioConst};

use either::Either;
use serde::{Deserialize, Serialize};

use super::{parse_to_int, parse_to_int_pow};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct FactorSer {
    #[serde(with = "either::serde_untagged")]
    pub(crate) inner: Either<f64, String>,
}

impl FactorSer {
    pub fn new<T: ToString>(expr: T) -> Self {
        Self {
            inner: Either::Right(expr.to_string()),
        }
    }
}

impl Into<Factor> for FactorSer {
    fn into(self) -> Factor {
        match self.inner {
            Either::Left(float) => Factor::Float(const_traits::Into::into(float)),
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
