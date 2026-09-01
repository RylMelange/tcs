use crate::simulation::gate_definitions::*;
use crate::simulation::simulator::Target;
use crate::simulation::wire_values::*;
use raylib::ffi::Vector2;
use std::fmt;

pub type Ports = Vec<TernaryValue>;

pub struct Gate {
    pub id: GateID,
    pub inputs: Ports,
    pub outputs: Ports,
    pub gate_type_id: GateTypeID,
    // TODO: does this need pub?
    pub targets: Vec<Target>,
    pub position: Vector2,
}

impl fmt::Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gate {} ({}): {{Inputs: {:?}, Outputs: {:?}}}",
            self.id, self.gate_type_id, self.inputs, self.outputs
        )
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gate {} ({}): {{Inputs: {:?}, Outputs: {:?}}}",
            self.id, self.gate_type_id, self.inputs, self.outputs
        )
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct GateID(pub u64);

impl fmt::Display for GateID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Gate {
    pub fn new(
        id: GateID,
        gate_type_id: String,
        inputs: Ports,
        outputs: Ports,
        targets: Vec<Target>,
        position: Vector2,
    ) -> Self {
        Self {
            id,
            inputs,
            outputs,
            gate_type_id,
            targets,
            position,
        }
    }
}
