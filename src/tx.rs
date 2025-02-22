/// Tx data.
pub struct TxData {
    pub to: Vec<u8>,
}

impl TxData {
    pub fn new(tx_data: Vec<Vec<u8>>) -> TxData {
        if !tx_data.is_empty() {
            Self {
                to: tx_data[0].clone(),
            }
        } else {
            Self { to: vec![] }
        }
    }
}
