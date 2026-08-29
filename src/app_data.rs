use crate::{render::renderer::Renderer, simulation::{gate_definitions::{GateDefinitions, default_definitions}, simulator::Simulator}};

pub struct AppData {
    pub renderer: Renderer,
    pub simulator: Simulator,
    pub implementations: GateDefinitions,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            simulator: Simulator::new(),
            implementations: default_definitions()
        }
    }
}
