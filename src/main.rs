mod core;
mod inputs;
mod tui;

fn main() {
    let mut state = inputs::start();
    let speed = state.1;

    tui::display_frame(state.0.clone(), 0);

    // display other states
    for generation in 1.. {
        let points = core::algorithm(state.0.clone(), state.1);
        state = (points.clone(), speed);
        tui::display_frame(points, generation);
    }
}
