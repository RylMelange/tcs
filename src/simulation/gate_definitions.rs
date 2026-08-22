use std::collections::HashMap;

// TODO: maybe replace Vec<i32> with some custom type like Trit(i32) or smth
pub trait GateDefinition {
    fn compute(&mut self, _inputs: Vec<i32>) -> Vec<i32> {
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
    fn compute(&mut self, inputs: Vec<i32>) -> Vec<i32> {
        match self.name {
            "inc" => inputs.iter().map(|i| (i + 1) % 2).collect(),
            _ => {
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
