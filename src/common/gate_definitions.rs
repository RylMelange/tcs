use crate::{
    common::gate_definitions::GateImplementation::*,
    render::renderer::GateRenderData,
    simulation::wire_values::{
        TernaryType::{self, Trit},
        TernaryValue,
    },
};
use raylib::ffi::{Rectangle, Vector2};
use std::collections::HashMap;

// TODO: move to objects or smth
#[derive(Clone, Copy)]
pub struct Rect {
    pub pos: Vector2,
    pub size: Vector2,
}
impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            pos: Vector2 { x, y },
            size: Vector2 {
                x: width,
                y: height,
            },
        }
    }
}
impl Into<Rectangle> for Rect {
    fn into(self) -> Rectangle {
        Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.size.y,
            height: self.size.y,
        }
    }
}

pub type GateTypeID = String;

pub type GateDefinitions = HashMap<GateTypeID, GateDefinition>;

fn insert_implementation(implementations: &mut GateDefinitions, definition: GateDefinition) {
    implementations.insert(definition.gate_type_id.clone(), definition);
}

pub fn default_definitions() -> GateDefinitions {
    let mut implementations = HashMap::new();
    insert_implementation(
        &mut implementations,
        GateDefinition::new(
            "source",
            vec![Trit],
            vec![Trit],
            Builtin(BuiltinGate {
                compute_function: |a| vec![a[0]],
            }),
            GateRenderData {
                visible_inports: Some(0),
                outport_geometries: vec![Rect::new(70.0, 40.0, 20.0, 20.0)],
                ..Default::default()
            },
        ),
    );
    insert_implementation(
        &mut implementations,
        GateDefinition::new(
            "increment",
            vec![Trit],
            vec![Trit],
            Builtin(BuiltinGate {
                compute_function: |a| vec![a[0] + 1],
            }),
            GateRenderData {
                ..Default::default()
            },
        ),
    );
    insert_implementation(
        &mut implementations,
        GateDefinition::new(
            "and",
            vec![Trit; 2],
            vec![Trit],
            Builtin(BuiltinGate {
                compute_function: |a| {
                    if a[0] == a[1] {
                        vec![a[0] + 1]
                    } else {
                        vec![TernaryValue::new(Trit, 0)]
                    }
                },
            }),
            GateRenderData {
                inport_geometries: vec![
                    Rect::new(10.0, 25.0, 10.0, 10.0),
                    Rect::new(10.0, 65.0, 10.0, 10.0),
                ],
                outport_geometries: vec![Rect::new(80.0, 45.0, 10.0, 10.0)],
                ..Default::default()
            },
        ),
    );
    implementations
}

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
    pub render_data: GateRenderData,
}

impl GateDefinition {
    pub fn new(
        gate_type_id: &str,
        input_signature: Vec<TernaryType>,
        output_signature: Vec<TernaryType>,
        implementation: GateImplementation,
        render_data: GateRenderData,
    ) -> Self {
        Self {
            gate_type_id: gate_type_id.to_string(),
            signature: GateSignature {
                inputs: input_signature,
                outputs: output_signature,
            },
            implementation,
            render_data,
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
            todo!("make an actual error here: wrong input types, expected smth got smth")
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
            todo!("Unknown truthtable gate {gate_type_id}! (create an error someday)")
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
        todo!("recursive gates are not implemented yet");
    }
}

pub struct CodeGate {}
impl CodeGate {
    pub fn compute(
        &self,
        gate_type_id: &GateTypeID,
        inputs: &Vec<TernaryValue>,
    ) -> Vec<TernaryValue> {
        todo!("code gates are not implemented yet");
    }
}
