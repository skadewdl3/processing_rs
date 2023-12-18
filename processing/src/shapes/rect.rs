use crate::{
    shader::{ShaderBuilder, Uniforms},
    state::get_state
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectUniforms {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[no_mangle]
pub extern "C" fn rect () {
    let state = get_state();
    let device = state.device.as_ref().expect("No device specified");
    let _shader = ShaderBuilder::new(&device)
        .with_label("Rect")
        .with_source("processing/src/shaders/rect.wgsl")
        .with_vertex_count(4)
        .with_uniforms(Uniforms::Rect(RectUniforms {
            x: 0.0,
            y: 0.0,
            width: 0.5,
            height: 0.5
        }))
        .build();
}