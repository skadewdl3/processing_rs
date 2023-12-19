use hex_color::HexColor;
use std::ffi::{CStr, c_char};

#[repr(C)]
#[derive(Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn from_hex (code: &str) -> Self {
        let color = HexColor::parse(code).unwrap();
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: 255
        }
    }

    pub fn from_rgb (r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b, a: 255
        }

    }

    pub fn from_rgba (r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r, g, b, a
        }

    }

    pub fn to_wgpu_color (&self) -> wgpu::Color {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;
        let a = self.a as f64 / 255.0;
        wgpu::Color { r, g, b, a }
    }
}

pub extern "C" fn color_rgb (r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r, g, b)
}

pub extern "C" fn color_rgba (r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

pub extern "C" fn color_hex (code: *const c_char) -> Color {
    let code = unsafe { CStr::from_ptr(code) };
    let code = code.to_str().unwrap();
    Color::from_hex(code)
}