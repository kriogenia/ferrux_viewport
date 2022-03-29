/// Gets the relative pixel in the screen to the given coordinates
#[inline]
pub fn buffer_index(x: f32, y: f32, width: usize, height: usize) -> usize {
	let a = (x + 1.0) * 0.5 * (width as f32);
	let b = (y + 1.0) * 0.5 * (height as f32);
	b as usize * width + a as usize
}

#[test]
fn start_point() {
	assert_eq!(0, buffer_index(-1.0, -1.0, 640, 480))
}

#[test]
fn center_point() {
	assert_eq!(240 * 640 + 320, buffer_index(0.0, 0.0, 640, 480))
}

#[test]
fn last_point() {
	assert_eq!(480 * 640 - 1, buffer_index(0.999, 0.999, 640, 480))
}

#[test]
fn random_point() {
	assert_eq!(288 * 640 + 160, buffer_index(-0.5, 0.2, 640, 480))
}