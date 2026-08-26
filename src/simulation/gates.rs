#![allow(unused)]
use std::fmt;

use crate::simulation::gate_definitions::*;
use crate::simulation::wire_values::*;

pub type Ports = Vec<TernaryValue>;

#[derive(Debug)]
pub struct Gate {
    pub id: GateID,
    pub inputs: Ports,
    pub outputs: Ports,
    pub gate_type_id: GateTypeID,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct GateID(pub u64);

impl fmt::Display for GateID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Gate {
    pub fn new(id: GateID, gate_type_id: String, inputs:Ports, outputs:Ports) -> Self {
        Self {
            id,
            inputs ,
            outputs,
            gate_type_id,
        }
    }
}
