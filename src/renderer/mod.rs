pub mod context;

use std::sync::{Arc, mpsc};

use winit::window::Window;

use crate::renderer::context::Context;

pub struct Renderer {
	context: Option<Context>,
	context_rx: Option<mpsc::Receiver<Context>>,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			context: None,
			context_rx: None,
		}
	}

	pub fn init(&mut self, window: &Arc<Window>) {
		let (surface, instance, size) = Context::create_surface(window);

		let (tx, rx) = mpsc::channel::<Context>();
		self.context_rx = Some(rx);

		#[cfg(not(target_arch = "wasm32"))]
		std::thread::spawn(move || {
			let context = pollster::block_on(Context::new(instance, surface, size));
			tx.send(context).unwrap();
		});

		#[cfg(target_arch = "wasm32")]
		wasm_bindgen_futures::spawn_local(async move {
			let context = Context::new(instance, surface, size).await;
			tx.send(context).unwrap();
		});
	}

	pub fn poll_loading(&mut self) -> bool {
		if let Some(rx) = &self.context_rx {
			if let Ok(context) = rx.try_recv() {
				self.context = Some(context);
				self.context_rx = None;
				return true;
			}
		}

		false
	}
}
