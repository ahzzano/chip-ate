#[derive(Debug)]
pub struct Memory {
    values: [u8; 0xFFF],
}

impl Memory {
    pub fn new() -> Self {
        Memory { values: [0; 0xFFF] }
    }
}
