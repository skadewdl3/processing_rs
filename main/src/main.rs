use processing::*;

extern "C" fn setup () {
    core::create_window(800, 800);
}

extern "C" fn draw () {
    shapes::rect::rect(400.0, 400.0, 100.0, 50.0);
}

fn main() {
    core::p_init(setup, draw);
    core::p_run();
}
