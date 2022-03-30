use crate::pixel::Pixel;

use super::{Render, Resize};

pub struct MockRenderer {

}

impl Render for MockRenderer {
    fn render(&mut self, buffer: &Vec<Pixel>) -> Result<(), crate::error::ViewportError> {
        todo!()
    }

    fn clear(&mut self) -> Result<(), crate::error::ViewportError> {
        todo!()
    }
}

impl Resize<u32> for MockRenderer {
    fn resize(&mut self, width: u32, height: u32) {
        todo!()
    }
}