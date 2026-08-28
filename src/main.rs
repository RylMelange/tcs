mod simulation;

use crate::simulation::{
    gate_definitions::default_definitions, gates::GateID, simulator::{InPort, Simulator}, wire_values::{TernaryType::*, TernaryValue},
};

fn main() {
    let implementations = default_definitions();

    let mut sim = Simulator::new();
    sim.insert_gate(
        GateID(0),
        implementations.get("source").unwrap(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        vec![vec![InPort::new(GateID(2), 0)]],
    );
    sim.insert_gate(
        GateID(1),
        implementations.get("source").unwrap(),
        Some(vec![TernaryValue::new(Trit, 1)]),
        vec![vec![InPort::new(GateID(2), 1)]],
    );
    sim.insert_gate(
        GateID(2),
        implementations.get("and").unwrap(),
        None,
        vec![vec![]],
    );

    sim.insert_pending(GateID(0));
    sim.insert_pending(GateID(1));

    println!("START:");
    println!("{:?}", sim.gates);

    println!("STEPPING...");
    sim.step(implementations);
    println!("END:");
    // println!("{:?}", sim.gates);
    println!("{:?}", sim.gates.get(&GateID(2)));
}
