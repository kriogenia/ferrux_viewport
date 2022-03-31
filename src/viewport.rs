//! Package containing the viewport logic

mod factory;
pub use factory::ViewportFactory;

use crate::pixel::Pixel;
use crate::render::{Render, Resize};
use crate::{PixelSize, Position, Voxel};
use crate::error::ViewportError;
use crate::util::{ to_pixel, buffer_index };
use log::info;

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
pub struct Viewport<'a, S, R> {
	width: S,
	height: S,
	buffer: Vec<Pixel<'a>>,
	renderer: R,	
}

impl<'a, S: PixelSize, R> Viewport<'a, S, R> {

	/// Builds a new Viewport to use.
	/// 
	/// # Arguments
	/// * `width`. Width in pixels of the screen, must be an unsigned value.
	/// * `height`. Height in pixels of the screen, must be an unsigned value.
	/// * `renderer`: Renderer to draw on
	/// 
	pub(crate) fn new(width: S, height: S, renderer: R) -> Self {
		assert!(width > S::zero());
		assert!(height > S::zero());
		
		let buffer_size = usize::cast(width * height);
		info!("Buffer size = {buffer_size:?}");
		Viewport { 
			width, 
			height, 
			buffer: vec![Pixel::default(); buffer_size],
			renderer
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

	/// Adds a pixel to the buffer. It also verifies the color array and throws a panic if it's not correct.
	fn push_pixel(&mut self, (x, y, z): Voxel, color: &'a [u8]) {
		assert_eq!(4, color.len());
		let i = buffer_index(x, y, usize::cast(self.width));
		if i < self.buffer.len() && z >= self.buffer[i].depth {
			self.buffer[i] = Pixel {
				color,
				depth: z
			};
		}
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
	/// ```no_run
	/// # let event_loop = winit::event_loop::EventLoop::new();
	/// # let window = winit::window::Window::new(&event_loop).unwrap();
	/// # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window).unwrap();
	/// viewport.draw_point((0.0, 0.0, 0.0), &[255, 255, 255, 255]); // white point in the center of the screen
	/// viewport.render(); // renders the point in the window
	/// ```
	/// 
	/// # Panic
	/// Passing a color with the wrong number of members will throw a panic. It's required to have length four (R, G, B, A);
	/// 
	pub fn draw_point(&mut self, position: Position, color: &'a [u8]) {
		let voxel = to_pixel(position, usize::cast(self.width), usize::cast(self.height));
		self.push_pixel(voxel, color);
	}
	
	/// TODO
	pub fn draw_line(&mut self, start: Position, end: Position, color: &[u8]) {
		// bresenham between the points
		// call push pixel for each given point
	}
	
	/// TODO
	pub fn draw_triangle(&mut self, point_a: Position, point_b: Position, point_c: Position, color: &[u8]) {
		// draw line between each pair of points
	}
	
	///TODO
	pub fn fill_triangle(&mut self, point_a: Position, point_b: Position, point_c: Position, color: &[u8]) {
		// sort vectors
		// match
			// * fill_flat_triangle topside
			// * fill_flat_triangle bottomside
			// * fill_non_flat_triangle
				// calculate intersection
				// fill top side
				// fill bottom side
	}
	
	/// TODO
	pub fn reset(&mut self) {
		// same as buffer creation
	}

}

impl<'a, S: PixelSize, R: Resize<S>> Viewport<'a, S, R> {
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
		self.renderer.resize(width, height);
	}
}

impl<'a, S: PixelSize, R: Render> Viewport<'a, S, R> {
	/// TODO
	pub fn render(&mut self) -> Result<(), ViewportError> {
		self.renderer.render(&self.buffer)
	}

	/// TODO
	pub fn clear_frame(&mut self) -> Result<(), ViewportError> {
		// TODO call render method?
		Ok(())
	}

}

#[cfg(test)]
mod test {
	use crate::{pixel::Pixel, viewport::ViewportFactory};

	#[test]
	fn draw_point() {
		let mut viewport = ViewportFactory::test(640, 480);
		let color = &[255, 255, 255, 255];

		viewport.draw_point((-1.0, -1.0, -1.0), color);
		viewport.draw_point((1.0, 1.0, 1.0), color);			// will be ignored
		viewport.draw_point((0.0, 0.0, 0.0), color);
		viewport.draw_point((0.0, 0.0, 0.5), color);			// will override the previous one
		viewport.draw_point((-0.25, 0.25, 0.25), color);
		viewport.draw_point((-0.25, 0.25, -0.25), color);		// will not override the previous

		assert_eq!(viewport.buffer[0], Pixel { color, depth: 0 }); 
		assert_eq!(viewport.buffer[153920], Pixel { color, depth: 7500 }); 
		assert_eq!(viewport.buffer[192240], Pixel { color, depth: 6250 }); 
	}

	#[test]
	#[should_panic]
	fn wrong_color() {
		ViewportFactory::test(640, 480).draw_point((0.0, 0.0, 0.0), &[0, 0, 0]);
	}

}