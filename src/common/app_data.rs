use std::collections::HashMap;

use raylib::ffi::Vector2;

use crate::{
    common::{
        gate_definitions::{GateDefinitions, default_definitions},
        helpers::Target,
    },
    render::renderer::Renderer,
    simulation::{
        gates::{Gate, GateID, Ports},
        simulator::Simulator,
    },
};

pub struct AppData {
    pub renderer: Renderer,
    pub simulator: Simulator,
    pub gate_definitions: GateDefinitions,
    pub gates: HashMap<GateID, Gate>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            simulator: Simulator::new(),
            gate_definitions: default_definitions(),
            gates: HashMap::new(),
        }
    }

    pub fn insert_gate(
        &mut self,
        gate_id: GateID,
        definition_name: String,
        initial_inputs_option: Option<Ports>,
        targets: Vec<Target>,
        position: Vector2,
    ) {
        // TODO: don't unwrap
        let definition = self.gate_definitions.get(&definition_name).unwrap();
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

        self.gates.insert(
            gate_id,
            Gate::new(gate_id, gate_type_id, inputs, outputs, targets, position),
        );
    }
}
