#![allow(unused)]
pub enum TernaryType {
    Trit,
    ThreeTrit,
    NineTrit,
}

#[derive(Clone, Eq, PartialEq)]
pub enum TernaryValue {
    Trit(TritValue),
    ThreeTrit(ThreeTritValue),
    NineTrit(NineTritValue),
}

#[derive(Clone, Eq, PartialEq)]
pub enum TritValue {
    Negative,
    Zero,
    Positive,
}

impl TritValue {
    pub fn increment(&self) -> Self {
        match self {
            Self::Negative => Self::Zero,
            Self::Zero => Self::Positive,
            Self::Positive => Self::Negative,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ThreeTritValue(i8);

#[derive(Clone, Eq, PartialEq)]
pub struct NineTritValue(i16);
