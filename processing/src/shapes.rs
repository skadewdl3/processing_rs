use crate::{
    shader::ShaderBuilder,
    state::get_state
};

#[no_mangle]
pub extern "C" fn rect () {
    let state = get_state();
    let device = state.device.as_ref().expect("No device specified");
    let shader = ShaderBuilder::new(&device)
        .with_label("Rect")
        .with_source("processing/shaders/rect.wgsl")
        .with_vertex_count(4)
        .build();
}