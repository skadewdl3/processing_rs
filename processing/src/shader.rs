use wgpu::{ShaderModule, RenderPipeline, Device, PipelineLayoutDescriptor, PipelineLayout};

pub struct Shader {
	pub module: ShaderModule,
	pub pipeline: RenderPipeline,
	pub vertex_count: u32
}

pub struct ShaderBuilder<'a> {
	label:Option<&'a str>,
	source: Option<&'a str>,
	pipeline_layout: Option<PipelineLayout>,
	device: &'a Device,
	vertex_count: Option<u32>
}

impl<'a> ShaderBuilder<'a> {
	pub fn new (device: &'a Device) -> ShaderBuilder<'_> {
		Self {
			label: None,
			source: None,
			pipeline_layout: None,
			vertex_count: None,
			device
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

	// TODO: Add render pipeline options later

	pub fn build (&self) -> Shader {
		let shader_string = std::fs::read_to_string(self.source.expect("No file path specified"))
			.expect("Could not read shader file");

		let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: self.label,
			source: wgpu::ShaderSource::Wgsl(shader_string.as_str().into()),
		});


		let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: self.label,
			layout: match &self.pipeline_layout {
				Some(x) => Some(x),
				None => None
			},

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
			vertex_count: self.vertex_count.expect("No vertex count specified")
		}
	}
}