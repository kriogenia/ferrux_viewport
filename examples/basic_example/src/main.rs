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

	let mut viewport = ViewportFactory::winit(&window, 100)?;

	assert_eq!(960, viewport.width());
	assert_eq!(720, viewport.height());
	assert_eq!(100, viewport.depth());

	// Basic creation of the colors to use using the `rgb` crate
	let white = [RGBA8::new(255, 255, 255, 255)];
	let white_low_alpha = [RGBA8::new(255, 255, 255, 25)];
	let red = [RGBA8::new(255, 0, 0, 255)];
	let yellow = [RGBA8::new(255, 255, 0, 255)];
	let green = [RGBA8::new(0, 255, 0, 255)];
	let cyan = [RGBA8::new(0, 255, 255, 255)];
	let blue = [RGBA8::new(0, 0, 255, 255)];

	// Figure drawing
	viewport.fill_triangle((0.0, 0.0, 0.0), (1.0, -0.5, 0.0), (1.0, -0.3, 0.0), red.as_bytes());
	viewport.fill_triangle((0.0, 0.0, 0.0), (1.0, -0.3, 0.0), (1.0, -0.1, 0.0), yellow.as_bytes());
	viewport.fill_triangle((0.0, 0.0, 0.0), (1.0, -0.1, 0.0), (1.0, 0.1, 0.0), green.as_bytes());
	viewport.fill_triangle((0.0, 0.0, 0.0), (1.0, 0.1, 0.0), (1.0, 0.3, 0.0), cyan.as_bytes());
	viewport.fill_triangle((0.0, 0.0, 0.0), (1.0, 0.3, 0.0), (1.0, 0.5, 0.0), blue.as_bytes());
	viewport.fill_triangle((0.0, -0.25, 0.0), (-0.25, 0.25, 0.0), (0.25, 0.25, 0.0), white_low_alpha.as_bytes());
	viewport.draw_triangle((0.0, -0.25, 0.0), (-0.25, 0.25, 0.0), (0.25, 0.25, 0.0), white.as_bytes());
	viewport.draw_line((-1.0, 0.25, 0.0), (0.0, 0.0, 0.0), white.as_bytes());
	
	// Figure rendering
	viewport.render().unwrap();

	// TODO change to event loop execution
	sleep(Duration::MAX);
	Ok(())
}
