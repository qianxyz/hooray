pub struct RGB([u8; 3]);

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b])
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
