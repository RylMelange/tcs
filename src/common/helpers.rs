use raylib::ffi::Vector2;

use crate::simulation::gates::GateID;

pub type Target = Vec<Port>;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Port {
    GATEPORT(GatePort),
    MOUSEPORT,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum GatePortType {
    INPORT,
    OUTPORT,
    INTERNAL,
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct GatePort {
    pub gate_id: GateID,
    pub port_index: usize,
    pub port_type: GatePortType,
}
impl GatePort {
    pub fn new(gate_id: GateID, port_type: GatePortType, port_index: usize) -> Self {
        Self {
            gate_id,
            port_index,
            port_type,
        }
    }
}
