fn main() {
	let viewport = ferrux_viewport::viewport::Viewport::new(640 as u32, 480 as u32);
	assert_eq!(640, viewport.width());
	assert_eq!(480, viewport.height());
}
