use crate::simulation::{
    gate_definitions::{GateDefinition, GateTypeID},
    gates::{Gate, GateID},
};
use std::collections::{HashMap, HashSet};

// TODO: move to a /lib or smth?
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
    pending_gates: Vec<GateID>,
    sorted_gates: Option<Vec<GateID>>,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            pending_gates: vec![],
            sorted_gates: None,
        }
    }

    pub fn insert_pending(&mut self, gate_id: GateID) {
        self.pending_gates.push(gate_id)
    }

    pub fn step(
        &mut self,
        gates: &mut HashMap<GateID, Gate>,
        implementations: &HashMap<GateTypeID, GateDefinition>,
    ) {
        if self.sorted_gates.is_none() {
            self.generate_sorted_gates(gates);
        }

        for gate_id in self.sorted_gates.as_ref().unwrap() {
            let outputs;

            if let Some(gate) = gates.get_mut(&gate_id) {
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
                        gates
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

    fn generate_sorted_gates(&mut self, gates: &HashMap<GateID, Gate>) {
        let mut sorted_gates = vec![];
        let mut temporary_gates = HashSet::new();
        for gate in &self.pending_gates {
            dfs_visit(*gate, &mut sorted_gates, &mut temporary_gates, gates);
        }
        sorted_gates.reverse();
        self.sorted_gates = Some(sorted_gates);
    }
}

fn dfs_visit(
    node: GateID,
    sorted_gates: &mut Vec<GateID>,
    temporary_gates: &mut HashSet<GateID>,
    gates: &HashMap<GateID, Gate>,
) {
    if sorted_gates.contains(&node) {
        return;
    };
    if temporary_gates.contains(&node) {
        eprintln!("TODO: account for cyclic graphs");
        return;
    };

    temporary_gates.insert(node);

    for target in &gates.get(&node).unwrap().targets {
        for port in target {
            dfs_visit(port.gate_id, sorted_gates, temporary_gates, &gates);
        }
    }

    sorted_gates.push(node);
}
