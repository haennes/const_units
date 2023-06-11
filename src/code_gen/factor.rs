use num_rational::Ratio;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum FactorSer {
    Ratio(String),
    Float(f64),
}

#[derive(Debug)]
pub enum Factor {
    Ratio(Ratio<i128>),
    Float(f64),
}

impl Into<Factor> for FactorSer {
    fn into(self) -> Factor {
        match self {
            FactorSer::Ratio(str) => Factor::Ratio({
                match str.split_once("/") {
                    Some((num, denom)) => {
                        let (num, denom): (i128, i128) =
                            (parse_to_int_pow(num), parse_to_int_pow(denom));
                        Ratio::new(num, denom)
                    }
                    None => Ratio::new(parse_to_int(&str), 1),
                }
            }),
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
        .expect(&format!("failed to parse {} to a string", num))
}
