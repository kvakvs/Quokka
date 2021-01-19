pub struct Binary {
    pub bytes: Vec<u8>,
    /// Value other than 0 represents a binary with last byte incomplete.
    pub unused_bits: u8,
}

impl Binary {
    pub fn new(b: &[u8]) -> Self {
        Self {
            bytes: Vec::from(b),
            unused_bits: 0,
        }
    }
}