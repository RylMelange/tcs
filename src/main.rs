mod simulation;
use std::collections::HashMap;

use crate::simulation::{
    gate_definitions::{BuiltinGate, GateDefinition, GateImplementation::Builtin},
    gates::GateID,
    simulator::{InPort, Simulator},
    wire_values::{TernaryType::*, TernaryValue},
};

fn main() {
    let mut implementations = HashMap::new();
    let increment_definition = GateDefinition::new(
        "increment",
        vec![Trit],
        vec![Trit],
        Builtin(BuiltinGate {
            compute_function: |a| vec![a[0] + 1],
        }),
    );
    implementations.insert("increment".to_string(), increment_definition);

    let and_definition = GateDefinition::new(
        "and",
        vec![Trit; 2],
        vec![Trit],
        Builtin(BuiltinGate {
            compute_function: |a| {
                if a[0] == a[1] {
                    vec![a[0] + 1]
                } else {
                    vec![TernaryValue::new(Trit, 0)]
                }
            },
        }),
    );
    implementations.insert("and".to_string(), and_definition);

    let mut sim = Simulator::new();
    sim.insert_gate(
        GateID(0),
        implementations.get("increment").unwrap(),
        vec![vec![InPort::new(GateID(2), 0)]],
    );
    sim.insert_gate(
        GateID(1),
        implementations.get("increment").unwrap(),
        vec![vec![InPort::new(GateID(2), 1)]],
    );
    sim.insert_gate(GateID(2), implementations.get("and").unwrap(), vec![vec![]]);

    sim.insert_pending(GateID(0));
    sim.insert_pending(GateID(1));

    println!("START:");
    println!("{:?}", sim.gates);

    println!("STEPPING...");
    sim.step(implementations);
    println!("END:");
    println!("{:?}", sim.gates);
}
