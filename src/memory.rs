use primitive_types::U256;

use crate::errors::ExecutionError;

pub struct Memory {
    store: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { store: Vec::new() }
    }

    pub fn save_word(&mut self, offset: usize, word: U256) -> Result<U256, ExecutionError> {
        let value_bytes = word.to_big_endian();

        // memory must have at least offset + 32 free bytes left.
        self.resize(offset, 32)?;

        for i in 0..32 {
            self.store[offset + i] = value_bytes[i];
        }
        Ok(word)
    }

    pub fn get_word(&mut self, offset: usize) -> Result<U256, ExecutionError> {
        let mut value = vec![];

        // memory must have at least offset + 32 free bytes left.
        self.resize(offset, 32)?;

        for i in 0..32 {
            value.push(*self.store.get(offset + i).unwrap_or(&0));
        }
        let value = U256::from_big_endian(value.as_slice());

        Ok(value)
    }

    fn resize(&mut self, offset: usize, size: usize) -> Result<(), ExecutionError> {
        if self.store.len() < offset + size {
            let resize_value = (offset + size - 1) / 32 + 1;
            if let Some(resize_value) = resize_value.checked_mul(32) {
                self.store.resize(resize_value, 0);
                return Ok(());
            } else {
                return Err(ExecutionError::IntegerOverflow);
            }
        }
        Ok(())
    }
}
