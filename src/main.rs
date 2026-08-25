mod simulation;
use std::collections::HashMap;

use crate::simulation::simulator::Simulator;

fn main() {
    let mut sim = Simulator::new();
    sim.step(HashMap::new());
}
