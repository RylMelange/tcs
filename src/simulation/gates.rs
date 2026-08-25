#![allow(unused)]
use std::fmt;

use crate::simulation::gate_definitions::*;
use crate::simulation::wire_values::*;

pub type Ports = Vec<TernaryValue>;

pub struct Gate {
    id: GateID,
    pub inputs: Ports,
    pub outputs: Ports,
    pub gate_type_id: GateTypeID,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct GateID(u64);
impl fmt::Display for GateID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Gate {
    pub fn new(id: u64) -> Self {
        Self {
            id: GateID(id),
            inputs: vec![],
            outputs: vec![],
            gate_type_id: GateTypeID(0),
        }
    }
}
