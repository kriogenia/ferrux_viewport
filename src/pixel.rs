// TODO change to enum to have blank? or use default?
#[derive(Clone, Debug, PartialEq)]
pub struct Pixel {
    pub color: [u8; 4],
    pub depth: usize,
}

impl Pixel {
    pub fn new(color: &[u8], depth: usize) -> Self {
        let color = {
            let mut copy = [0; 4];
            copy.copy_from_slice(color);
            copy
        };
        Pixel { color, depth }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            color: [0, 0, 0, 0],
            depth: usize::MIN,
        }
    }
}
