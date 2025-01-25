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
            OpCode::Push1 => push(1, pc, &mut self.stack, self.code.as_ref()),
        }
    }
}

pub enum ExecutionResult {
    Success,
    Halt,
    Revert,
}

fn push(x: usize, pc: &mut usize, stack: &mut Vec<U256>, code: &[u8]) -> ExecutionResult {
    let start = *pc + 1;
    let end = start + x;

    if end <= code.len() {
        let data = &code[start..end];
        let value = U256::from_big_endian(data);

        stack.push(value);
        *pc += x + 1;

        ExecutionResult::Success
    } else {
        ExecutionResult::Revert
    }
}
