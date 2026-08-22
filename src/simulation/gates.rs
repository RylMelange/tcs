#![allow(unused)]
use crate::simulation::gate_definitions::*;
use crate::simulation::gates::GateType::*;
use crate::simulation::wire_values::Trit;

pub struct Gate {
    id: GateID,
    inputs: Vec<i32>,
    pub outputs: Vec<Trit>,
    gate_type: GateType,
}

struct GateID(u64);

impl Gate {
    pub fn new(id: u64) -> Self {
        Self {
            id: GateID(id),
            inputs: vec![],
            outputs: vec![],
            gate_type: Inbuilt(InbuiltGate::new("inc")),
        }
    }

    pub fn compute(&mut self) {
        self.outputs = self.gate_type.compute(&[]);
    }
}

enum GateType {
    Inbuilt(InbuiltGate),
    TruthTable(TruthTableGate),
    Code(CodeGate),
    Recursive(RecursiveGate),
}

impl GateType {
    pub fn compute(&mut self, inputs: &[Trit]) -> Vec<Trit> {
        match self {
            GateType::Inbuilt(gate) => gate.compute(inputs),
            GateType::TruthTable(gate) => gate.compute(inputs),
            GateType::Code(gate) => gate.compute(inputs),
            GateType::Recursive(gate) => gate.compute(inputs),
        }
    }
}
