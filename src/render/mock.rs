use crate::pixel::Pixel;

use super::{Render, Resize};

pub struct MockRenderer {
	pub render_calls: i32,
	pub clear_calls: i32,
	pub size: (u32, u32),
}

impl Render for MockRenderer {
    fn render(&mut self, _: &[Pixel]) -> Result<(), crate::error::ViewportError> {
        self.render_calls += 1;
		Ok(())
    }

    fn clear(&mut self) -> Result<(), crate::error::ViewportError> {
        self.clear_calls += 1;
		Ok(())
    }
}

impl Resize<u32> for MockRenderer {
    fn resize(&mut self, width: u32, height: u32) {
        self.size = (width, height);
    }
}

impl Default for MockRenderer {
    fn default() -> Self {
        Self { render_calls: Default::default(), clear_calls: Default::default(), size: Default::default() }
    }
}