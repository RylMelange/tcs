#![allow(unused)]
pub enum TernaryType {
    Trit,
    ThreeTrit,
    NineTrit,
}

pub enum TernaryValue {
    Trit(Trit),
    ThreeTrit(ThreeTrit),
    NineTrit(NineTrit),
}

pub enum Trit {
    Negative,
    Zero,
    Positive,
}

impl Trit {
    pub fn increment(&self) -> Trit {
        match self {
            Self::Negative => Self::Zero,
            Self::Zero => Self::Positive,
            Self::Positive => Self::Negative,
        }
    }
}

pub struct ThreeTrit(i8);

pub struct NineTrit(i16);
