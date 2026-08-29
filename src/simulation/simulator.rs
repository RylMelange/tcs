use crate::simulation::{
    gate_definitions::{GateDefinition, GateTypeID},
    gates::{Gate, GateID, Ports},
};
use std::collections::{HashMap, HashSet};

pub type Target = Vec<InPort>;
pub struct InPort {
    gate_id: GateID,
    port_index: usize,
}
impl InPort {
    pub fn new(gate_id: GateID, port_index: usize) -> Self {
        Self {
            gate_id: gate_id,
            port_index,
        }
    }
}

pub struct Simulator {
    pub graph: HashMap<GateID, Vec<Target>>,
    pub gates: HashMap<GateID, Gate>,
    pending_gates: Vec<GateID>,
    sorted_gates: Option<Vec<GateID>>,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            gates: HashMap::new(),
            pending_gates: vec![],
            sorted_gates: None,
        }
    }

    pub fn insert_gate(
        &mut self,
        gate_id: GateID,
        definition: &GateDefinition,
        initial_inputs_option: Option<Ports>,
        targets: Vec<Target>,
    ) {
        self.graph.insert(gate_id.clone(), targets);
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
            .insert(gate_id, Gate::new(gate_id, gate_type_id, inputs, outputs));
    }

    pub fn insert_pending(&mut self, gate_id: GateID) {
        self.pending_gates.push(gate_id)
    }

    pub fn step(&mut self, implementations: &HashMap<GateTypeID, GateDefinition>) {
        if self.sorted_gates.is_none() {
            self.generate_sorted_gates();
        }

        for gate_id in self.sorted_gates.as_ref().unwrap() {
            let outputs;

            if let Some(gate) = self.gates.get_mut(&gate_id) {
                let definition = implementations
                    .get(&gate.gate_type_id)
                    .expect("gate definitions missing!");
                outputs = definition.compute(&gate.inputs);
                gate.outputs = outputs.clone();
            } else {
                println!("{gate_id} was listed in sorted_gates, but not gates!");
                continue;
            }

            if let Some(targets) = self.graph.get(&gate_id) {
                for index in 0..outputs.len() {
                    for target in &targets[index] {
                        self.gates
                            .get_mut(&target.gate_id)
                            .expect("couldn't find target gate")
                            .inputs[target.port_index] = outputs[index].clone();
                    }
                }
            } else {
                println!("{gate_id} didn't seem to have target gates!");
                continue;
            }
        }
    }

    fn generate_sorted_gates(&mut self) {
        let mut sorted_gates = vec![];
        let mut temporary_gates = HashSet::new();
        for gate in &self.pending_gates {
            dfs_visit(*gate, &mut sorted_gates, &mut temporary_gates, &self.graph);
        }
        sorted_gates.reverse();
        self.sorted_gates = Some(sorted_gates);
    }
}

fn dfs_visit(
    node: GateID,
    sorted_gates: &mut Vec<GateID>,
    temporary_gates: &mut HashSet<GateID>,
    graph: &HashMap<GateID, Vec<Target>>,
) {
    if sorted_gates.contains(&node) {
        return;
    };
    if temporary_gates.contains(&node) {
        eprintln!("TODO: account for cyclic graphs");
        return;
    };

    temporary_gates.insert(node);

    for output in graph.get(&node).unwrap() {
        for target in output {
            dfs_visit(target.gate_id, sorted_gates, temporary_gates, graph);
        }
    }

    sorted_gates.push(node);
}
