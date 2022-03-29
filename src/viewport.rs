use crate::{Color, PixelSize, Position};
use crate::error::ViewportError;

pub struct Viewport<S: PixelSize> {
	width: S,
	height: S
}

impl<S: PixelSize> Viewport<S> {

	/// Builds a new Viewport to use.
	/// 
	/// # Arguments
	/// * `width`. Width in pixels of the screen, must be an unsigned value.
	/// * `height`. Height in pixels of the screen, must be an unsigned value.
	/// 
	/// # Example
	/// ```no_run
	/// let viewport = Viewport::new(640 as u32, 480 as u32);
	/// ```
	/// 
	pub fn new(width: S, height: S) -> Self {
		// TODO build buffer and store window
		Viewport { width, height }
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
		// TODO resize buffer
	}

	/// TODO	
	pub fn render(&mut self) -> Result<(), ViewportError> {
		Ok(())
	}

	//TODO
	pub fn clear_frame(&mut self) -> Result<(), ViewportError> {
		Ok(())
	}

	///TODO
	pub fn draw_point(&mut self, position: Position, color: Color) {
		assert_eq!(4, color.len());
		println!("Draw in pixel {position:?} with color {color:?}");
	}
	
	/// TODO
	pub fn draw_line(&mut self, start: Position, end: Position, color: Color) {}
	
	///TODO
	pub fn draw_triangle(&mut self, point_a: Position, point_b: Position, point_c: Position, color: Color) {}
	
	///TODO
	pub fn fill_triangle(&mut self, point_a: Position, point_b: Position, point_c: Position, color: Color) {}
	
	///TODO
	pub fn reset(&mut self) {}

}