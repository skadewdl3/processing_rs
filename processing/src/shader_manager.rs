use wgpu::{Device, Queue};
use crate::shader::Shader;

pub struct ShaderManager<'a> {
    pub shaders: Vec<Shader>,
    queue: &'a Queue,
    // pipelines: Vec<Pipeline>,
    device: &'a Device
}

impl<'a> ShaderManager<'a> {
    pub fn new (device: &'a Device, queue: &'a Queue) -> Self {
        Self {
            device,
            queue,
            shaders: vec![]
        }
    }

    pub fn add_shader (&mut self, shader: Shader) {
        self.shaders.push(shader);
    }
}