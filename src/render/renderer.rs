use crate::{
    common::gate_definitions::{GateDefinitions, Rect},
    simulation::{
        gates::{Gate, GateID},
        wire_values::TernaryValue,
    },
};
use raylib::prelude::*;
use std::collections::HashMap;
const PORT_SIZE: Vector2 = Vector2 { x: 10.0, y: 10.0 };
const PADDING: f32 = 10.0;

#[derive(Clone)]
pub struct GateRenderData {
    pub size: Vector2,
    pub color: Color,
    pub visible_inports: Option<i16>,
    pub inport_geometries: Vec<Rect>,
    pub outport_geometries: Vec<Rect>,
}
impl Default for GateRenderData {
    fn default() -> Self {
        Self {
            size: Vector2 { x: 100.0, y: 100.0 },
            color: Color::MEDIUMTURQUOISE,
            visible_inports: None,
            inport_geometries: vec![],
            outport_geometries: vec![],
        }
    }
}

pub struct Renderer {}
impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_all(
        &mut self,
        mut d: RaylibDrawHandle,
        gate_definitions: &GateDefinitions,
        gates: &mut HashMap<GateID, Gate>,
    ) {
        d.clear_background(Color::new(30, 30, 50, 255));

        for (gate_id, gate) in gates {
            if let Some(gate_definition) = gate_definitions.get(&gate.gate_type_id).as_ref() {
                let render_data = &gate_definition.render_data;
                let position = &gate.position;
                let targets = &gate.targets;
                let inputs = &gate.inputs;
                let outputs = &gate.outputs;

                self.draw_gate_body(&mut d, position, render_data);

                self.draw_ports(&mut d, position, inputs, &render_data.inport_geometries);
                self.draw_ports(&mut d, position, outputs, &render_data.outport_geometries);
            } else {
                eprintln!("could not find render_data in gate_definitions for {gate_id}")
            }
        }

        d.draw_text("GUI goes here", 12, 12, 35, Color::RAYWHITE);
    }

    fn draw_gate_body(
        &self,
        d: &mut RaylibDrawHandle,
        position: &Vector2,
        render_data: &GateRenderData,
    ) {
        // TODO: draw such that "position" of render_data used is relative to camera
        d.draw_rectangle_v(*position, render_data.size, Color::BLUEVIOLET);
    }
    fn draw_ports(
        &self,
        d: &mut RaylibDrawHandle,
        origin: &Vector2,
        values: &Vec<TernaryValue>,
        geometries: &Vec<Rect>,
    ) {
        for index in 0..geometries.len() {
            let color = value_to_color(values[index].value);
            let geometry = geometries[index];
            d.draw_rectangle_v(geometry.pos + *origin, geometry.size, color);
        }
    }
}

fn value_to_color(value: i16) -> Color {
    match value {
        -1 => Color::RED,
        0 => Color::DARKSLATEGRAY,
        1 => Color::BLUE,
        _ => Color::GREENYELLOW,
    }
}
