mod shader_manager;
mod shader;
use shader_manager::ShaderManager;
use shader::ShaderBuilder;

use wgpu::{
    Backends,
    InstanceDescriptor
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{WindowBuilder, Window}
};

pub fn get_backends () {
    let instance = wgpu::Instance::new(
        InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        }
    );

    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }
}

async fn run (event_loop: EventLoop<()>, window: &Window) {      
    let size = window.inner_size();
    
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    
    let surface = unsafe { instance.create_surface(&window) }.expect("Could not create a surface");
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor::default(),
            None,
        )
        .await
        .expect("Failed to create device");

    let surface_caps = surface.get_capabilities(&adapter);
    
    let format = surface_caps.formats[0];

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    
    surface.configure(&device, &config);



    event_loop.run(move |event, _, control_flow| {    

        let mut shader_manager = ShaderManager::new(&device, &queue);
        // shader_manager.add_shader(shader)
        let mut shader_builder = ShaderBuilder::new(&device);

        let shader1 = shader_builder
            .with_label("Rainbow Triangle")
            .with_source("processing/src/shader.wgsl")
            .build();

        let shader2 = shader_builder
            .with_label("Red Triangle")
            .with_source("processing/src/shader2.wgsl")
            .build();

        shader_manager.add_shader(shader2);
        shader_manager.add_shader(shader1);

        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the surface with the new size
                instance.poll_all(true);
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
            }
            Event::RedrawRequested(_) => {
                let frame = surface.get_current_texture().unwrap();                
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {r: 0.05, g:0.062, b:0.08, a:1.0}),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        ..Default::default()
                    });
                
                    for shader in shader_manager.shaders.iter() {
                        rpass.set_pipeline(&shader.pipeline);
                        rpass.draw(0..3, 0..1);
                    }

                }
              
                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

pub fn test () {
    let event_loop = EventLoopBuilder::new().build();

    let window = WindowBuilder::new().
        with_title("My Window")
        .build(&event_loop)
        .expect("Could not create a window");

    env_logger::init();
    pollster::block_on(run(event_loop, &window));
}