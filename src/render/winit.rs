use crate::pixel::Pixel;

use super::{Render, Resize};

pub struct WinitRenderer {

}

impl Render for WinitRenderer {
    fn render(&mut self, buffer: &Vec<Pixel>) -> Result<(), crate::error::ViewportError> {
        todo!()
    }

    fn clear(&mut self) -> Result<(), crate::error::ViewportError> {
        todo!()
    }
}

impl Resize<u32> for WinitRenderer {
    fn resize(&mut self, width: u32, height: u32) {
        todo!()
    }
}