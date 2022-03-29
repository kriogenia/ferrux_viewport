//! Ferrux Viewport is an abstraction layer over the [Pixels](https://crates.io/crates/pixels) crate.
//! It manages the pixel buffer exposing simple operations to draw pixels, lines and figures of one
//! color in the screen. In its current state it only works with [Winit](https://crates.io/crates/winit).
//! 
//! It is a new iteration and twist over my previous library [Ferrux Canvas](https://crates.io/crates/ferrux_canvas).
//! This one works with coordinates on a [-1.0, 1.0] 3D space which makes the drawing easier as it can work with
//! normalized vectors and allows the user of _layers_ based on the depth without previous check from the user.
//!
//! # Building a new viewport
//! The [`Viewport`] requires a [`Window`] of [`winit`], which will need itself an [`EventLoop`] reference.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let event_loop = winit::event_loop::EventLoop::new();
//! let window = winit::window::Window::new(&event_loop)?;
//! //let viewport = ferrux_viewport::viewport::Viewport::new(&window)?;
//! # Ok(())}
//! ```
//!
//! # Running the viewport
//! The main flow to use the viewport is:
//!
//! * Use the drawing functions like [`draw_line`] and [`draw_triangle`].
//! * Call the [`render`] method to print it on screen.
//! * Use [`reset_buffer`] to clear the current buffer and draw a new frame.
//!
//! The following example takes the [`Viewport`] we built and draws a morphing triangle.
//! ```no_run
//! //use ferrux_viewport::viewport::Viewport;
//! //use winit::event::Event;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! //let event_loop = winit::event_loop::EventLoop::new();
//! //let window = winit::window::Window::new(&event_loop)?;
//! //let mut viewport = ferrux_viewport::viewport::Viewport::new(&window)?;
//! //let mut x: i32 = 1;
//! //let mut incrementing = true;
//!//
//! //event_loop.run(move |event, _, control_flow| {
//! //  match event {
//! //    Event::MainEventsCleared => {
//! //      window.request_redraw();
//! //    }
//! //    Event::RedrawRequested(_) => {
//! //      if !(1..100).contains(&x) {
//! //        incrementing = !incrementing;
//! //      }
//! //      x += if incrementing { 1 } else { -1 };
//! //      canvas.draw_triangle((100, (100 - x) as u32), (100 - x as u32, 100),
//! //                           (200 - x as u32, 200 - x as u32), palette::WHITE);
//! //      canvas.render().unwrap();
//! //      canvas.reset_frame();
//! //    }
//! //    _ => (),
//! //  }
//! //});
//! # Ok(()) }
//! ```
//!
//! [`draw_line`]: viewport::Viewport::draw_line
//! [`draw_triangle`]: viewport::Viewport::draw_triangle
//! [`EventLoop`]: winit::event_loop::EventLoop
//! [`render`]: viewport::Viewport::render
//! [`reset_buffer`]: viewport::Viewport::reset_buffer
//! [`Viewport`]: viewport::Viewport
//! [`Window`]: winit::window::Window
//! [`winit`]: winit
//!
use num_traits::{NumAssignOps, NumOps, Unsigned, NumCast};

#[allow(clippy::pedantic)]
extern crate winit;

pub mod viewport;
mod error;
mod pixel;
mod util;

type Position = (f32, f32, f32);

/// Trait to englobe unsigned integers to use as PixelSize in the Viewport construcion
pub trait PixelSize: Unsigned + NumAssignOps + NumOps + NumCast + Copy + Ord  {
	#[inline]
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}
impl<T: Unsigned + NumAssignOps + NumOps + NumCast + Copy + Ord> PixelSize for T {}

type Color<'a> = &'a [u8];