use crate::core::{
    shader::ShaderBuilder,
    state::get_state
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointUniforms {
    pub x: f32,
    pub y: f32,
}

#[no_mangle]
pub extern "C" fn point () {
    let state = get_state();
    let device = state.device.as_ref().expect("No device specified");
    let _shader = ShaderBuilder::new(&device)
        .with_label("Rect")
        .with_source("processing/src/shaders/rect.wgsl")
        .with_vertex_count(4)
        .build();
}