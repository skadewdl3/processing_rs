use super::state::get_state;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub enum PEvent {
    PMousePressed,
    PMouseReleased,
    PMouseMoved,
    #[default]
    NoEvent
}

pub type PEventCallback = extern "C" fn(PEventData) -> ();

#[repr(C)]
#[derive(Default)]
pub struct PEventData {
    pub event_type: PEvent,
    pub mouse_button: PMouseButton,
    pub mouse_x: f32,
    pub mouse_y: f32
}

impl PEventData {
    pub fn from_mouse_event (event_type: PEvent, mouse_button: PMouseButton) -> Self {
        let state = get_state();
        Self {
            event_type,
            mouse_button,
            mouse_x: state.mouse_x as f32,
            mouse_y: state.mouse_y as f32,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Default, Debug)]
pub enum PMouseButton {
    Left,
    Right,
    Middle,
    #[default]
    NoButton
}

impl PMouseButton {
    pub fn from (button: winit::event::MouseButton) -> Self {
        match button {
            winit::event::MouseButton::Left => PMouseButton::Left,
            winit::event::MouseButton::Right => PMouseButton::Right,
            winit::event::MouseButton::Middle => PMouseButton::Middle,
            _ => PMouseButton::NoButton
        }
    }
}

#[no_mangle]
pub extern "C" fn mouseX () -> f64 {
    let state = get_state();
    state.mouse_x
} 

#[no_mangle]
pub extern "C" fn mouseY () -> f64 {
    let state = get_state();
    state.mouse_y
}