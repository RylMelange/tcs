use crate::simulation::{gate_definitions::{GateDefinitions, default_definitions}, simulator::Simulator};

pub struct AppData {
    pub simulator: Simulator,
    pub implementations: GateDefinitions,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            simulator: Simulator::new(),
            implementations: default_definitions()
        }
    }
}
