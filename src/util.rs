use crate::{Position, Voxel};

const DEPTH: f32 = 10_000.0;

/// Converts the normalized position into the pixel equivalent in the given screen
#[inline]
pub fn to_pixel((x, y, z): Position, width: usize, height: usize) -> Voxel {
	let w = (x + 1.0) * 0.5 * (width as f32);
	let h = (y + 1.0) * 0.5 * (height as f32);
	let d = (z + 1.0) * 0.5 * DEPTH;
	(w as usize, h as usize, d as usize)
}

/// Gets the relative pixel in the screen to the given coordinates
#[inline]
pub fn buffer_index(w: usize, h: usize, width: usize) -> usize {
	h * width + w
}

macro_rules! converts_to {
	($from:tt -> $to:tt) => {
		assert_eq!($to, to_pixel($from, 640, 480));	
	};
}

macro_rules! is_indexed_in {
	($w:literal, $h:literal with $width:literal has index $i:tt) => {
		assert_eq!($i, buffer_index($w, $h, $width));
	};
}

#[test]
fn to_pixel_test() {
	converts_to!((-1.0, -1.0, -1.0) -> (0, 0, 0));							// Mininum
	converts_to!((0.0, 0.0, 0.0)    -> (320, 240, 5000));					// Middle
	converts_to!((1.0, 1.0, 1.0)    -> (640, 480, 10000));					// Maximum
	converts_to!((-0.25, 0.2, 0.6)  -> (240, 288, 8000));							// Random
}

#[test]
fn buffer_index_test() {
	is_indexed_in!(0, 0 with 640 has index 0);
	is_indexed_in!(320, 240 with 640 has index 153920);
	is_indexed_in!(640, 479 with 640 has index (480 * 640));
	is_indexed_in!(124, 213 with 640 has index 136444);
}