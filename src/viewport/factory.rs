use winit::window::Window;
use crate::error::ViewportError;
use crate::{viewport::Viewport, render};

use super::WinitViewport;

/// Factory to build the viewports for the different window tools, currently it offers the following:
/// 
/// * `winit` to use with the [winit] crate
/// 
pub struct ViewportFactory;

impl ViewportFactory {

	/// Returns a [Viewport] to render the content drawn into a [winit]
	/// 
	/// # Arguments
	/// * `window`, reference to the winit Window to draw on.
	/// 
	/// # Error
	/// If no graphics adapter is found
	/// 
	/// # Example
	/// The [`Viewport`] requires a [`Window`] of [`winit`], which will need itself an [`EventLoop`] reference.
	///
	/// ```no_run
	/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let event_loop = winit::event_loop::EventLoop::new();
	/// let window = winit::window::Window::new(&event_loop)?;
	/// let viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100)?;
	/// # Ok(())}
	/// ``` 
	/// 
	/// [`EventLoop`]: winit::event_loop::EventLoop
	/// [`Window`]: winit::window::Window
	///
	pub fn winit(window: &Window, depth: u32) -> Result<WinitViewport<u32>, ViewportError> {
		let renderer = render::WinitRenderer::new(window)?;
		let size = window.inner_size();
		Ok(Viewport::new(size.width, size.height, depth, renderer))
	}

	#[cfg(test)]
	pub fn test(width: u32, height: u32, depth: u32) -> Viewport<u32, render::mock::MockRenderer> {
		Viewport::new(width, height, depth, render::mock::MockRenderer::default())
	}

}