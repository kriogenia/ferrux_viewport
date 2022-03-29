use rgb::{ComponentBytes, RGBA8};

fn main() {
	let mut viewport = ferrux_viewport::viewport::Viewport::new(640 as u32, 480 as u32);
	assert_eq!(640, viewport.width());
	assert_eq!(480, viewport.height());
	let color = [RGBA8::new(255, 0, 0, 255)];
	viewport.draw_point((0.2, 0.3, 1.0), color.as_bytes());
}
