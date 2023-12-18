use crate::{
    state::{set_state, get_state},
    shader::{ShaderBuilder, Vertex},
    shapes::triangle::TriangleUniforms
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder
};


pub type Callback = extern "C" fn() -> ();

#[no_mangle]
pub extern "C" fn p_init (setup: Callback, draw: Callback) {
    set_state!{
        setup = Some(setup);
        draw = Some(draw);
    }
}

#[no_mangle]
pub extern "C" fn create_window (width: u32, height: u32) {
    set_state! {
        width = Some(width);
        height = Some(height);
    };
    println!("Width: {}", get_state().width.expect("No width specified"));
}

#[no_mangle]
pub extern "C" fn p_run () {
    let setup: Callback = get_state().setup.expect("No setup function specified");
    setup();
    println!("Width: {}", get_state().width.expect("No width specified"));
    pollster::block_on(run());
}

async fn run () {
    let state = get_state();
    let width = state.width.expect("No width specified");
    let height = state.height.expect("No height specified");
    std::mem::drop(state);

    let event_loop = EventLoopBuilder::new().build();

    let window = WindowBuilder::new()
        .with_title("Processing")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();
    
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


    set_state! {
        window = Some(window);
        device = Some(device);
        queue = Some(queue);
        surface = Some(surface);
    };

    let shader = ShaderBuilder::new(&get_state().device.as_ref().unwrap())
        .with_label("triangle shader")
        .with_source("processing/src/shaders/triangle.wgsl")
        .with_vertex_buffer(vec![
            Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 0.0, 1.0, 1.0] },
            Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 1.0, 1.0] },
        ])
        .with_index_bufer(vec![0, 1, 3, 3, 1, 2])
        .build();

    set_state! {
        shaders.push(shader);
    }

    let state = get_state();

    event_loop.run(move |event, _, control_flow| {    

        
        let draw = state.draw.expect("No draw function specified");
        draw();

        let device = state.device.as_ref().expect("No device specified");
        let queue = state.queue.as_ref().expect("No queue specified");
        let surface = state.surface.as_ref().expect("No surface specified");


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

                    
                
                    for shader in state.shaders.iter() {
                        rpass.set_pipeline(&shader.pipeline);
                        // if let Some(x) = shader.
                        if shader.has_uniforms { rpass.set_bind_group(0, &shader.bind_group.as_ref().unwrap(), &[]) }

                        if let Some(vertex_buffer) = shader.vertex_buffer.as_ref() {
                            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        }

                        if let Some(index_buffer) = shader.index_buffer.as_ref() {
                            rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        }

                        if shader.has_index_buffer {
                            println!("this ran");
                            rpass.draw_indexed(0..shader.index_count.unwrap(), 0, 0..1);
                        }
                        else { rpass.draw(0..shader.vertex_count, 0..1) }
                        
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