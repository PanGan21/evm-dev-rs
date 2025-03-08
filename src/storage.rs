use std::collections::HashMap;

use primitive_types::U256;

/// Storage of a contract.
#[derive(Clone)]
pub struct Storage {
    /// The mapping between the contract address and its storage.
    pub store: HashMap<U256, StorageData>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            store: HashMap::default(),
        }
    }

    pub fn set_constract_slot(&mut self, address: U256, slot: U256, value: U256) {
        let contract_storage = self
            .store
            .entry(address)
            .or_insert_with(|| StorageData::default());

        contract_storage.set_value(slot, value);
    }

    pub fn load_slot(&self, address: U256, slot: U256) -> U256 {
        if let Some(contract_address) = self.store.get(&address) {
            contract_address.get_value(slot)
        } else {
            U256::zero()
        }
    }
}

/// Storage data for a contract.
#[derive(Default, Clone)]
pub struct StorageData {
    /// Mapping between storage slot and value.
    pub data: HashMap<U256, U256>,
}

impl StorageData {
    pub fn set_value(&mut self, slot: U256, value: U256) {
        self.data.insert(slot, value);
    }

    pub fn get_value(&self, slot: U256) -> U256 {
        *self.data.get(&slot).unwrap_or(&0.into())
    }
}
