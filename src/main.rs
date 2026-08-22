mod simulation;
use crate::simulation::gates::*;

fn main() {
    let mut gate = Gate::new(0);
    gate.compute();
    let outputs = gate.outputs;
}
