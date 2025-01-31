use primitive_types::U256;

use crate::{errors::ExecutionError, opcode::OpCode};

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
                if let Ok(_) = self.transact(&mut pc, opcode) {
                    // move the pc to the next instruction
                    pc += 1;
                } else {
                    return ExecutionResult::Halt;
                }
            } else {
                return ExecutionResult::Revert;
            }
        }
        ExecutionResult::Success
    }

    pub fn transact(&mut self, pc: &mut usize, opcode: OpCode) -> Result<(), ExecutionError> {
        match opcode {
            OpCode::Stop => Err(ExecutionError::Halt),
            OpCode::Push0 => {
                self.stack.push(0.into());
                Ok(())
            }
            OpCode::Push1
            | OpCode::Push2
            | OpCode::Push3
            | OpCode::Push4
            | OpCode::Push5
            | OpCode::Push6
            | OpCode::Push7
            | OpCode::Push8
            | OpCode::Push9
            | OpCode::Push10
            | OpCode::Push11
            | OpCode::Push12
            | OpCode::Push13
            | OpCode::Push14
            | OpCode::Push15
            | OpCode::Push16
            | OpCode::Push17
            | OpCode::Push18
            | OpCode::Push19
            | OpCode::Push20
            | OpCode::Push21
            | OpCode::Push22
            | OpCode::Push23
            | OpCode::Push24
            | OpCode::Push25
            | OpCode::Push26
            | OpCode::Push27
            | OpCode::Push28
            | OpCode::Push29
            | OpCode::Push30
            | OpCode::Push31
            | OpCode::Push32 => {
                let push_data_size = opcode.push_data_size();
                push(push_data_size, pc, &mut self.stack, self.code.as_ref());
                *pc += push_data_size;
                Ok(())
            }
            OpCode::Pop => {
                pop(&mut self.stack)?;
                Ok(())
            }
            OpCode::Add => {
                add(&mut self.stack)?;
                Ok(())
            }
            OpCode::Mul => {
                mul(&mut self.stack)?;
                Ok(())
            }
            OpCode::Sub => {
                sub(&mut self.stack)?;
                Ok(())
            }
            OpCode::Div => {
                div(&mut self.stack)?;
                Ok(())
            }
            OpCode::Mod => {
                modop(&mut self.stack)?;
                Ok(())
            }
            OpCode::AddMod => {
                addmod(&mut self.stack)?;
                Ok(())
            }
        }
    }

    /// Returns the stack at the end of execution. Note that the stack here is reversed.
    pub fn stack(&self) -> Vec<U256> {
        self.stack.iter().rev().cloned().collect()
    }
}

pub enum ExecutionResult {
    Success,
    Halt,
    Revert,
}

fn push(
    push_data_size: usize,
    pc: &mut usize,
    stack: &mut Vec<U256>,
    code: &[u8],
) -> ExecutionResult {
    let start = *pc + 1;
    let remaining_code = &code[start..];
    let push_data = &remaining_code[..push_data_size];
    let push_data = U256::from_big_endian(push_data);
    stack.push(push_data);

    ExecutionResult::Success
}

fn pop(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let item = stack.pop().ok_or_else(|| ExecutionError::StackUnderflow)?;
    Ok(item)
}

fn add(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let (new_item, _) = first.overflowing_add(second);
    stack.push(new_item);

    Ok(new_item)
}

fn mul(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let (new_item, _) = first.overflowing_mul(second);
    stack.push(new_item);

    Ok(new_item)
}

fn sub(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let (new_item, _) = first.overflowing_sub(second);
    stack.push(new_item);

    Ok(new_item)
}

fn div(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let new_item = first.checked_div(second).unwrap_or(0.into());
    stack.push(new_item);

    Ok(new_item)
}

fn modop(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let new_item = first.checked_rem(second).unwrap_or(0.into());
    stack.push(new_item);

    Ok(new_item)
}

fn addmod(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    // let first = pop(stack)?;
    // let second = pop(stack)?;

    // let new_item = first.checked_rem(second).unwrap_or(0.into());
    // stack.push(new_item);
    let _ = add(stack)?;
    modop(stack)
}
