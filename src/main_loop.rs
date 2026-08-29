use crate::app_data::AppData;

pub fn render_loop(app_data: &mut AppData) {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Ternary Circuit Simulator")
        .build();

    while !rl.window_should_close() {
        app_data.simulator.step(&app_data.implementations);
        app_data.renderer.render_all(
            rl.begin_drawing(&thread),
            &mut app_data.simulator.graph,
            &mut app_data.simulator.gates,
        );
    }
}
