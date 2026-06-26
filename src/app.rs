use std::sync::Arc;

use winit::{
	application::ApplicationHandler,
	dpi::PhysicalSize,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{Window, WindowId},
};

use crate::renderer::Renderer;

enum AppState {
	Loading,
	Ready,
}

pub struct App {
	window: Option<Arc<Window>>,
	renderer: Renderer,
	state: AppState,
}

impl Default for App {
	fn default() -> Self {
		Self {
			window: None,
			renderer: Renderer::new(),
			state: AppState::Loading,
		}
	}
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		if self.window.is_some() {
			return;
		}

		#[allow(unused_mut)]
		let mut window_attributes = Window::default_attributes();

		#[cfg(not(target_arch = "wasm32"))]
		{
			window_attributes = window_attributes
				.with_title("winit-test")
				.with_inner_size(PhysicalSize::new(800, 600));
		}

		#[cfg(target_arch = "wasm32")]
		{
			use wasm_bindgen::JsCast;
			use web_sys::HtmlCanvasElement;
			use winit::platform::web::WindowAttributesExtWebSys;

			let canvas = wgpu::web_sys::window()
				.unwrap()
				.document()
				.unwrap()
				.get_element_by_id("canvas")
				.unwrap()
				.dyn_into::<HtmlCanvasElement>()
				.unwrap();

			window_attributes = window_attributes.with_canvas(Some(canvas));
		}

		let window = Arc::new(
			event_loop
				.create_window(window_attributes)
				.unwrap(),
		);
		self.window = Some(window);

		if let Some(window) = &self.window {
			self.renderer.init(window);
		}
	}

	fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
		self.window
			.as_ref()
			.unwrap()
			.request_redraw();
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => match self.state {
				AppState::Loading => {
					if self.renderer.poll_loading() {
						self.state = AppState::Ready;
						println!("Ready!");

						#[cfg(target_arch = "wasm32")]
						web_sys::console::log_1(&"Ready!".into());
					}
				}
				AppState::Ready => {}
			},
			_ => (),
		}
	}
}
