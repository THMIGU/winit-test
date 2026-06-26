use wgpu::rwh::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

pub struct Context {
	pub surface: wgpu::Surface<'static>,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub config: wgpu::SurfaceConfiguration,
}

impl Context {
	pub async fn new(window: &Window) -> Self {
		let size = window.inner_size();
		let instance = wgpu::Instance::default();

		let display_handle = window
			.display_handle()
			.unwrap()
			.as_raw();
		let window_handle = window
			.window_handle()
			.unwrap()
			.as_raw();

		let surface_target = wgpu::SurfaceTargetUnsafe::RawHandle {
			raw_display_handle: Some(display_handle),
			raw_window_handle: window_handle,
		};

		let surface = unsafe {
			instance
				.create_surface_unsafe(surface_target)
				.unwrap()
		};

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
