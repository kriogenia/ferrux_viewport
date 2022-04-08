//! Ferrux Viewport is an abstraction layer over the [Pixels](https://crates.io/crates/pixels) crate.
//! It manages the pixel buffer exposing simple operations to draw pixels, lines and figures of one
//! color in the screen. In its current state it only works with [Winit](https://crates.io/crates/winit).
//! 
//! It is a new iteration and twist over my previous library [Ferrux Canvas](https://crates.io/crates/ferrux_canvas).
//! This one works with coordinates on a [-1.0, 1.0] 3D space which makes the drawing easier as it can work with
//! normalized vectors and allows the use of _layers_ based on the depth without previous check from the user.
//!
//! # Building a new viewport
//! The [`Viewport`] requires a [`Window`] of [`winit`], which will need itself an [`EventLoop`] reference.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_loop = winit::event_loop::EventLoop::new();
//! let window = winit::window::Window::new(&event_loop)?;
//! let viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100)?;
//! # Ok(())}
//! ```
//!
//! # Running the viewport
//! The main flow to use the viewport is:
//!
//! * Use the drawing functions like [`draw_line`] and [`draw_triangle`].
//! * Call the [`render`] method to print it on screen.
//! * Use [`reset`] to clear the current buffer and draw a new frame.
//!
//! The following example takes the [`Viewport`] we built and draws a red morphing triangle.
//! ```no_run
//! # use winit::event::Event;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let event_loop = winit::event_loop::EventLoop::new();
//! # let window = winit::window::Window::new(&event_loop)?;
//! # let mut viewport = ferrux_viewport::viewport::ViewportFactory::winit(&window, 100)?;
//! let mut i = 0.0;
//! let step = 0.05;
//! let mut incrementing = true;
//! 
//! event_loop.run(move |event, _, control_flow| {
//!   match event {
//!     Event::MainEventsCleared => {
//!         i += if incrementing { step } else { -step };
//!         if i >= 1.0 { incrementing = false } else if i <= 0.0 { incrementing = true }
//!         window.request_redraw();
//!     }
//!     Event::RedrawRequested(_) => {
//!       viewport.fill_triangle((0.0, 0.0, -0.1), (0.0 + i, -0.5 * i/2.0, -0.2), (0.0 + i, -0.3 * i/2.0, -0.2), 
//!         &[255, 0, 0, 255]);
//!       viewport.render().expect("render failed");
//!       viewport.reset_buffer();
//!     }
//!     _ => (),
//!   }
//! });
//! # Ok(()) }
//! ```
//! 
//! ## Coloring
//! All the library drawing operations request a color in the form of a &[u8; 4] (RGBA). This allows the library to be compatible
//! with the different color crates (in the examples we use the `rgb` crate).
//! 
//! The array dimmension is not enforced at compilation time but a panic will be thrown if the provided value is not of length four.
//! But this can probably change in the future, allowing to provide just RGB or single channels.
//! 
//! Even if we request the alpha, the crate currently **DOES NOT** have transparency as you expect it.
//! You can use the alpha to play with the colors but they will always be mixed with the black background, not with whatever color could be behind.
//!
//! [`draw_line`]: viewport::Viewport::draw_line
//! [`draw_triangle`]: viewport::Viewport::draw_triangle
//! [`EventLoop`]: winit::event_loop::EventLoop
//! [`render`]: viewport::Viewport::render
//! [`reset`]: viewport::Viewport::reset_buffer
//! [`Viewport`]: viewport::Viewport
//! [`Window`]: winit::window::Window
//! [`winit`]: winit
//!

#![allow(clippy::pedantic)]

extern crate winit;

use num_traits::{NumAssignOps, NumOps, Unsigned, NumCast};

pub mod error;
pub mod render;
pub mod viewport;
mod pixel;
mod util;

type Position = (f32, f32, f32);
type Voxel<T> = (T, T, T);

/// Trait to englobe unsigned integers to use as PixelSize in the Viewport construcion
pub trait PixelSize: Unsigned + NumAssignOps + NumOps + NumCast + Copy + Ord  {
	#[inline]
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}
impl<T: Unsigned + NumAssignOps + NumOps + NumCast + Copy + Ord> PixelSize for T {}