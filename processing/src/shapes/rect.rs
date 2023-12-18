use crate::{
    shader::ShaderBuilder,
    state::{get_state, set_state}
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectUniforms {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

macro_rules! vtx {
    (position: $pos:expr, color: $col:expr) => {
        crate::shader::Vertex {
            position: $pos,
            color: $col
        }
    };
}

// macro to convert vertex position to -1 to 1 coordinates
macro_rules! normalized_vtx {
    (position: $pos:expr, color: $col:expr) => {
        crate::shader::Vertex {
            position: [
                $pos[0] / crate::state::get_state().width.expect("Width of window has not been set") as f32 * 2.0 - 1.0,
                $pos[1] / crate::state::get_state().height.expect("Height of window has not been set") as f32 * 2.0 - 1.0,
                0.0
            ],
            color: $col
        }
    };
}

#[no_mangle]
pub extern "C" fn rect (x: f32, y: f32, width: f32, height: f32) {
    let state = get_state();
    let device = state.device.as_ref().expect("No device specified");


    let bottom_left = normalized_vtx!(position: [x, y + height, 0.0], color: [1.0, 0.0, 0.0, 1.0]);
    let bottom_right = normalized_vtx!(position: [x + width, y + height, 0.0], color: [0.0, 1.0, 0.0, 1.0]);
    let top_right = normalized_vtx!(position: [x + width, y, 0.0], color: [1.0, 1.0, 1.0, 1.0]);
    let top_left = normalized_vtx!(position: [x, y, 0.0], color: [0.0, 0.0, 1.0, 1.0]);

    // convert to normalized device coordinates
    

    let shader = ShaderBuilder::new(&device)
        .with_label("Rect")
        .with_source("processing/src/shaders/rect.wgsl")
        .with_vertex_buffer(vec![bottom_left, bottom_right, top_right, top_left])
        .with_index_buffer(vec![0, 1, 2, 2, 3, 0])
        .build();

    drop(state);

    set_state! {
        shaders.push(shader);
    }
}