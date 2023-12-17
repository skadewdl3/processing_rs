use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use lazy_static::lazy_static;
use crate::{
    core::Callback,
    shader::Shader
};
use winit::window::Window;
use wgpu::{Device, Queue, Surface};

#[derive(Default)]
pub struct Bruh {
    pub bruh: u32
}

#[derive(Default)]
pub struct State {
    pub window: Option<Window>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub test: Bruh,
    pub draw: Option<Callback>,
    pub setup: Option<Callback>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub surface: Option<Surface>,
    pub shaders: Vec<Shader>
}

impl State {
    pub fn add_shader (&mut self, shader: Shader) {
        self.shaders.push(shader);
    }
}

// pub struct ShaderState 

lazy_static! {
    pub static ref STATE: RwLock<State> = RwLock::new(State::default());
}

pub fn get_state () -> RwLockReadGuard<'static, State> {
    STATE.try_read().expect("Could not read the RwLock")
}

macro_rules! set_state {

    // base cases
    ($var:ident$(.$var2:ident)* = $value:expr;) => {
        {
            crate::state::STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)* = $value;
        }
    };
    ($var:ident$(.$var2:ident)*($value:expr);) => {
        {
            crate::state::STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)*($value);
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
