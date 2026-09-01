use crate::simulation::gates::GateID;

pub type Target = Vec<InPort>;

#[derive(Clone)]
pub struct InPort {
    pub gate_id: GateID,
    pub port_index: usize,
}
impl InPort {
    pub fn new(gate_id: GateID, port_index: usize) -> Self {
        Self {
            gate_id: gate_id,
            port_index,
        }
    }
}
