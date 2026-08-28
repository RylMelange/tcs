use std::ops::{Add, Rem};

use crate::simulation::wire_values::TernaryType::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub enum TernaryType {
    Trit,
    ThreeTrit,
    NineTrit,
}
impl TernaryType {
    pub fn init(&self) -> TernaryValue {
        TernaryValue {
            value: 0,
            ternary_type: *self,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub struct TernaryValue {
    pub value: i16,
    ternary_type: TernaryType,
}
impl Add<i16> for TernaryValue {
    type Output = TernaryValue;
    fn add(self, rhs: i16) -> Self::Output {
        let mut output = self;
        output.value += rhs;
        output.wrap_value();
        output
    }
}
impl Rem<i16> for TernaryValue {
    type Output = TernaryValue;
    fn rem(self, rhs: i16) -> Self::Output {
        let mut output = self;
        output.value %= rhs;
        output
    }
}

impl TernaryValue {
    pub fn new(ternary_type: TernaryType, value: i16) -> Self {
        let output = Self {
            value,
            ternary_type,
        };
        output.verify_type();
        output
    }

    fn verify_type(&self) {
        match self.ternary_type {
            Trit => {
                if self.value > 1 || self.value < -1 {
                    todo!("implement a proper error")
                }
            }
            ThreeTrit => {
                if self.value > 13 || self.value < -13 {
                    todo!("implement a proper error")
                }
            }
            NineTrit => {
                if self.value > 9841 || self.value < -9841 {
                    todo!("implement a proper error")
                }
            }
        }
    }

    fn wrap_value(&mut self) {
        match self.ternary_type {
            Trit => {
                if self.value > 1 || self.value < -1 {
                    self.value %= 2
                }
            }
            ThreeTrit => {
                if self.value > 13 || self.value < -13 {
                    self.value %= 14
                }
            }
            NineTrit => {
                if self.value > 9841 || self.value < -9841 {
                    self.value %= 9842
                }
            }
        }
    }
}

impl TernaryValue {
    pub fn ternary_type(&self) -> TernaryType {
        return self.ternary_type;
    }
}
