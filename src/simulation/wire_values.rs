use std::{
    fmt::{Debug, Display},
    ops::Add,
};

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
impl Display for TernaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Trit => "Trit",
                ThreeTrit => "ThreeTrits",
                NineTrit => "NineTrits",
            }
        )
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
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
impl Debug for TernaryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.ternary_type, self.value)
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
                    self.value += 1;
                    self.value %= 3;
                    self.value -= 1;
                }
            }
            ThreeTrit => {
                if self.value > 13 || self.value < -13 {
                    self.value += 13;
                    self.value %= 27;
                    self.value -= 13;
                }
            }
            NineTrit => {
                if self.value > 9841 || self.value < -9841 {
                    self.value += 9841;
                    self.value %= 19683;
                    self.value -= 9841;
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
