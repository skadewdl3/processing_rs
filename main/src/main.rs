use processing::{*, core::{color, event::PEventData}};

extern "C" fn mouse_pressed_handler (data: PEventData) {
    println!("Mouse pressed: {:?}, {:?}", data.mouse_x, data.mouse_y);
}

extern "C" fn setup () {
    core::window::create_window(800, 800);
    core::window::background(core::color::Color::from_hex("#ffff00"));
    core::color::fill(color::Color::from_hex("#ff0000"));
}

static mut X: f32 = 400.0;
static mut INC: f32 = 10.0;

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
    core::p_on(core::event::PEvent::PMousePressed, mouse_pressed_handler);
    core::p_run();
}
