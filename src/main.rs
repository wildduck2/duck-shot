use dirs::home_dir;
use image::{ImageBuffer, Rgba};
use std::io::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, ptr};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use x11::xlib::*;

#[tokio::main]
async fn main() -> Result<()> {
    // CONNECT TO X11 SERVER.
    let display: *mut _XDisplay = unsafe { XOpenDisplay(ptr::null()) };
    if display.is_null() {
        eprintln!("failed to connect to X11 server with the default display");
        return Ok(());
    };

    // GET VALUES.
    let screen: i32 = unsafe { XDefaultScreen(display) };
    let root_window: u64 = unsafe { XRootWindow(display, screen) };
    let width: i32 = unsafe { XDisplayWidth(display, screen) };
    let height: i32 = unsafe { XDisplayHeight(display, screen) };

    // GET IMAGE FORM X11
    let image: *mut XImage = unsafe {
        XGetImage(
            display,
            root_window,
            0,
            0,
            width as u32,
            height as u32,
            XAllPlanes(),
            ZPixmap,
        )
    };
    if image.is_null() {
        eprintln!("failed to get image from X11 server");
        unsafe { XCloseDisplay(display) };
        return Ok(());
    }

    // EXTRACTING IMAGE BUFFER AND FILL IT WITH DATA
    let img = extract_image_buffer(image);

    // DISPLAYING IMG
    let selected_img_boundaries = display_fullscreen_image(img).await;

    // Saving img.
    let home = home_dir().expect("failed to find home directory!!");
    let screenshots_dir = home.join("Pictures").join("Screenshots");
    if screenshots_dir.exists() {
        fs::create_dir_all(&screenshots_dir)?;
    }

    let timestamps = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("failed to get time stamp");
    let filename = format!("duck-shot-{}.png", timestamps.as_secs());
    let filepath = screenshots_dir.join(&filename);

    // img.save(&filepath).expect("failed to save image");

    // Clearing out
    unsafe {
        XDestroyImage(image);
        XCloseDisplay(display);
    }

    Ok(())
}

fn extract_image_buffer(image: *mut XImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Getting Imgae data.
    let bytes_per_pixel: i32 = unsafe { (*image).bits_per_pixel / 8 };
    let img_data = unsafe {
        std::slice::from_raw_parts(
            (*image).data as *const u8,
            ((*image).width * (*image).height * bytes_per_pixel) as usize,
        )
    };

    // Creating image buffer.
    let bytes_per_pixel_usize: usize = bytes_per_pixel
        .try_into()
        .expect("failed to convert bytes_per_pixel to usize!!");
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(unsafe { (*image).width as u32 }, unsafe {
        (*image).height as u32
    });

    // Filling img buffer with img data.
    unsafe {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = (y * (*image).width as u32 + x) as usize * bytes_per_pixel_usize;
            let chunk = &img_data[index..index + bytes_per_pixel_usize];
            *pixel = Rgba([chunk[2], chunk[1], chunk[0], 255])
        }
    };

    img
}

use std::num::NonZeroU32;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, WindowBuilder};

async fn display_fullscreen_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> u32 {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .expect("Failed to create a window");

    let backend = wgpu::Backends::all();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
    });
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let img_width = img.width();
    let img_height = img.height();

    let surface_format = surface.get_capabilities(&adapter).formats[0];
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: img_width,
        height: img_height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let raw_image_data = img.into_raw();

    let texture_size = wgpu::Extent3d {
        width: img_width,
        height: img_height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Screen Texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &raw_image_data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(NonZeroU32::new(4 * img_width).unwrap()),
            rows_per_image: Some(NonZeroU32::new(img_height).unwrap()),
        },
        texture_size,
    );

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Texture Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Uint,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&device.create_sampler(
                    &wgpu::SamplerDescriptor {
                        label: Some("Texture Sampler"),
                        ..Default::default()
                    },
                )),
            },
        ],
        label: Some("Bind Group"),
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    // Flag to control if the screen should be cleared
    let mut clear_screen = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    state: ElementState::Pressed,
                    ..
                } = input
                {
                    // Toggle clear screen flag
                    clear_screen = !clear_screen;
                }
            }
            Event::RedrawRequested(_) => {
                let frame = surface.get_current_texture().unwrap();
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: if clear_screen {
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), // Clear to white or any other color
                                    store: true,
                                }
                            } else {
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                }
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_bind_group(0, &bind_group, &[]);
                    render_pass.draw(0..6, 0..1); // Draw the texture
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
