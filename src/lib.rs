#[allow(clippy::pedantic)]
extern crate winit;

/// TODO

pub mod viewport;
mod error;
mod pixel;

use num_traits::{NumAssignOps, NumOps, Unsigned, NumCast};

type Position = (f32, f32, f32);

pub trait PixelSize: Unsigned + NumAssignOps + NumOps + NumCast + Copy + Ord  {
	#[inline]
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}
impl<T: Unsigned + NumAssignOps + NumOps + NumCast + Copy + Ord> PixelSize for T {}

type Color<'a> = &'a [u8];