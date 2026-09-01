use std::collections::HashMap;

use raylib::prelude::*;

use crate::simulation::{
    gates::{Gate, GateID},
};

pub struct GateRenderData {
    pub pos: Vector2,
    pub size: Vector2,
    pub color: Color,
    pub visible_inports: Option<i16>,
}

impl GateRenderData {
    pub fn new(position: Option<Vector2>) -> Self {
        let pos;
        if position.is_some() {
            pos = position.unwrap()
        } else {
            pos = Vector2 { x: 0.0, y: 0.0 }
        }
        Self {
            pos,
            size: Vector2::new(100.0, 100.0),
            color: Color::new(100, 100, 200, 255),
            visible_inports: None,
        }
    }
}

pub struct Renderer {
    gate_render_data: HashMap<GateID, GateRenderData>,
}
impl Renderer {
    pub fn new() -> Self {
        Self {
            gate_render_data: HashMap::new(),
        }
    }

    pub fn render_all(&mut self, mut d: RaylibDrawHandle, gates: &mut HashMap<GateID, Gate>) {
        d.clear_background(Color::new(30, 30, 50, 255));
        d.draw_text(
            "this is so cool!!! raylib is easy",
            12,
            12,
            35,
            Color::RAYWHITE,
        );

        // TODO: remove temppos
        let mut temppos = Vector2::new(100.0, 300.0);

        for (gate_id, gate) in gates {
            if self.gate_render_data.get(gate_id).is_none() {
                self.gate_render_data
                    .insert(*gate_id, GateRenderData::new(Some(temppos)));
            }

            temppos += Vector2::new(200.0, 0.0);

            let render_data = self.gate_render_data.get(gate_id).unwrap();

            let targets = &gate.targets;
            let inputs = &gate.inputs;
            let outputs = &gate.outputs;

            d.draw_rectangle_v(render_data.pos, render_data.size, Color::BLUEVIOLET);
        }
    }
}
