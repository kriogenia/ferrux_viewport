use ferrux_viewport::error::ViewportError;
use ferrux_viewport::viewport::{ViewportFactory};
use rgb::{ComponentBytes, RGBA8, RGBA};
use winit::event::Event;
use winit::{event_loop::EventLoop, dpi::LogicalSize, window::WindowBuilder};

// Definition of the colors to use using the `rgb` crate
static WHITE: [RGBA<u8>; 1] = [RGBA8::new(255, 255, 255, 255)];
static WHITE_LOW_ALPHA: [RGBA<u8>; 1] = [RGBA8::new(255, 255, 255, 25)];
static RED: [RGBA<u8>; 1] = [RGBA8::new(255, 0, 0, 255)];
static YELLOW: [RGBA<u8>; 1] = [RGBA8::new(255, 255, 0, 255)];
static GREEN: [RGBA<u8>; 1] = [RGBA8::new(0, 255, 0, 255)];
static CYAN: [RGBA<u8>; 1] = [RGBA8::new(0, 255, 255, 255)];
static BLUE: [RGBA<u8>; 1] = [RGBA8::new(0, 0, 255, 255)];

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


	let mut i = -1.0;
	let step = 0.05;
	let mut incrementing = true;

	event_loop.run(move |event, _, _| {
	  match event {
	    Event::MainEventsCleared => {
			i += if incrementing { step } else { -step };
			if i >= 1.0 {
				incrementing = false
			} else if i <= -1.0 {
				incrementing = true
			}
			window.request_redraw();
	    }
	    Event::RedrawRequested(_) => {   
			let (left, right) = if i > 0.0 { (1.0, i) } else { (i + 1.0, 0.0) }; 
			let right_y = right / 2.0;  
			// White line drawing
			viewport.draw_line((-1.0, 0.25, 0.1), (-1.0 + left, 0.25 - left / 4.0, 0.25), WHITE.as_bytes());
			// Center triangle drawing with fill and border
			viewport.fill_triangle((0.0, -0.25, 0.0), (-0.25, 0.25, 0.0), (0.25, 0.25, 0.0), WHITE_LOW_ALPHA.as_bytes());
			viewport.draw_triangle((0.0, -0.25, 0.01), (-0.25, 0.25, 0.01), (0.25, 0.25, 0.01), WHITE.as_bytes());
			// Colored triangles drawing
			viewport.fill_triangle((0.0, 0.0, -0.1), (0.0 + right, -0.5 * right_y, -0.2), (0.0 + right, -0.3 * right_y, -0.2), RED.as_bytes());
			viewport.fill_triangle((0.0, 0.0, -0.1), (0.0 + right, -0.3 * right_y, -0.2), (0.0 + right, -0.1 * right_y, -0.2), YELLOW.as_bytes());
			viewport.fill_triangle((0.0, 0.0, -0.1), (0.0 + right, -0.1 * right_y, -0.2), (0.0 + right, 0.1 * right_y, -0.2), GREEN.as_bytes());
			viewport.fill_triangle((0.0, 0.0, -0.1), (0.0 + right, 0.1 * right_y, -0.2), (0.0 + right, 0.3 * right_y, -0.2), CYAN.as_bytes());
			viewport.fill_triangle((0.0, 0.0, -0.1), (0.0 + right, 0.3 * right_y, -0.2), (0.0 + right, 0.5 * right_y, -0.2), BLUE.as_bytes());
			
			// Invoke render
			viewport.render().expect("render failed");
			// Clear buffer to let it prepared for the next frame
	      	viewport.reset_buffer();
	    }
	    _ => (),
	  }
	});
}
