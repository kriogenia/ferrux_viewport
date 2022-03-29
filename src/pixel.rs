// TODO change to enum to have blank? or use default?
#[derive(Clone, Debug, PartialEq)]
pub struct Pixel<'a> {
	pub color: &'a [u8],
	pub depth: f32
}

impl<'a> Default for Pixel<'a> {
    fn default() -> Self {
        Self { color: &[0, 0, 0, 0], depth: f32::MIN }
    }
}