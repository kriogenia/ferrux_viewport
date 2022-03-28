use crate::{PixelSize, Position, Coordinate};
use crate::error::ViewportError;

type Color<'a> = &'a [u8; 4];

pub struct Viewport<S: PixelSize> {
	width: S,
	height: S,
}

impl<S: PixelSize> Viewport<S> {
	/* Extract to trait Size */
	fn width(&self) -> S {
		self.width
	}
	fn height(&self) -> S {
		self.height
	}
	fn resize(&mut self, width: S, height: S) {}

	/* Extract to trait Render */	
	fn render(&mut self) -> Result<(), ViewportError> {
		Ok(())
	}
	fn clear_frame(&mut self) -> Result<(), ViewportError> {
		Ok(())
	}

	/* Extract to trait Draw */
	fn draw_point<P: Coordinate>(&mut self, position: Position<P>, color: Color) {}
	fn draw_line<P: Coordinate>(&mut self, start: Position<P>, end: Position<P>, color: Color) {}
	fn draw_triangle<P: Coordinate>(&mut self, point_a: Position<P>, point_b: Position<P>, point_c: Position<P>, color: Color) {}
	fn fill_triangle<P: Coordinate>(&mut self, point_a: Position<P>, point_b: Position<P>, point_c: Position<P>, color: Color) {}
	fn reset(&mut self) {}

}