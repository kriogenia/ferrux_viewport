#[allow(clippy::pedantic)]

pub mod viewport;
mod error;

use num_traits::{Float, NumAssignOps, NumOps, Signed, Unsigned};

pub trait Coordinate: Float + Signed + NumOps + Copy + Ord {}
impl<T: Float + Signed + NumOps + Copy + Ord> Coordinate for T {}

pub struct Position<T: Coordinate> (T, T, T);

pub trait PixelSize: Unsigned + NumAssignOps + NumOps + Copy + Ord  {}
impl<T: Unsigned + NumAssignOps + NumOps + Copy + Ord> PixelSize for T {}