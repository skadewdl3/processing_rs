use crate::core::{
    shader::{ShaderBuilder, normalized_vtx, Uniforms},
    state::{get_state, set_state}
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectUniforms {
    pub stroke: [f32; 4],
    pub fill: [f32; 4],
}

#[no_mangle]
pub extern "C" fn rect (x: f32, y: f32, width: f32, height: f32) {
    let state = get_state();
    let device = state.device.as_ref().expect("No device specified");



    let bottom_left = normalized_vtx!(x, y + height);
    let bottom_right = normalized_vtx!(x + width, y + height);
    let top_right = normalized_vtx!(x + width, y);
    let top_left = normalized_vtx!(x, y);
    

    let shader = ShaderBuilder::new(&device)
        .with_label("Rect")
        .with_content(include_str!("../shaders/rect.wgsl"))
        .with_vertex_buffer(vec![bottom_left, bottom_right, top_right, top_left])
        .with_index_buffer(vec![0, 1, 2, 2, 3, 0])
        .with_uniforms(Uniforms::Rect(RectUniforms {
            stroke: state.stroke.to_array(),
            fill: state.fill.to_array()
        }))
        .build();

    drop(state);

    set_state! {
        shaders.push(shader);
    }
}


#[no_mangle]
pub extern "C" fn square (x: f32, y: f32, side: f32) {
    let state = get_state();
    let device = state.device.as_ref().expect("No device specified");



    let bottom_left = normalized_vtx!(x, y + side);
    let bottom_right = normalized_vtx!(x + side, y + side);
    let top_right = normalized_vtx!(x + side, y);
    let top_left = normalized_vtx!(x, y);
    

    let shader = ShaderBuilder::new(&device)
        .with_label("Rect")
        .with_content(include_str!("../shaders/rect.wgsl"))
        .with_vertex_buffer(vec![bottom_left, bottom_right, top_right, top_left])
        .with_index_buffer(vec![0, 1, 2, 2, 3, 0])
        .with_uniforms(Uniforms::Rect(RectUniforms {
            stroke: state.stroke.to_array(),
            fill: state.fill.to_array()
        }))
        .build();

    drop(state);

    set_state! {
        shaders.push(shader);
    }
}