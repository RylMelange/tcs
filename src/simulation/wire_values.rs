#![allow(unused)]
pub enum TernaryType {
    Trit,
    ThreeTrit,
    NineTrit,
}

#[derive(Clone)]
pub enum TernaryValue {
    Trit(Trit),
    ThreeTrit(ThreeTrit),
    NineTrit(NineTrit),
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ThreeTrit(i8);

#[derive(Clone)]
pub struct NineTrit(i16);
