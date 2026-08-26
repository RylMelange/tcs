#![allow(unused)]
use crate::simulation::{
    gate_definitions::GateImplementation::*,
    wire_values::{
        TernaryType,
        TernaryValue::{self, *},
    },
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

pub struct GateSignature {
    pub inputs: Vec<TernaryType>,
    pub outputs: Vec<TernaryType>,
}

pub struct GateDefinition {
    pub gate_type_id: GateTypeID,
    pub signature: GateSignature,
    pub implementation: GateImplementation,
}

impl GateDefinition {
    pub fn new(
        gate_type_id: &str,
        input_signature: Vec<TernaryType>,
        output_signature: Vec<TernaryType>,
        implementation: GateImplementation,
    ) -> Self {
        Self {
            gate_type_id: gate_type_id.to_string(),
            signature: GateSignature {
                inputs: input_signature,
                outputs: output_signature,
            },
            implementation,
        }
    }

    pub fn compute(&self, inputs: &Vec<TernaryValue>) -> Vec<TernaryValue> {
        self.verify_signature(inputs);

        match &self.implementation {
            Builtin(gate) => gate.compute(&self.gate_type_id, inputs),
            TruthTable(gate) => gate.compute(&self.gate_type_id, inputs),
            Recursive(gate) => gate.compute(&self.gate_type_id, inputs),
            Code(gate) => gate.compute(&self.gate_type_id, inputs),
        }
    }

    fn verify_signature(&self, inputs: &Vec<TernaryValue>) {
        if inputs.len() != self.signature.inputs.len()
            || inputs
                .iter()
                .map(|i| i.ternary_type())
                .collect::<Vec<TernaryType>>()
                != self.signature.inputs
        {
            // TODO:make an actual error here
            panic!()
        }
    }
}

pub struct BuiltinGate {
    pub compute_function: fn(inputs: &Vec<TernaryValue>) -> Vec<TernaryValue>,
}

impl BuiltinGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        (self.compute_function)(inputs)
    }
}

pub struct TruthTableGate {
    truth_table: HashMap<Vec<TernaryValue>, Vec<TernaryValue>>,
}
impl TruthTableGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        if let Some(outputs) = self.truth_table.get(inputs) {
            outputs.clone()
        } else {
            panic!("Unknown truthtable gate {gate_type_id}!")
        }
    }
}

pub struct RecursiveGate {}
impl RecursiveGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        panic!("recursive gates are not implemented yet");
    }
}

pub struct CodeGate {}
impl CodeGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        panic!("code gates are not implemented yet");
    }
}
