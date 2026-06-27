mod cli_display;
mod core;
mod inputs;

fn main() {
    let mut state = inputs::start();
    let speed = state.1;
    let x = state.2;
    let y = state.3;
    cli_display::display_frame(state.0.clone(), 0, x, y);

    // display other states
    for generation in 1.. {
        let points = core::algorithm(state.0.clone(), state.1);
        state = (points.clone(), speed, x, y);
        cli_display::display_frame(points, generation, x, y);
        if state.0.is_empty() {
            println!();
            println!("All cells died, simulation ended.");
            println!();
            break;
        }
    }
}
