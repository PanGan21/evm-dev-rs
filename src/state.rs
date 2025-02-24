use std::collections::HashMap;

use primitive_types::U256;

// State
#[derive(Debug)]
pub struct State {
    pub entries: Vec<StateData>,
}

impl State {
    pub fn new(state_data: HashMap<Vec<u8>, (usize, Vec<u8>, Vec<u8>)>) -> State {
        let mut entries = vec![];
        for (address, data) in state_data {
            let state_data = StateData {
                address: U256::from_big_endian(&address),
                data: AddressData {
                    balance: U256::from_big_endian(&data.1),
                },
            };
            entries.push(state_data);
        }
        State { entries }
    }

    pub fn get_balance(&self, address: U256) -> U256 {
        self.entries
            .iter()
            .find(|s| s.address == address)
            .map(|s| s.data.balance.clone())
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct StateData {
    pub address: U256,
    pub data: AddressData,
}

#[derive(Debug)]
pub struct AddressData {
    pub balance: U256,
}
