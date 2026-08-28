mod app_data;
mod simulation;

use crate::{
    app_data::AppData,
    simulation::{
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

    println!("START:");
    println!("{:?}", app_data.simulator.gates);

    println!("STEPPING...");
    app_data.simulator.step(app_data.implementations);
    println!("END:");
    println!("{:?}", app_data.simulator.gates.get(&GateID(2)));
}
