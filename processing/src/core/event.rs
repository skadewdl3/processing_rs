#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub enum PEvent {
    PMousePressed,
    #[default]
    None
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

#[repr(C)]
#[derive(Default)]
pub enum PMouseButton {
    Left,
    Right,
    Middle,
    #[default]
    None
}

impl PMouseButton {
    pub fn from (button: winit::event::MouseButton) -> Self {
        match button {
            winit::event::MouseButton::Left => PMouseButton::Left,
            winit::event::MouseButton::Right => PMouseButton::Right,
            winit::event::MouseButton::Middle => PMouseButton::Middle,
            _ => PMouseButton::None
        }
    }
}

