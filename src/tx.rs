/// Tx data.
#[derive(Default)]
pub struct TxData {
    pub to: Vec<u8>,
    pub from: Vec<u8>,
    pub origin: Vec<u8>,
}

impl TxData {
    pub fn new(tx_data: Vec<Vec<u8>>) -> TxData {
        if !tx_data.is_empty() {
            return Self {
                to: tx_data[0].clone(),
                from: tx_data[1].clone(),
                origin: tx_data[2].clone(),
            };
        }

        Self::default()
    }
}
