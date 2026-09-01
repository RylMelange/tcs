use std::collections::HashMap;

use crate::{
    render::renderer::Renderer, simulation::{
        gate_definitions::{GateDefinition, GateDefinitions, default_definitions}, gates::{Gate, GateID, Ports}, simulator::{Simulator, Target},
    },
};

pub struct AppData {
    pub renderer: Renderer,
    pub simulator: Simulator,
    pub implementations: GateDefinitions,

    pub gates: HashMap<GateID, Gate>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            simulator: Simulator::new(),
            implementations: default_definitions(),

            gates: HashMap::new(),
        }
    }

    pub fn insert_gate(
        &mut self,
        gate_id: GateID,
        definition_name: String,
        initial_inputs_option: Option<Ports>,
        targets: Vec<Target>,
    ) {
        // TODO: don't unwrap
        let definition = self.implementations.get(&definition_name).unwrap();
        let gate_type_id = definition.gate_type_id.clone();

        let inputs;
        if let Some(initial_inputs) = initial_inputs_option {
            inputs = initial_inputs;
        } else {
            inputs = definition
                .signature
                .inputs
                .iter()
                .map(|t| t.init())
                .collect();
        }

        let outputs = definition
            .signature
            .outputs
            .iter()
            .map(|t| t.init())
            .collect();

        self.gates
            .insert(gate_id, Gate::new(gate_id, gate_type_id, inputs, outputs,targets));
    }
}
