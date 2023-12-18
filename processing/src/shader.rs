use wgpu::{ShaderModule, RenderPipeline, Device, PipelineLayoutDescriptor, PipelineLayout, util::DeviceExt};
use crate::{shapes::{
	rect::RectUniforms,
	point::{PointUniforms, self},
	triangle::TriangleUniforms
}, state::get_state};

pub enum Uniforms {
    Rect(RectUniforms),
	Point(PointUniforms),
	Triangle(TriangleUniforms)
}

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
	pub bind_group: wgpu::BindGroup
}

pub struct ShaderBuilder<'a> {
	label:Option<&'a str>,
	source: Option<&'a str>,
	pipeline_layout: Option<PipelineLayout>,
	device: &'a Device,
	vertex_count: Option<u32>,
	uniforms: Option<Uniforms>,
	bind_group: Option<wgpu::BindGroup>,
	bind_group_layout: Option<wgpu::BindGroupLayout>
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
			bind_group_layout: None
		}
	}

	pub fn with_label (&mut self, label: &'a str) -> &mut Self {
		self.label = Some(label);
		self
	}

	pub fn with_source (&mut self, source: &'a str) -> &mut Self {
		self.source = Some(source);
		self
	}

	pub fn with_pipeline_layout (&mut self, layout: PipelineLayoutDescriptor<'_>) -> &mut Self {
		let x = self.device.create_pipeline_layout(&layout);
		self.pipeline_layout = Some(x);
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

	// TODO: Add render pipeline options later

	pub fn build (&mut self) -> Shader {
		let shader_string = std::fs::read_to_string(self.source.expect("No file path specified"))
			.expect("Could not read shader file");

		let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: self.label,
			source: wgpu::ShaderSource::Wgsl(shader_string.as_str().into()),
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


		let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: self.label,
			layout: self.pipeline_layout.as_ref(),

			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[],
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
			bind_group: self.bind_group.take().expect("No bind group specified"),
			vertex_count: self.vertex_count.expect("No vertex count specified")
		}
	}
}