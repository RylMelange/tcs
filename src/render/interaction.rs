use std::collections::HashMap;

use raylib::{
    RaylibHandle,
    ffi::{MouseButton, Rectangle, Vector2},
};

use crate::{
    common::{
        app_data::AppData,
        gate_definitions::GateDefinitions,
        helpers::{GatePort, GatePortType, Port::GATEPORT},
    },
    render::renderer::Draggable,
    simulation::gates::{Gate, GateID},
};
pub fn handle_inputs(rl: &mut RaylibHandle, app_data: &mut AppData) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let gates = &mut app_data.gates;
        if let Some(hovered_gate_id) = find_hovered_gate(rl, gates) {
            let hovered_gate = gates
                .get_mut(&hovered_gate_id)
                .expect("couldn't find hovered gate {hovered_gate_id}");
            if let Some(hovered_port) =
                find_hovered_gate_port(rl, hovered_gate, &app_data.gate_definitions)
            {
                match hovered_port.port_type {
                    GatePortType::INPORT => {
                        // TODO: *port == &port doesn't seem right
                        let wire = app_data
                            .renderer
                            .wires
                            .iter()
                            .find(|(_, port)| *port == &GATEPORT(hovered_port.clone()))
                            .expect("couldn't find wire connected to hovered port");
                        let (inport, outport) = (wire.0.clone(), wire.1.clone());
                        app_data.disconnect_gates(&outport, &inport);
                        app_data.connect_gates(&outport, &inport);
                    }
                    GatePortType::OUTPORT => {}
                    GatePortType::INTERNAL => {
                        hovered_gate.inputs[hovered_port.port_index].increment();
                    }
                }
            } else {
                // dragging gate
                app_data.renderer.dragged_component = Draggable::GATE(hovered_gate_id);
                app_data.renderer.mouse_start = hovered_gate.position;
            }
        }
    } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        app_data.renderer.dragged_component = Draggable::NONE;
        app_data.renderer.mouse_start = Vector2::zero();
    }

    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let gates = &mut app_data.gates;
        match app_data.renderer.dragged_component {
            Draggable::GATE(gate_id) => {
                let gate = gates.get_mut(&gate_id).expect("couldn't find gate to drag");
                gate.position += rl.get_mouse_delta();
            }
            Draggable::PORT => {
                todo!("draggable ports")
            }
            Draggable::NONE => {}
        }
    }
}

fn find_hovered_gate(rl: &RaylibHandle, gates: &HashMap<GateID, Gate>) -> Option<GateID> {
    for (gate_id, gate) in gates {
        if Rectangle::new(gate.position.x, gate.position.y, gate.size.x, gate.size.y)
            .check_collision_point_rec(rl.get_mouse_position())
        {
            return Some(*gate_id);
        }
    }
    None
}
// TODO: to remove gate_definitions?
fn find_hovered_gate_port(
    rl: &RaylibHandle,
    gate: &Gate,
    gate_definitions: &GateDefinitions,
) -> Option<GatePort> {
    None
}
