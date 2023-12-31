pub mod window;
pub mod shader;
pub mod color;
pub mod state;
pub mod event;

use state::{get_state, set_state};
use event::PEvent;

use self::event::PEventCallback;

pub type Callback = extern "C" fn() -> ();


#[no_mangle]
pub extern "C" fn p_init (setup: Callback, draw: Callback) {
    set_state!{
        setup = Some(setup);
        draw = Some(draw);
    }
}

#[no_mangle]
pub extern "C" fn p_run () {
    let setup: Callback = get_state().setup.expect("No setup function specified");
    setup();
    println!("Width: {}", get_state().width.expect("No width specified"));
    pollster::block_on(window::start_event_loop());
}

#[no_mangle]
pub extern "C" fn p_on (event: PEvent, callback: PEventCallback) {
    set_state! {
        events.insert(event, callback);
    }
}