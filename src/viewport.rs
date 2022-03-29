use crate::pixel::Pixel;
use crate::{Color, PixelSize, Position};
use crate::error::ViewportError;

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
	/// let viewport = Viewport::new(640 as u32, 480 as u32);
	/// ```
	/// 
	pub fn new(width: S, height: S) -> Self {
		// TODO store window
		let buffer_size = usize::cast(width * height);
		dbg!(buffer_size);
		Viewport { 
			width, 
			height, 
			buffer: Vec::with_capacity(buffer_size)
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
		Ok(())
	}

	//TODO
	pub fn clear_frame(&mut self) -> Result<(), ViewportError> {
		Ok(())
	}

	///TODO
	pub fn draw_point(&mut self, (x, y, z): Position, color: &'a [u8]) {
		assert_eq!(4, color.len());
		//dbg!(x, y, z);
		
		// Extract function and make better as the range is [-1.0, 1.0] for both axis
		//let width = (usize::cast(self.width) as f32 * x) as usize;
		//let height = (usize::cast(self.height) as f32 * y) as usize;
		//let pos = height * usize::cast(self.width) + width;
		//dbg!(width, height, pos);

		self.buffer.push(Pixel {
			color,
			depth: z		
		});
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