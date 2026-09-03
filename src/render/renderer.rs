use crate::{
    common::{
        gate_definitions::{GateDefinitions, Rect},
        helpers::{GatePortType, Port},
    },
    simulation::{
        gates::{Gate, GateID},
        wire_values::TernaryValue,
    },
};
use raylib::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct GateRenderData {
    pub size: Vector2,
    // pub color: Color,
    pub visible_inports: Option<i16>,
    pub inport_geometries: Vec<Rect>,
    pub outport_geometries: Vec<Rect>,
}
impl Default for GateRenderData {
    fn default() -> Self {
        Self {
            size: Vector2 { x: 100.0, y: 100.0 },
            // color: Color::MEDIUMTURQUOISE,
            visible_inports: None,
            inport_geometries: vec![],
            outport_geometries: vec![],
        }
    }
}

pub enum Draggable {
    GATE(GateID),
    PORT(Vector2, Port),
    NONE,
}

pub struct Renderer {
    pub wires: HashMap<Port, Port>,
    pub dragged_component: Draggable,
}
impl Renderer {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
            dragged_component: Draggable::NONE,
        }
    }

    pub fn render_all(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        gate_definitions: &GateDefinitions,
        gates: &HashMap<GateID, Gate>,
    ) {
        // TODO: is there a better way to do this?
        let mouse_position = rl.get_mouse_position();

        let mut d = &mut rl.begin_drawing(thread);
        d.clear_background(Color::new(30, 30, 50, 255));

        for (gate_id, gate) in gates {
            if let Some(gate_definition) = gate_definitions.get(&gate.gate_type_id) {
                let render_data = &gate_definition.render_data;
                let position = &gate.position;
                let inputs = &gate.inputs;
                let outputs = &gate.outputs;

                draw_gate_body(d, position, &gate.size);

                // TODO: change render_data to be within gate?
                draw_ports(
                    d,
                    position,
                    inputs,
                    &render_data.inport_geometries,
                    &render_data.visible_inports,
                );
                draw_ports(
                    d,
                    position,
                    outputs,
                    &render_data.outport_geometries,
                    &None,
                );
            } else {
                eprintln!("could not find render_data in gate_definitions for {gate_id}")
            }
        }

        for (inport, outport) in &self.wires {
            draw_wire(d, mouse_position, outport, inport, gates, gate_definitions);
        }

        d.draw_text("GUI goes here", 12, 12, 35, Color::RAYWHITE);
    }
}

fn draw_gate_body(d: &mut RaylibDrawHandle, position: &Vector2, size: &Vector2) {
    // TODO: draw such that "position" of render_data used is relative to camera
    d.draw_rectangle_v(*position, *size, Color::BLUEVIOLET);
}
fn draw_ports(
    d: &mut RaylibDrawHandle,
    origin: &Vector2,
    values: &[TernaryValue],
    geometries: &[Rect],
    visible_ports: &Option<i16>,
) {
    let ports_number = match visible_ports {
        Some(num) => *num as usize,
        None => geometries.len(),
    };
    for index in 0..ports_number {
        let color = value_to_color(values[index].value);
        let geometry = geometries[index];
        d.draw_rectangle_v(
            geometry.pos + *origin - (geometry.size.scale(0.5)),
            geometry.size,
            color,
        );
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

fn draw_wire(
    d: &mut RaylibDrawHandle,
    mouse_position: Vector2,
    outport: &Port,
    inport: &Port,
    gates: &HashMap<GateID, Gate>,
    gate_definitions: &GateDefinitions,
) {
    let (origin_position, value) =
        get_port_position(mouse_position, gates, gate_definitions, outport);
    let (target_position, _) = get_port_position(mouse_position, gates, gate_definitions, inport);

    // d.draw_line_bezier(origin_position, target_position, 6.0, value_to_color(value));
    d.draw_spline_bezier_cubic(
        &[
            origin_position,
            (origin_position + Vector2::new(100.0, 0.0)),
            (target_position + Vector2::new(-100.0, 0.0)),
            target_position,
        ],
        6.0,
        value_to_color(value),
    );
}

fn get_port_position(
    mouse_position: Vector2,
    gates: &HashMap<GateID, Gate>,
    gate_definitions: &GateDefinitions,
    port: &Port,
) -> (Vector2, i16) {
    match port {
        Port::GATEPORT(gate_port) => {
            let gate = gates
                .get(&gate_port.gate_id)
                .expect("origin gate doesn't exist");
            let render_data = &gate_definitions
                .get(&gate.gate_type_id)
                .expect("render data of origin gate doesn't exist")
                .render_data;
            let (geometries, values) = match gate_port.port_type {
                GatePortType::INPORT => (&render_data.inport_geometries, &gate.inputs),
                GatePortType::OUTPORT => (&render_data.outport_geometries, &gate.outputs),
                // GatePortType::INTERNAL => {
                //     todo!() /*render_data.internal_geometries*/
                // }
            };
            (
                gate.position + geometries[gate_port.port_index].pos,
                values[gate_port.port_index].value,
            )
        }
        Port::MOUSEPORT => (mouse_position, 0),
    }
}
