use crate::{Position, Voxel};

/// Converts the normalized position into the pixel equivalent in the given screen
#[inline]
pub fn to_pixel((x, y, z): Position, (width, height, depth): (usize, usize, usize)) -> Voxel<usize> {
	let w = (x + 1.0) * 0.5 * (width as f32);
	let h = (y + 1.0) * 0.5 * (height as f32);
	let d = (z + 1.0) * 0.5 * (depth as f32);
	(w as usize, h as usize, d as usize)
}

/// Converts the return type of [to_pixel] to allow it to work with the Bresenham crate
#[inline]
pub fn as_signed((x, y, z): Voxel<usize>) -> Voxel<isize> {
	(x as isize, y as isize, z as isize)
}

/// Gets the relative pixel in the screen to the given coordinates
#[inline]
pub fn buffer_index(w: usize, h: usize, width: usize) -> usize {
	h * width + w
}

/// Calculates the intersection between the three points over the line connecting top and bot having the
/// same height as mid
#[inline]
pub fn calculate_intersection(top: Voxel<isize>, mid: Voxel<isize>, bot: Voxel<isize>) -> Voxel<isize> {
	let diff_y_mid = mid.1 as f32 - top.1 as f32;
	let diff_y_bot = bot.1 as f32 - top.1 as f32;
	let diff_x = bot.0 as f32 - top.0 as f32;
	let diff_z = bot.2 as f32 - top.2 as f32;
	let x = top.0 as f32 + (diff_y_mid / diff_y_bot) * diff_x;
	let z = top.2 as f32 + (diff_y_mid / diff_y_bot) * diff_z;
	(x as isize, mid.1 as isize, z as isize)
}

/// Receives three points and returns them sorted by Y value.
/// This is a method to ease the finding of the middle vector and both peaks when filling a triangle
#[inline]
pub fn sort_vectors(p1: Voxel<isize>, p2: Voxel<isize>, p3: Voxel<isize>) -> (Voxel<isize>, Voxel<isize>, Voxel<isize>) {
	let mut points = [p1, p2, p3];
	points.sort_by(|&a, &b| a.1.cmp(&b.1));
	(points[2], points[1], points[0])
}

#[cfg(test)]
macro_rules! converts_to {
	($from:tt -> $to:tt) => {
		assert_eq!($to, to_pixel($from, (640, 480, 100)));	
	};
}

#[cfg(test)]
macro_rules! is_indexed_in {
	($w:literal, $h:literal with $width:literal width has index $i:tt) => {
		assert_eq!($i, buffer_index($w, $h, $width));
	};
}

#[test]
fn to_pixel_test() {
	converts_to!((-1.0, -1.0, -1.0) -> (0, 0, 0));							// Mininum
	converts_to!((0.0, 0.0, 0.0)    -> (320, 240, 50));					// Middle
	converts_to!((1.0, 1.0, 1.0)    -> (640, 480, 100));					// Maximum
	converts_to!((-0.25, 0.2, 0.6)  -> (240, 288, 80));							// Random
}

#[test]
fn buffer_index_test() {
	is_indexed_in!(0, 0 with 640 width has index 0);
	is_indexed_in!(320, 240 with 640 width has index 153920);
	is_indexed_in!(640, 479 with 640 width has index (480 * 640));
	is_indexed_in!(124, 213 with 640 width has index 136444);
}

#[test]
fn sort_vectors_test() {
	assert_eq!(((10, 10, 10), (5, 5, 5), (0, 0, 0)), sort_vectors((10, 10, 10), (5, 5, 5), (0, 0, 0)));
	assert_eq!(((5, 10, 0), (10, 5, 0), (0, 0, 0)), sort_vectors((10, 5, 0), (5, 10, 0), (0, 0, 0)));
	assert_eq!(((0, 10, 5), (10, 5, 0), (5, 0, 10)), sort_vectors((5, 0, 10), (10, 5, 0), (0, 10, 5)));
}

#[test]
fn calculate_intersection_test() {
	assert_eq!(calculate_intersection((4, 0, 2), (0, 2, 1), (0, 4, 4)), (2, 2, 3));
	assert_eq!(calculate_intersection((4, 0, 2), (0, 2, 1), (8, 4, -2)), (6, 2, 0));
}
