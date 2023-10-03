use super::super::MAXIMUM_BASEUNITS;
use crate::BaseUName;
use const_ops::{Div, Mul, Neg};
use core::marker::ConstParamTy;
use cvec::CVec;
use std::{fmt::Display, io::empty};

#[derive(PartialEq, Eq, ConstParamTy)]
pub struct UName(CVec<BaseUNamePow, MAXIMUM_BASEUNITS>);

// impl UName {
//     pub const fn neg(self) -> Self {
//         self.neg()
//     }
// }

impl UName {
    pub const fn new() -> Self {
        Self(CVec::empty())
    }

    pub const fn new_arr<const L: usize>(names: [BaseUNamePow; L]) -> Self {
        let mut out = Self::new();
        let mut idx = 0;
        while idx < L {
            out.insert(names[idx]);
            idx += 1;
        }
        out
    }

    pub const fn insert(&mut self, item: BaseUNamePow) {
        let mut idx = 0;
        while idx < MAXIMUM_BASEUNITS {
            if let Some(item_self) = self.0.get(idx) {
                if item_self.name == item.name {
                    let mut item_self = self.0.remove(idx);
                    item_self.pow += item.pow;
                    self.0.insert(item_self);
                    return;
                }
            }
            idx += 1;
        }
        // item with the same name not found.
        self.0.insert(item);
    }
}

//UName*UName
impl const Mul for UName {
    type Output = UName;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut self_cp = &self;
        let mut output = Self::new();
        loop {
            let (elem, rest) = rhs.0.poped();
            match elem {
                Some(elem) => self_cp.insert(elem),
                None => break,
            }
        }
        output
    }
}

//UName*UName
impl const Div for UName {
    type Output = UName;

    fn div(self, rhs: Self) -> Self::Output {
        let mut self_cp = &self;
        let mut output = Self::new();
        loop {
            let (elem, rest) = rhs.0.poped();
            match elem {
                Some(elem) => self_cp.insert(elem.inv()),
                None => break,
            }
        }
        output
    }
}

impl const Neg for UName {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut iter = self.0;
        let mut output = Self::new();
        let mut idx = 0;
        loop {
            let (elem, iter) = iter.poped();
            match elem {
                Some(elem) => output.insert(elem.neg()),
                None => break,
            }
            idx += 1;
        }
        output
    }
}

#[derive(PartialEq, Eq, ConstParamTy, Clone, Copy)]
pub struct BaseUNamePow {
    pow: i8,
    name: BaseUName,
}

impl BaseUNamePow {
    pub const fn inv(&self) -> Self {
        Self {
            pow: -self.pow,
            name: self.name,
        }
    }
}

impl const Neg for BaseUNamePow {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            pow: -self.pow,
            name: self.name,
        }
    }
}
