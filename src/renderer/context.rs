use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

pub struct Context {
	pub surface: wgpu::Surface<'static>,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub config: wgpu::SurfaceConfiguration,
}

impl Context {
	pub fn create_surface(
		window: &Arc<Window>,
	) -> (wgpu::Surface<'static>, wgpu::Instance, PhysicalSize<u32>) {
		let mut size = window.inner_size();
		if size.width == 0 || size.height == 0 {
			size.width = 800;
			size.height = 600;
		}

		let instance = wgpu::Instance::default();

		let surface = instance
			.create_surface(window.clone())
			.unwrap();

		(surface, instance, size)
	}

	pub async fn new(
		instance: wgpu::Instance,
		surface: wgpu::Surface<'static>,
		size: PhysicalSize<u32>,
	) -> Self {
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(&wgpu::DeviceDescriptor::default())
			.await
			.unwrap();

		let caps = surface.get_capabilities(&adapter);
		let surface_format = caps.formats[0];
		let alpha_mode = caps.alpha_modes[0];

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
			alpha_mode: alpha_mode,
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
		};

		surface.configure(&device, &config);

		Self {
			surface,
			device,
			queue,
			config,
		}
	}
}
