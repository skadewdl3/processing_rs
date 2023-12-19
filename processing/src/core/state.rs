use std::{sync::{RwLock, RwLockReadGuard}, time::Instant};
use lazy_static::lazy_static;
use crate::core::{
    Callback,
    shader::Shader,
    color::Color
};
use winit::window::Window;
use wgpu::{Device, Queue, Surface};


#[derive(Default)]
pub struct State {
    pub window: Option<Window>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub draw: Option<Callback>,
    pub setup: Option<Callback>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub surface: Option<Surface>,
    pub shaders: Vec<Shader>,
    pub stroke: Color,
    pub fill: Color,
    pub background: Color,
    pub target_fps: u64,
    pub last_redraw_time: Option<Instant>,
    pub max_fps: u64
}

lazy_static! {
    pub static ref STATE: RwLock<State> = RwLock::new(State {
        stroke: Color::from_hex("#000000"),
        fill: Color::from_hex("#000000"),
        background: Color::from_hex("#ffffff"),
        target_fps: 60,
        max_fps: 60,
        ..Default::default()
    });
}

pub fn get_state () -> RwLockReadGuard<'static, State> {
    STATE.try_read().expect("Could not read the RwLock")
}

macro_rules! set_state {

    // base cases
    ($var:ident$(.$var2:ident)* = $value:expr;) => {
        {
            crate::core::state::STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)* = $value;
        }
    };
    ($var:ident$(.$var2:ident)*($value:expr);) => {
        {
            crate::core::state::STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)*($value);
        }
    };

    // expr - expr
    ($var:ident$(.$var2:ident)* = $value:expr; $($var3:ident$(.$var4:ident)* = $value2:expr;)*) => {
        set_state!{ $var$(.$var2)* = $value; };
        set_state!{ $($var3$(.$var4)* = $value2;)* };
    };

    // fn - expr
    ($var:ident$(.$var2:ident)*($value:expr); $($var3:ident$(.$var4:ident)* = $value2:expr;)*) => {
        set_state!{ $var$(.$var2)*($value); };
        set_state!{ $($var3$(.$var4)* = $value2;)* };
    };

    // expr - fn
    ($var:ident$(.$var2:ident)* = $value:expr; $($var3:ident$(.$var4:ident)*($value2:expr);)*) => {
        set_state!{ $var$(.$var2)* = $value; };
        set_state!{ $($var3$(.$var4)*($value2);)* };
    };  

    // fn - fn
    ($var:ident$(.$var2:ident)*($value:expr); $($var3:ident$(.$var4:ident)*($value2:expr);)*) => {
        set_state!{ $var$(.$var2)*($value); };
        set_state!{ $($var3$(.$var4)*($value2);)* };
    };  
}


pub(crate) use set_state;
