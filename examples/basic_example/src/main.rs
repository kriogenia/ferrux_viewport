use ferrux_viewport::viewport::Viewport;
use ferrux_viewport::render::WinitRenderer;
use rgb::{ComponentBytes, RGBA8};
use winit::{event_loop::EventLoop, dpi::LogicalSize, window::WindowBuilder};

fn main() {
	let event_loop = EventLoop::new();

    let window = {
        let size: LogicalSize<i32> = LogicalSize::new(960, 480);
        WindowBuilder::new()
          .with_title("FerruX Canvas")
          .with_inner_size(size)
          .with_min_inner_size(size)
          .build(&event_loop)
          .unwrap()
    };

	let renderer = WinitRenderer::new(&window).unwrap();
	let mut viewport = Viewport::new(640 as u32, 480 as u32, renderer);

	assert_eq!(640, viewport.width());
	assert_eq!(480, viewport.height());
	
	let color = [RGBA8::new(255, 0, 0, 255)];
	viewport.draw_point((0.5, 0.5, 1.0), color.as_bytes());
	
	viewport.render().unwrap();
}
