use crate::{common::app_data::AppData, render::interaction::handle_inputs};

pub fn render_loop(app_data: &mut AppData) {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Ternary Circuit Simulator")
        .build();

    while !rl.window_should_close() {
        app_data
            .simulator
            .step(&mut app_data.gates, &app_data.gate_definitions);
        handle_inputs(&mut rl, app_data);
        app_data.renderer.render_all(
            &mut rl,
            &thread,
            &app_data.gate_definitions,
            &app_data.gates,
        );
    }
}
