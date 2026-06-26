mod app;
mod renderer;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
	#[cfg(target_arch = "wasm32")]
	console_error_panic_hook::set_once();

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

	let mut app = App::default();
	let _ = event_loop.run_app(&mut app);
}
