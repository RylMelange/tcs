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
    // TODO:
    // INTERNAL,
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct GatePort {
    pub gate_id: GateID,
    pub port_index: usize,
    pub port_type: GatePortType,
}
