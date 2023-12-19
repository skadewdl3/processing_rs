use processing::*;

extern "C" fn setup () {
    core::window::create_window(800, 800);
    core::window::background(core::color::Color::from_hex("#ffff00"));
    core::window::set_frame_rate(20);
}

static mut X: f32 = 400.0;
static mut INC: f32 = 1.0;

extern "C" fn draw () {
    unsafe {
        if X <= 0.0 || (X + 100.0) >= core::window::width() as f32 {
            INC *= -1.0;
        }
        shapes::rect::rect(X, 400.0, 100.0, 50.0);
        X += INC;
    }
}

fn main() {
    core::p_init(setup, draw);
    core::p_run();
}
