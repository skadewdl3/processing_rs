use crate::core::{
    shader::{ShaderBuilder, Uniforms},
    state::get_state
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniforms {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub x3: f32,
    pub y3: f32,
}

#[no_mangle]
pub extern "C" fn triangle () {
    let state = get_state();
    let device = state.device.as_ref().expect("No device specified");
    let _shader = ShaderBuilder::new(&device)
        .with_label("Triangle")
        .with_source("processing/src/shaders/triangle.wgsl")
        .with_vertex_count(4)
        .with_uniforms(Uniforms::Triangle(TriangleUniforms {
            x1: -0.5,
            y1: 0.0,
            x2: 0.5,
            y2: 0.0,
            x3: 0.0,
            y3: 0.5
        }))
        .build();
}