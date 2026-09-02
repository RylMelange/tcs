use crate::simulation::gates::GateID;

pub type Target = Vec<Port>;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Port {
    pub gate_id: GateID,
    pub port_index: usize,
}
impl Port {
    pub fn new(gate_id: GateID, port_index: usize) -> Self {
        Self {
            gate_id: gate_id,
            port_index,
        }
    }
}
