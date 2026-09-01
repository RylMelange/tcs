mod app_data;
mod render;
mod simulation;
mod main_loop;

use crate::{
    app_data::AppData, main_loop::render_loop, simulation::{
        gates::GateID,
        simulator::InPort,
        wire_values::{TernaryType::*, TernaryValue},
    },
};

fn main() {
    let mut app_data = AppData::new();
    app_data.insert_gate(
        GateID(0),
        "source".to_string(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        vec![vec![InPort::new(GateID(2), 0)]],
    );
    app_data.insert_gate(
        GateID(1),
        "source".to_string(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        vec![vec![InPort::new(GateID(2), 1)]],
    );
    app_data.insert_gate(
        GateID(2),
        "and".to_string(),
        None,
        vec![vec![]],
    );

    app_data.simulator.insert_pending(GateID(0));
    app_data.simulator.insert_pending(GateID(1));

    render_loop(&mut app_data);
}
