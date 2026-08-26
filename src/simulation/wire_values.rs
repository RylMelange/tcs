#![allow(unused)]
#[derive(Clone, Eq, PartialEq)]
pub enum TernaryType {
    Trit,
    ThreeTrit,
    NineTrit,
}
impl TernaryType {
    pub fn init(&self) -> TernaryValue {
        match self {
            TernaryType::Trit => TernaryValue::Trit(TritValue::Zero),
            TernaryType::ThreeTrit => TernaryValue::ThreeTrit(ThreeTritValue(0)),
            TernaryType::NineTrit => TernaryValue::NineTrit(NineTritValue(0))
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TernaryValue {
    Trit(TritValue),
    ThreeTrit(ThreeTritValue),
    NineTrit(NineTritValue),
}

impl TernaryValue {
    pub fn ternary_type(&self) -> TernaryType {
        match self {
            TernaryValue::Trit(_) => TernaryType::Trit,
            TernaryValue::ThreeTrit(_) => TernaryType::ThreeTrit,
            TernaryValue::NineTrit(_) => TernaryType::NineTrit,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ThreeTritValue(i8);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct NineTritValue(i16);
