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
    app_data.simulator.insert_gate(
        GateID(0),
        app_data.implementations.get("source").unwrap(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        vec![vec![InPort::new(GateID(2), 0)]],
    );
    app_data.simulator.insert_gate(
        GateID(1),
        app_data.implementations.get("source").unwrap(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        vec![vec![InPort::new(GateID(2), 1)]],
    );
    app_data.simulator.insert_gate(
        GateID(2),
        app_data.implementations.get("and").unwrap(),
        None,
        vec![vec![]],
    );

    app_data.simulator.insert_pending(GateID(0));
    app_data.simulator.insert_pending(GateID(1));

    render_loop(&mut app_data);
}
