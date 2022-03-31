use std::{time::Duration, thread::sleep};

use ferrux_viewport::error::ViewportError;
use ferrux_viewport::viewport::{ViewportFactory};
use rgb::{ComponentBytes, RGBA8};
use winit::{event_loop::EventLoop, dpi::LogicalSize, window::WindowBuilder};

fn main() -> Result<(), ViewportError> {
	let event_loop = EventLoop::new();
    let window = {
		let size: LogicalSize<i32> = LogicalSize::new(960, 720);
        WindowBuilder::new()
		.with_title("FerruX Viewport")
		.with_inner_size(size)
		.with_min_inner_size(size)
		.build(&event_loop)
		.unwrap()
    };

	let mut viewport = ViewportFactory::winit(&window)?;

	assert_eq!(960, viewport.width());
	assert_eq!(720, viewport.height());
	
	let color = [RGBA8::new(255, 255, 255, 255)];
	viewport.draw_point((0.0, 0.0, 0.0), color.as_bytes());

	viewport.render().unwrap();

	// TODO change to event loop execution
	sleep(Duration::MAX);
	Ok(())
}
