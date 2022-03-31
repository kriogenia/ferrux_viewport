use log::info;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::{pixel::Pixel, error::ViewportError};

use super::{Render, Resize};

pub struct WinitRenderer {
	pixels: Pixels
}

impl WinitRenderer {

	pub fn new(window: &Window) -> Result<Self, ViewportError> {
		info!("[WinitRenderer] Initializing.");

		let window_size = window.inner_size();
		let width = window_size.width;
		let height = window_size.height;
		info!("[WinitRenderer] Width: {}. Height: {}", &width, &height);

		info!("[WinitRenderer] Creating pixel buffer.");
		let pixels = {
			let surface_texture =
				SurfaceTexture::new(window_size.width, window_size.height, &window);
			Pixels::new(window_size.width, window_size.height, surface_texture)
				.map_err(|_| ViewportError::AdapterNotFound)?
		};

		Ok(Self { pixels })
	}

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