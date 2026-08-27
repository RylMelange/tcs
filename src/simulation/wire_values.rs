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
            TernaryType::Trit => TernaryValue::Trit(TritValue(0)),
            TernaryType::ThreeTrit => TernaryValue::ThreeTrit(ThreeTritValue(0)),
            TernaryType::NineTrit => TernaryValue::NineTrit(NineTritValue(0)),
        }
    }
}

// TODO: remove this enum?
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

    pub fn assert_trit(value: TernaryValue) -> TritValue {
        if let TernaryValue::Trit(val) = value {
            val
        } else {
            unreachable!("This value is guaranteed to be a trit")
        }
    }
    pub fn assert_three_trit(value: TernaryValue) -> ThreeTritValue {
        if let TernaryValue::ThreeTrit(val) = value {
            val
        } else {
            unreachable!("This value is guaranteed to be three trits")
        }
    }
    pub fn assert_nine_trit(value: TernaryValue) -> NineTritValue {
        if let TernaryValue::NineTrit(val) = value {
            val
        } else {
            unreachable!("This value is guaranteed to be nine trits")
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TritValue(i8);

impl TritValue {
    pub fn new(value: i8) -> Self {
        // TODO: return Result<> someday
        if value < -1 || value > 1 {
            todo!("non-Trit value attempted to pretend to be a Trit")
        } else {
            Self(value)
        }
    }

    pub fn increment(&self) -> Self {
        match self {
            Self(1) => Self(-1),
            Self(0) => Self(1),
            Self(-1) => Self(0),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ThreeTritValue(i8);

impl ThreeTritValue {
    pub fn new(value: i8) -> Self {
        // TODO: return Result<> someday
        if value < -13 || value > 13 {
            todo!("non-ThreeTrit value attempted to pretend to be a ThreeTrit")
        } else {
            Self(value)
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct NineTritValue(i16);

impl NineTritValue {
    pub fn new(value: i16) -> Self {
        // TODO: return Result<> someday
        if value < -9841 || value > 9841 {
            todo!("non-NineTrit value attempted to pretend to be a NineTrit")
        } else {
            Self(value)
        }
    }
}
