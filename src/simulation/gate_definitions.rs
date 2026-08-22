use crate::simulation::wire_values::*;
use std::collections::HashMap;

pub trait GateDefinition {
    fn compute(&self, _inputs: &[Trit]) -> Vec<Trit> {
        vec![]
    }
}

pub struct InbuiltGate {
    name: &'static str,
}
impl InbuiltGate {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}
impl GateDefinition for InbuiltGate {
    fn compute(&self, inputs: &[Trit]) -> Vec<Trit> {
        match self.name {
            "inc" => inputs.iter().map(|i| i.increment()).collect(),
            _ => {
                eprintln!("TODO: add definition for {} gate", self.name);
                vec![]
            }
        }
    }
}

pub struct TruthTableGate {
    truth_table: HashMap<String, i32>,
}

impl GateDefinition for TruthTableGate {}

pub struct CodeGate {}
impl GateDefinition for CodeGate {}

pub struct RecursiveGate {}
impl GateDefinition for RecursiveGate {}
