use std::time::{Instant, Duration};

use crate::core::{
    set_state, get_state,
    color::Color, event::{PEvent, PEventData, PMouseButton}
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder, monitor::MonitorHandle
};

#[no_mangle]
pub extern "C" fn create_window (width: u32, height: u32) {
    set_state! {
        width = Some(width);
        height = Some(height);
    };
    println!("Width: {}", get_state().width.expect("No width specified"));
}

#[no_mangle]
pub extern "C" fn background (color: Color) {
    set_state! {
        background = color;
    }
}

#[no_mangle]
pub extern "C" fn width () -> u32 {
    get_state().width.expect("No width specified")
}

#[no_mangle]
pub extern "C" fn height () -> u32 {
    get_state().height.expect("No height specified")
}

#[no_mangle]
pub extern "C" fn set_frame_rate (rate: u64) {
    if rate > get_state().max_fps { return }
    set_state! {
        target_fps = rate;
    }
}

pub extern "C" fn get_frame_rate () -> u64 {
    get_state().target_fps
}

pub async fn start_event_loop () {
    let state = get_state();
    let width = state.width.expect("No width specified");
    let height = state.height.expect("No height specified");
    drop(state);

    let event_loop = EventLoopBuilder::new().build();

    let window = WindowBuilder::new()
        .with_title("Processing")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();

    let x = window.available_monitors();
    let monitors: Vec<MonitorHandle> = x.collect();
    if monitors.len() > 0 {
        let m = monitors.get(0).unwrap();

        match m.refresh_rate_millihertz() {
            Some(rate) => {
                set_state! {
                    max_fps = rate as u64;
                }
            },
            _ => ()
        }
    }

    
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
        last_redraw_time = Some(Instant::now());
    };



    event_loop.run(move |event, _, control_flow| {    

        set_state! {
            shaders = vec![];
        }

        get_state().window.as_ref().expect("No window created!").request_redraw();


        let current_time = Instant::now();
        let delta = current_time.duration_since(get_state().last_redraw_time.unwrap());
        let target_delta = Duration::from_secs_f64(1.0 / get_state().target_fps as f64);


        if delta >= target_delta {

            let draw = get_state().draw.expect("No draw function specified");
            draw();
        
            let state = get_state();
            let device = state.device.as_ref().expect("No device specified");
            let queue = state.queue.as_ref().expect("No queue specified");
            let surface = state.surface.as_ref().expect("No surface specified");

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
                            load: wgpu::LoadOp::Clear(state.background.to_wgpu_color()),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None
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
                        rpass.draw_indexed(0..shader.index_count.unwrap(), 0, 0..1);
                    }
                    else { rpass.draw(0..shader.vertex_count, 0..1) }
                    
                }

            }
        
            queue.submit(Some(encoder.finish()));
            frame.present();
            drop(state);
            set_state! {
                last_redraw_time = Some(current_time);
            }
        }


        
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the surface with the new size
                let state = get_state();
                let device = state.device.as_ref().expect("No device specified");
                let surface = state.surface.as_ref().expect("No surface specified");
                instance.poll_all(true);
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,


            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::MouseInput { button, .. } => {

                        let state = get_state();
                        if let Some(handler) = state.events.get(&PEvent::PMousePressed) {
                            let data = PEventData {
                                event_type: PEvent::PMousePressed,
                                mouse_x: state.mouse_x as f32,
                                mouse_y: state.mouse_y as f32,
                                mouse_button: PMouseButton::from(button),
                            };
                            handler(data)
                        }
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        println!("Mouse moved: {:?}, {:?}", position.x, position.y);
                        set_state! {
                            mouse_x = position.x;
                            mouse_y = position.y;
                        }
                    }
                    _ => ()
                }
            }

            _ => ()
            
        }
    });


}