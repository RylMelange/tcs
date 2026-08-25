#![allow(unused)]
use crate::simulation::{gate_definitions::GateImplementation::Code, wire_values::*};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub struct GateTypeID(pub u8);

pub type GateDefinitions = HashMap<GateTypeID, GateDefinition>;

pub enum GateImplementation {
    Code(CodeGate),
    Builtin(BuiltinGate),
    TruthTable(TruthTableGate),
    Recursive(RecursiveGate),
}

pub struct GateDefinition {
    pub implementation: GateImplementation,
    gate_type_id: GateTypeID,
}

impl GateDefinition {
    pub fn compute(&self, inputs: &Vec<TernaryValue>) -> Vec<TernaryValue> {
        match &self.implementation {
            Code(gate) => gate.compute(&self.gate_type_id,inputs),
            _ => {}
        }
        vec![]
    }
}

pub struct CodeGate {}
impl CodeGate {
    pub fn compute(&self, gate_type_id: &GateTypeID, inputs: &Vec<TernaryValue>) {}
}

pub struct BuiltinGate {}
impl BuiltinGate {
    pub fn compute(&self, gate_type_id: &GateTypeID, inputs: &Vec<TernaryValue>) {}
}

pub struct TruthTableGate {}
impl TruthTableGate {
    pub fn compute(&self, gate_type_id: &GateTypeID, inputs: &Vec<TernaryValue>) {}
}

pub struct RecursiveGate {}
impl RecursiveGate {
    pub fn compute(&self, gate_type_id: &GateTypeID, inputs: &Vec<TernaryValue>) {}
}
