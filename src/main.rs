use winit::event_loop::EventLoop;

use crate::app::App;

mod app;

fn main() {
	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

	let mut app = App::default();
	let _ = event_loop.run_app(&mut app);
}
