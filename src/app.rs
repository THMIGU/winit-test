use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{Window, WindowId},
};

#[derive(Default)]
pub struct App {
	window: Option<Window>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window_attributes = Window::default_attributes().with_title("winit-test");
		self.window = Some(
			event_loop
				.create_window(window_attributes)
				.unwrap(),
		);
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
			WindowEvent::RedrawRequested => {}
			_ => (),
		}
	}
}
