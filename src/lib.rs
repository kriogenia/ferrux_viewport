#[allow(clippy::pedantic)]
extern crate winit;

/// TODO

pub mod viewport;
mod error;

use num_traits::{NumAssignOps, NumOps, Unsigned};

type Position = (f32, f32, f32);

pub trait PixelSize: Unsigned + NumAssignOps + NumOps + Copy + Ord  {}
impl<T: Unsigned + NumAssignOps + NumOps + Copy + Ord> PixelSize for T {}

type Color<'a> = &'a [u8];