use processing::*;

extern "C" fn setup () {
    core::create_window(800, 800);
}

extern "C" fn draw () {
}

fn main() {
    core::p_init(setup, draw);
    core::p_run();
}
