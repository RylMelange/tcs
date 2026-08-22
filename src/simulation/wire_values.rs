trait TernaryValue {
}

pub enum Trit {
    Negative,
    Zero,
    Positive,
}

impl Trit {
    pub fn increment(&self) -> Trit {
        match self {
            Self::Negative => {Self::Zero},
            Self::Zero => {Self::Positive},
            Self::Positive => {Self::Negative},
        }
    }
}
impl TernaryValue for Trit {}

pub struct ThreeTrit(i8);
impl ThreeTrit { }
impl TernaryValue for ThreeTrit {}

pub struct NineTrit(i16);
impl NineTrit { }
impl TernaryValue for NineTrit {}
