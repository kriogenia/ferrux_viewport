use crate::{error::ViewportError, PixelSize, pixel::Pixel};

mod winit;

#[cfg(test)]
pub(crate) mod mock;

pub use self::winit::WinitRenderer;

pub trait Render {
	fn render(&mut self, buffer: &Vec<Pixel>) -> Result<(), ViewportError>;
	fn clear(&mut self) -> Result<(), ViewportError>;
}

pub trait Resize<S: PixelSize> {
	fn resize(&mut self, width: S, height: S);
}