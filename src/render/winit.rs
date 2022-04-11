use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::{error::ViewportError, pixel::Pixel};

use super::{Render, Resize};

/// Renderer able to work with Winit's [Window]
pub struct WinitRenderer {
    pixels: Pixels,
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
    fn render(&mut self, buffer: &[Pixel]) -> Result<(), ViewportError> {
        for (pixel, color) in self
            .pixels
            .get_frame()
            .chunks_exact_mut(4)
            .zip(buffer.iter())
        {
            pixel.copy_from_slice(&color.color);
        }

        self.pixels.render().map_err(|e| {
            error!("pixels.render() failed: {:?}", e);
            ViewportError::Rendering
        })
    }

    fn clear(&mut self) -> Result<(), crate::error::ViewportError> {
        for pixel in self.pixels.get_frame().chunks_exact_mut(4) {
			pixel.copy_from_slice(&[0, 0, 0, 0]);
		}

		self.pixels.render().map_err(|e| {
			error!("pixels.render() failed: {:?}", e);
			ViewportError::Rendering
		})
    }
}

impl Resize<u32> for WinitRenderer {
    fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height);
    }
}
