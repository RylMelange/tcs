use crate::{
    common::{
        gate_definitions::{GateDefinitions, default_definitions},
        helpers::{GatePort, GatePortType, Port},
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

    pub fn connect_gates(&mut self, outport: &Port, inport: &Port) {
        match outport {
            Port::GATEPORT(gate_port) => {
                if let Some(outgate) = self.gates.get_mut(&gate_port.gate_id) {
                    outgate.targets[gate_port.port_index].push(inport.clone());
                } else {
                    // TODO: return real error
                    eprintln!("couldn't find in-gate to connect")
                }
            }
            Port::MOUSEPORT => {}
        };
        self.renderer.wires.insert(inport.clone(), outport.clone());
        if let Port::GATEPORT(ingateport) = inport {
            self.simulator.insert_pending(ingateport.gate_id);
            self.simulator.request_resort();
        }
    }

    pub fn disconnect_gates(&mut self, outport: &Port, inport: &Port) {
        match outport {
            Port::GATEPORT(gate_port) => {
                if let Some(outgate) = self.gates.get_mut(&gate_port.gate_id) {
                    outgate.targets[gate_port.port_index].retain(|port| port != inport);
                } else {
                    // TODO: return real error
                    eprintln!("couldn't find in-gate to disconnect")
                }
            }
            Port::MOUSEPORT => {}
        }
        self.renderer.wires.remove(inport);
        self.simulator.request_resort();
    }

    pub fn insert_gate(
        &mut self,
        optional_gate_id: Option<GateID>,
        definition_name: String,
        initial_inputs_option: Option<Ports>,
        position: Vector2,
    ) {
        // TODO: surely there's a better way
        let mut gate_id = optional_gate_id.unwrap_or(GateID(rand::random()));
        while self.gates.contains_key(&gate_id) {
            gate_id = GateID(rand::random());
        }

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

        let size = definition.render_data.size;

        self.gates.insert(
            gate_id,
            Gate::new(gate_id, gate_type_id, inputs, outputs, position, size),
        );
    }

    pub fn remove_gate(&mut self, gate_id: GateID) {
        if let Some(gate) = self.gates.get(&gate_id) {

            let mut ports_to_remove = vec![];
            for (port_index, _) in gate.inputs.iter().enumerate() {
                let inport = Port::GATEPORT(GatePort {
                    gate_id,
                    port_index,
                    port_type: GatePortType::INPORT,
                });
                if let Some(outport) = self.renderer.wires.get(&inport) {
                    ports_to_remove.push((outport.clone(), inport));
                }
            }
            // TODO: this doesn't seem like the right way to do things
            for (outport, inport) in ports_to_remove {
                self.disconnect_gates(&outport, &inport);
            }

            self.renderer.wires.retain(|inport, outport| {
                if let Port::GATEPORT(port) = inport {
                    if port.gate_id == gate_id {
                        return false;
                    }
                };
                if let Port::GATEPORT(port) = outport {
                    if port.gate_id == gate_id {
                        return false;
                    }
                };
                true
            });

            self.gates.remove(&gate_id);
            self.simulator.remove_pending(&gate_id);
            self.simulator.request_resort();
        }
    }
}
