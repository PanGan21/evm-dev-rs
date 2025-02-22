/// Block data.
#[derive(Default)]
pub struct BlockData {
    pub basefee: Vec<u8>,
    pub coinbase: Vec<u8>,
}

impl BlockData {
    pub fn new(block_data: Vec<Vec<u8>>) -> BlockData {
        if !block_data.is_empty() {
            return Self {
                basefee: block_data[0].clone(),
                coinbase: block_data[1].clone(),
            };
        }

        Self::default()
    }
}
