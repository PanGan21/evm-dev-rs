use std::collections::HashMap;

use primitive_types::U256;

use crate::errors::ExecutionError;

// State
#[derive(Debug, Clone)]
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
                    nonce: data.0,
                    balance: U256::from_big_endian(&data.1),
                    code: data.2,
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

    pub fn get_code(&self, address: U256) -> Vec<u8> {
        self.entries
            .iter()
            .find(|s| s.address == address)
            .map(|s| s.data.code.clone())
            .unwrap_or_default()
    }

    pub fn get_nonce(&self, address: U256) -> usize {
        self.entries
            .iter()
            .find(|s| s.address == address)
            .map(|s| s.data.nonce.clone())
            .unwrap_or_default()
    }

    pub fn save_code(
        &mut self,
        address: U256,
        code: Vec<u8>,
        value_transferred: U256,
    ) -> Result<(), ExecutionError> {
        match self.entries.iter().find(|s| s.address == address) {
            Some(_state) => Err(ExecutionError::ContractAddressCollision),
            None => {
                let address_data = AddressData {
                    nonce: 0,
                    balance: value_transferred,
                    code,
                };

                let state_data = StateData {
                    address,
                    data: address_data,
                };

                self.entries.push(state_data);
                Ok(())
            }
        }
    }

    pub fn transfer_balance(&mut self, balance: U256, dest: U256) {
        if let Some(account) = self.entries.iter_mut().find(|s| s.address == dest) {
            account.data.balance += balance;
        } else {
            let new_account = StateData {
                address: dest,
                data: AddressData {
                    nonce: 0,
                    balance,
                    code: vec![],
                },
            };
            self.entries.push(new_account);
        }
    }

    pub fn delete_account(&mut self, address: U256) {
        self.entries.retain(|account| account.address != address);
    }
}

#[derive(Debug, Clone)]
pub struct StateData {
    pub address: U256,
    pub data: AddressData,
}

#[derive(Debug, Clone)]
pub struct AddressData {
    pub nonce: usize,
    pub balance: U256,
    pub code: Vec<u8>,
}
