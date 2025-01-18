use primitive_types::U256;

use crate::opcode::OpCode;

pub struct Evm {
    pub code: Box<[u8]>,
    pub stack: Vec<U256>,
}

impl Evm {
    pub fn new(code: Box<[u8]>, stack: Vec<U256>) -> Self {
        Self { code, stack }
    }

    pub fn execute(&mut self) -> ExecutionResult {
        let mut pc = 0;
        while pc < self.code.len() {
            if let Some(opcode) = OpCode::new(self.code[pc]) {
                if let ExecutionResult::Halt = self.transact(&mut pc, opcode) {
                    return ExecutionResult::Halt;
                }
            } else {
                return ExecutionResult::Revert;
            }
        }
        ExecutionResult::Success
    }

    pub fn transact(&mut self, pc: &mut usize, opcode: OpCode) -> ExecutionResult {
        match opcode {
            OpCode::Stop => ExecutionResult::Halt,
            OpCode::Push0 => {
                self.stack.push(0.into());
                *pc += 1;
                ExecutionResult::Success
            }
            OpCode::Push1 => {
                let push_data = self.code[*pc + 1];
                self.stack.push(push_data.into());
                *pc += 2;
                ExecutionResult::Success
            }
        }
    }
}

pub enum ExecutionResult {
    Success,
    Halt,
    Revert,
}
