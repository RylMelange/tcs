mod common;
mod main_loop;
mod render;
mod simulation;
use crate::{
    common::{
        app_data::AppData,
        helpers::{
            GatePort,
            GatePortType::{INPORT, OUTPORT},
            Port,
        },
    },
    main_loop::render_loop,
    simulation::{
        gates::GateID,
        wire_values::{TernaryType::*, TernaryValue},
    },
};
use raylib::ffi::Vector2;

fn main() {
    let mut app_data = AppData::new();
    app_data.insert_gate(
        Some(GateID(0)),
        "source".to_string(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        Vector2::new(100.0, 100.0),
    );
    app_data.insert_gate(
        Some(GateID(1)),
        "source".to_string(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        Vector2::new(100.0, 300.0),
    );
    app_data.insert_gate(
        Some(GateID(2)),
        "rotand".to_string(),
        None,
        Vector2::new(300.0, 200.0),
    );

    app_data.connect_gates(
        &Port::GATEPORT(GatePort {
            gate_id: GateID(0),
            port_index: 0,
            port_type: OUTPORT,
        }),
        &Port::GATEPORT(GatePort {
            gate_id: GateID(2),
            port_index: 0,
            port_type: INPORT,
        }),
    );
    app_data.connect_gates(
        &Port::GATEPORT(GatePort {
            gate_id: GateID(1),
            port_index: 0,
            port_type: OUTPORT,
        }),
        &Port::GATEPORT(GatePort {
            gate_id: GateID(2),
            port_index: 1,
            port_type: INPORT,
        }),
    );

    app_data.simulator.insert_pending(GateID(0));
    app_data.simulator.insert_pending(GateID(1));

    render_loop(&mut app_data);
}
