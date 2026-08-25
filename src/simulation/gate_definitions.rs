#![allow(unused)]
use crate::simulation::{
    gate_definitions::GateImplementation::*,
    wire_values::TernaryValue::{self, *},
};
use std::collections::HashMap;

pub type GateTypeID = String;

pub type GateDefinitions = HashMap<GateTypeID, GateDefinition>;

pub enum GateImplementation {
    Builtin(BuiltinGate),
    TruthTable(TruthTableGate),
    Recursive(RecursiveGate),
    Code(CodeGate),
}

pub struct GateDefinition {
    pub implementation: GateImplementation,
    gate_type_id: GateTypeID,
}

impl GateDefinition {
    pub fn compute(&self, inputs: &Vec<TernaryValue>) -> Vec<TernaryValue> {
        match &self.implementation {
            Builtin(gate) => gate.compute(&self.gate_type_id, inputs),
            TruthTable(gate) => gate.compute(&self.gate_type_id, inputs),
            Recursive(gate) => gate.compute(&self.gate_type_id, inputs),
            Code(gate) => gate.compute(&self.gate_type_id, inputs),
        }
    }
}

pub struct BuiltinGate {}
impl BuiltinGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        match gate_type_id.as_str() {
            "inc" => inputs
                .iter()
                .map(|v| {
                    match v {
                        Trit(val) => Trit(val.increment()),
                        ThreeTrit(val) => ThreeTrit(val),
                        NineTrit(val) => NineTrit(val),
                    };
                })
                .collect(),
            _ => {
                println!("Unknown gate {gate_type_id}!")
            }
        }
        vec![]
    }
}

pub struct TruthTableGate {}
impl TruthTableGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        vec![]
    }
}

pub struct RecursiveGate {}
impl RecursiveGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        vec![]
    }
}

pub struct CodeGate {}
impl CodeGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        vec![]
    }
}
