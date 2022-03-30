//! Package containing the viewport logic

use log::info;

use crate::pixel::Pixel;
use crate::{Color, PixelSize, Position};
use crate::error::ViewportError;
use crate::util::{ to_pixel, buffer_index };

/// Entity in charge of offering the functions to draw on the screen and handle to logic of the operation.
/// It works using three-dimensional normalized vectors of type (x: f32, y: f32, z: f32).
/// The point to draw in the screen will be the one relative to given position in the `x` and `y` axes.
/// So, any point outside the (-1.0, 1.0) range will not be drawn.
/// The `z` value works as a layer function, it will draw only the point with the highest `z` on the same translated pixel.
/// 
/// **The viewport doesn't perform projection**, that should be handled by the user before calling the functions.
/// Viewport just draws the pixels of the highest depth relative to the given coordinates.
/// 
/// The axes directions are:
/// * `x`: west -> east
/// * `y`: north -> south
/// * `z`: far -> near
pub struct Viewport<'a, S: PixelSize> {
	width: S,
	height: S,
	buffer: Vec<Pixel<'a>>
}

impl<'a, S: PixelSize> Viewport<'a, S> {

	/// Builds a new Viewport to use.
	/// 
	/// # Arguments
	/// * `width`. Width in pixels of the screen, must be an unsigned value.
	/// * `height`. Height in pixels of the screen, must be an unsigned value.
	/// 
	/// # Example
	/// ```no_run
	/// let viewport = ferrux_viewport::viewport::Viewport::new(640 as u32, 480 as u32);
	/// ```
	/// 
	pub fn new(width: S, height: S) -> Self {
		assert!(width > S::zero());
		assert!(height > S::zero());
		// TODO store window
		let buffer_size = usize::cast(width * height);
		info!("Buffer size = {buffer_size:?}");
		Viewport { 
			width, 
			height, 
			buffer: vec![Pixel::default(); buffer_size]
		}
	}

	/// Returns the width of the current window
	pub fn width(&self) -> S {
		self.width
	}

	/// Returns the height of the current window
	pub fn height(&self) -> S {
		self.height
	}

	/// Changes the size of the rendered window
	/// 
	/// # Arguments
	/// * `width`. New width of the window.
	/// * `height`. New height of the window.
	/// 
	pub fn resize(&mut self, width: S, height: S) {
		self.width = width;
		self.height = height;
		// TODO resize buffer -> new and copy
	}

	/// TODO	
	pub fn render(&mut self) -> Result<(), ViewportError> {
		// TODO move buffer to pixels buffer
		Ok(())
	}

	/// TODO
	pub fn clear_frame(&mut self) -> Result<(), ViewportError> {
		
		Ok(())
	}

	/// Commands the drawing of a point in the window. It will be rendered in the next call to [`Viewport::render`].
	/// If thow points fall on the same pixel, the point with the lowest `z` will be ignored.
	/// 
	/// # Arguments
	/// * `position`, coordinates of the point in `(f32, f32, f32)`.
	/// * `color`, color of the point to draw. It should be provided as raw RGB values, alpha is included,
	/// so the expectation is a &[u8; 4] color like `&[255, 0, 0, 255]` for red with 100% opacity.
	/// 
	/// # Example
	/// ```
	/// # use ferrux_viewport::viewport::Viewport;
	/// # let mut viewport = Viewport::new(640 as u32, 480 as u32);
	/// viewport.draw_point((0.0, 0.0, 0.0), &[255, 255, 255, 255]); // white point in the center of the screen
	/// viewport.render(); // renders the point in the window
	/// ```
	/// 
	/// # Panic
	/// The position coordinates must be restricted to the range [-1.0, 1.0), otherwise a panic will be thrown.
	/// 
	pub fn draw_point(&mut self, position: Position, color: &'a [u8]) {
		assert_eq!(4, color.len());
		assert!((-1.0..1.0).contains(&position.0));
		assert!((-1.0..1.0).contains(&position.1));
		assert!((-1.0..1.0).contains(&position.2));

		let (i, z) = {
			let usize_width = usize::cast(self.width);
			let (x, y, z) = to_pixel(position, usize_width, usize::cast(self.height));
			(buffer_index(x, y, usize_width), z)
		};

		// TODO check depth with the current one
		self.buffer[i] = Pixel {
			color,
			depth: z		
		};
	}
	
	/// TODO
	pub fn draw_line(&mut self, start: Position, end: Position, color: Color) {}
	
	/// TODO
	pub fn draw_triangle(&mut self, point_a: Position, point_b: Position, point_c: Position, color: Color) {}
	
	///TODO
	pub fn fill_triangle(&mut self, point_a: Position, point_b: Position, point_c: Position, color: Color) {}
	
	/// TODO
	pub fn reset(&mut self) {}

}

#[cfg(test)]
mod test {
	use crate::pixel::Pixel;
	use super::Viewport;

	#[test]
	fn draw_point() {
		let mut viewport = Viewport::new(640 as u32, 480 as u32);
		let color = &[255, 255, 255, 255];

		viewport.draw_point((-1.0, -1.0, -1.0), color);
		viewport.draw_point((0.0, 0.0, 0.0), color);
		// TODO add overriden point
		viewport.draw_point((-0.25, 0.25, 0.25), color);

		assert_eq!(viewport.buffer[0], Pixel { color, depth: 0 }); 
		assert_eq!(viewport.buffer[153920], Pixel { color, depth: 5000 }); 
		assert_eq!(viewport.buffer[192240], Pixel { color, depth: 6250 }); 
	}

	#[test]
	#[should_panic]
	fn invalid_draw_point() {
		let mut viewport = Viewport::new(640 as u32, 480 as u32);
		viewport.draw_point((-2.0, 0.0, 0.0), &[0, 0, 0, 0]);
	}

}