use std::collections::HashMap;

use raylib::{
    RaylibHandle,
    ffi::{MouseButton, Rectangle},
};

use crate::{
    common::{
        app_data::AppData,
        gate_definitions::GateDefinitions,
        helpers::{
            GatePort, GatePortType,
            Port::{self, GATEPORT},
        },
    },
    render::renderer::Draggable,
    simulation::gates::{Gate, GateID},
};
pub fn handle_inputs(rl: &mut RaylibHandle, app_data: &mut AppData) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let gates = &mut app_data.gates;

        if let Some(hovered_gate_id) = find_hovered_gate(rl, gates)
            && !try_drag_hovered_port(app_data, rl, hovered_gate_id)
        {
            app_data.renderer.dragged_component = Draggable::GATE(hovered_gate_id);
        }
    } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        match app_data.renderer.dragged_component {
            Draggable::GATE(_) => {}
            Draggable::INPORT(_, ref outport) => {
                let outport = outport.clone();
                app_data.disconnect_gates(&outport.clone(), &Port::MOUSEPORT);
                if let Some(hovered_gate_id) = find_hovered_gate(rl, &app_data.gates) {
                    let gate = app_data
                        .gates
                        .get(&hovered_gate_id)
                        .expect("couldn't get the hovered gate");
                    if let Some(inport) =
                        find_hovered_gate_port(rl, gate, &app_data.gate_definitions)
                    {
                        // TODO: check if a wire is connected to the inport we just found, and
                        // disconnect it.
                        if let Some(Port::GATEPORT(previous_outport)) =
                            app_data.renderer.wires.get(&Port::GATEPORT(inport.clone()))
                        {
                            let previous_outgate = app_data.gates.get_mut(&previous_outport.gate_id).expect("couldn't find the original out-gate of the wire we're replacing");
                            previous_outgate.targets[previous_outport.port_index]
                                .retain(|port| port != &Port::GATEPORT(inport.clone()));
                        }
                        app_data.connect_gates(&outport, &Port::GATEPORT(inport));
                    }
                }
            }
            Draggable::NONE => {}
        }
        app_data.renderer.dragged_component = Draggable::NONE;
    }

    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let gates = &mut app_data.gates;
        match app_data.renderer.dragged_component {
            Draggable::GATE(gate_id) => {
                let gate = gates.get_mut(&gate_id).expect("couldn't find gate to drag");
                gate.position += rl.get_mouse_delta();
            }
            Draggable::INPORT(mut position, _) => {
                position += rl.get_mouse_delta();
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
    let mouse_pos = rl.get_mouse_position();
    let definition = gate_definitions
        .get(&gate.gate_type_id)
        .expect("couldn't find definition for hovered gate!");

    for (index, inport) in definition.render_data.inport_geometries.iter().enumerate() {
        if Rectangle::new(
            inport.pos.x - inport.size.x / 2.0 + gate.position.x,
            inport.pos.y - inport.size.y / 2.0 + gate.position.y,
            inport.size.x,
            inport.size.y,
        )
        .check_collision_point_rec(mouse_pos)
        {
            return Some(GatePort {
                gate_id: gate.id,
                port_index: index,
                port_type: GatePortType::INPORT,
            });
        }
    }
    for (index, outport) in definition.render_data.outport_geometries.iter().enumerate() {
        if Rectangle::new(
            outport.pos.x - outport.size.x / 2.0 + gate.position.x,
            outport.pos.y - outport.size.y / 2.0 + gate.position.y,
            outport.size.x,
            outport.size.y,
        )
        .check_collision_point_rec(mouse_pos)
        {
            return Some(GatePort {
                gate_id: gate.id,
                port_index: index,
                port_type: GatePortType::OUTPORT,
            });
        }
    }
    None
}
fn try_drag_hovered_port(
    app_data: &mut AppData,
    rl: &RaylibHandle,
    hovered_gate_id: GateID,
) -> bool {
    let gate = app_data
        .gates
        .get_mut(&hovered_gate_id)
        .expect("couldn't find hovered gate {hovered_gate_id}");
    if let Some(hovered_port) = find_hovered_gate_port(rl, gate, &app_data.gate_definitions) {
        match hovered_port.port_type {
            GatePortType::INPORT => {
                if let Some(wire) = app_data
                    .renderer
                    .wires
                    .iter()
                    .find(|(port, _)| *port == &GATEPORT(hovered_port.clone()))
                {
                    let outport = wire.1.clone();
                    let inport = GATEPORT(hovered_port);
                    app_data.disconnect_gates(&outport, &inport);
                    app_data.connect_gates(&outport, &Port::MOUSEPORT);
                    app_data.renderer.dragged_component =
                        Draggable::INPORT(rl.get_mouse_position(), outport);
                    return true;
                }
            }
            GatePortType::OUTPORT => {
                app_data.connect_gates(&GATEPORT(hovered_port.clone()), &Port::MOUSEPORT);
                app_data.renderer.dragged_component =
                    Draggable::INPORT(rl.get_mouse_position(), GATEPORT(hovered_port));
                return true;
            } // GatePortType::INTERNAL => {
              //     gate.inputs[hovered_port.port_index].increment();
              // }
        }
    }
    false
}
