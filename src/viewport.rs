use crate::{PixelSize, Position, Coordinate};
use crate::error::ViewportError;

type Color<'a> = &'a [u8; 4];

pub trait Viewport<S: PixelSize, P: Coordinate> {

	/* Extract to trait Size */
	fn width(&self) -> S;
	fn height(&self) -> S;
	fn resize(&mut self, width: S, height: S);

	/* Extract to trait Render */	
	fn render(&mut self) -> Result<(), ViewportError>;
	fn clear_frame(&mut self) -> Result<(), ViewportError>;

	/* Extract to trait Draw */
	fn draw_point(&mut self, position: Position<P>, color: Color);
	fn draw_line(&mut self, start: Position<P>, end: Position<P>, color: Color);
	fn draw_triangle(&mut self, point_a: Position<P>, point_b: Position<P>, point_c: Position<P>, color: Color);
	fn fill_triangle(&mut self, point_a: Position<P>, point_b: Position<P>, point_c: Position<P>, color: Color);
	fn reset(&mut self);

}