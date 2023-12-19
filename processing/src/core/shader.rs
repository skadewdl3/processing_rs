use wgpu::{ShaderModule, RenderPipeline, Device, PipelineLayout, util::DeviceExt};
use crate::shapes::{
	rect::RectUniforms,
	point::PointUniforms,
	triangle::TriangleUniforms
};

pub enum Uniforms {
    Rect(RectUniforms),
	Point(PointUniforms),
	Triangle(TriangleUniforms)
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
	pub position: [f32; 3],
	pub color: [f32; 4],
}

// macro to convert vertex position to -1 to 1 coordinates
// world space: x axis left to right, y axis top to bottom. origin is top-left corner
// normalized device coordinates: x axis left to right, y axis top to bottom. origin is center. range is -1 to 1
// convert world space to normalized device coordinates

macro_rules! normalized_vtx {
    (position: $pos:expr, color: $col:expr) => {
        crate::core::shader::Vertex {
            position: [
                $pos[0] / crate::core::state::get_state().width.expect("Width of window has not been set") as f32 * 2.0 - 1.0,
                -($pos[1] / crate::core::state::get_state().height.expect("Height of window has not been set") as f32 * 2.0 - 1.0),
                0.0
            ],
            color: $col
        }
    };
}

pub(crate) use normalized_vtx;

impl Uniforms {
	fn get_contents(&self) -> Vec<u8> {
		match self {
			Uniforms::Rect(rect_uniforms) => bytemuck::cast_slice(&[*rect_uniforms]).to_vec(),
			Uniforms::Point(point_uniforms) => bytemuck::cast_slice(&[*point_uniforms]).to_vec(),
			Uniforms::Triangle(triangle_uniforms) => bytemuck::cast_slice(&[*triangle_uniforms]).to_vec(),
		}
	}
}

pub struct Shader {
	pub module: ShaderModule,
	pub pipeline: RenderPipeline,
	pub vertex_count: u32,
	pub bind_group: Option<wgpu::BindGroup>,
	pub vertex_buffer: Option<wgpu::Buffer>,
	pub index_buffer: Option<wgpu::Buffer>,
	pub index_count: Option<u32>,


	pub has_index_buffer: bool,
	pub has_vertex_buffer: bool,
	pub has_uniforms: bool
}

pub struct ShaderBuilder<'a> {
	label:Option<&'a str>,
	source: Option<&'a str>,
	content: Option<String>,
	pipeline_layout: Option<PipelineLayout>,
	device: &'a Device,
	uniforms: Option<Uniforms>,
	bind_group: Option<wgpu::BindGroup>,
	bind_group_layout: Option<wgpu::BindGroupLayout>,
	vertex_buffer: Option<wgpu::Buffer>,
	vertex_count: Option<u32>,
	index_buffer: Option<wgpu::Buffer>,
	index_count: Option<u32>
}

impl<'a> ShaderBuilder<'a> {
	pub fn new (device: &'a Device) -> ShaderBuilder<'_> {
		Self {
			label: None,
			source: None,
			pipeline_layout: None,
			vertex_count: None,
			device,
			uniforms: None,
			bind_group: None,
			bind_group_layout: None,
			vertex_buffer: None,
			index_buffer: None,
			index_count: None,
			content: None
		}
	}

	pub fn with_label (&mut self, label: &'a str) -> &mut Self {
		self.label = Some(label);
		self
	}

	pub fn with_content (&mut self, content: &'a str) -> &mut Self {
		self.content = Some(String::from(content));
		self
	}

	pub fn with_source (&mut self, source: &'a str) -> &mut Self {
		self.source = Some(source);
		self
	}

	pub fn with_vertex_count (&mut self, count: u32) -> &mut Self {
		self.vertex_count = Some(count);
		self
	}

	pub fn with_uniforms (&mut self, uniforms: Uniforms) -> &mut Self {
		self.uniforms = Some(uniforms);

		let buffer = self.device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Uniform Buffer"),
				contents: &self.uniforms.as_ref().unwrap().get_contents(),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
		);

		let bind_group_layout = self.device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor {
				label: Some("bind group layout"),
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None
						},
						count: None
					}
				]
			}
		);


		let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("bind group"),
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: buffer.as_entire_binding()
				}
			]
		});

		self.bind_group_layout = Some(bind_group_layout);
		self.bind_group = Some(bind_group);

		self
	}

	pub fn with_vertex_buffer (&mut self, vertices: Vec<Vertex>) -> &mut Self {
		let vertex_buffer = self.device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: bytemuck::cast_slice(&vertices),
				usage: wgpu::BufferUsages::VERTEX,
			}
		);

		self.vertex_buffer = Some(vertex_buffer);
		self.vertex_count = Some(vertices.len() as u32);
		self
	}

	pub fn with_index_buffer (&mut self, indices: Vec<u16>) -> &mut Self {
		let index_buffer = self.device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Index Buffer"),
				contents: bytemuck::cast_slice(&indices),
				usage: wgpu::BufferUsages::INDEX,
			}
		);

		self.index_buffer = Some(index_buffer);
		self.index_count = Some(indices.len() as u32);
		self
	}

	pub fn build (&mut self) -> Shader {
		
		if self.content.is_none() {
			self.content = Some(std::fs::read_to_string(self.source.expect("No path to shader file specified"))
			.expect("Could not read shader file"));
		}

		let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: self.label,
			source: wgpu::ShaderSource::Wgsl(self.content.take().unwrap().as_str().into()),
		});


		let pipeline_layout: Option<PipelineLayout> = match self.bind_group.as_ref() {
			Some(_) => {
				Some(self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
					label: Some("pipline layout"),
					bind_group_layouts: &[
						self.bind_group_layout.as_ref().unwrap()
					],
					push_constant_ranges: &[]
				}))
			}
			None => None
		};
		self.pipeline_layout = pipeline_layout;

		
		
		let mut buffers: Vec<wgpu::VertexBufferLayout> = vec![];
		let attributes = &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];
		if self.vertex_buffer.is_some() {
			let x = wgpu::VertexBufferLayout {
				array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
				step_mode: wgpu::VertexStepMode::Vertex,
				attributes
			};
			buffers.push(x);
		}
		

		let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: self.label,
			layout: self.pipeline_layout.as_ref(),

			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &buffers,
			},

			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: wgpu::TextureFormat::Bgra8UnormSrgb,
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::REPLACE,
						alpha: wgpu::BlendComponent::REPLACE,
					}),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState::default(),
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
		});

		Shader {
			module: shader,
			pipeline,
			bind_group: self.bind_group.take(),
			vertex_count: self.vertex_count.take().expect("No vertex count specified"),
			
			//  determine whether buffer exists before taking them
			has_index_buffer: self.index_buffer.is_some(),
			has_vertex_buffer: self.vertex_buffer.is_some(),
			has_uniforms: self.uniforms.is_some(),

			// take buffers
			vertex_buffer: self.vertex_buffer.take(),
			index_count: self.index_count.take(),
			index_buffer: self.index_buffer.take(),
		}
	}
}
