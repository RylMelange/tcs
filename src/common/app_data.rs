use crate::{
    common::{
        gate_definitions::{GateDefinitions, default_definitions},
        helpers::Port,
    },
    render::renderer::Renderer,
    simulation::{
        gates::{Gate, GateID, Ports},
        simulator::Simulator,
    },
};
use raylib::ffi::Vector2;
use std::collections::HashMap;

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

    pub fn connect_gates(&mut self, outport: Port, inport: Port) {
        if let Some(outgate) = self.gates.get_mut(&outport.gate_id) {
            outgate.targets[outport.port_index].push(inport.clone());
        } else {
            // TODO: return real error
            eprintln!("couldn't find in-gate to connect")
        }
        // if let Some(outgate) = self.gates.get_mut(&outport.gate_id) {
        //     outgate.sources[inport.port_index].push(inport.clone());
        // } else {
        //     // TODO: return real error
        //     eprintln!("couldn't find out-gate to connect")
        // }
        self.renderer.wires.insert(inport, outport);
    }

    pub fn insert_gate(
        &mut self,
        gate_id: GateID,
        definition_name: String,
        initial_inputs_option: Option<Ports>,
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
            Gate::new(gate_id, gate_type_id, inputs, outputs, position),
        );
    }
}
