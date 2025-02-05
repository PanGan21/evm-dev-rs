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
                println!("STACK: {:#?}", self.stack());
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
            OpCode::Sdiv => {
                sdiv(&mut self.stack)?;
                Ok(())
            }
            OpCode::Mod => {
                modop(&mut self.stack)?;
                Ok(())
            }
            OpCode::Smod => {
                smod(&mut self.stack)?;
                Ok(())
            }
            OpCode::AddMod => {
                addmod(&mut self.stack)?;
                Ok(())
            }
            OpCode::MulMod => {
                mulmod(&mut self.stack)?;
                Ok(())
            }
            OpCode::Exp => {
                exp(&mut self.stack)?;
                Ok(())
            }
            OpCode::Signextend => {
                sign_extend(&mut self.stack)?;
                Ok(())
            }
            OpCode::Lt => {
                lt(&mut self.stack)?;
                Ok(())
            }
            OpCode::Gt => {
                gt(&mut self.stack)?;
                Ok(())
            }
            OpCode::Slt => {
                slt(&mut self.stack)?;
                Ok(())
            }
            OpCode::Sgt => {
                sgt(&mut self.stack)?;
                Ok(())
            }
            OpCode::Eq => {
                eq(&mut self.stack)?;
                Ok(())
            }
            OpCode::Iszero => {
                iszero(&mut self.stack)?;
                Ok(())
            }
        }
    }

    /// Returns the stack at the end of execution. Note that the stack here is reversed.
    pub fn stack(&self) -> Vec<U256> {
        self.stack.iter().rev().cloned().collect()
    }
}

#[derive(Debug)]
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
    let _ = add(stack)?;
    modop(stack)
}

pub fn mulmod(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;
    let third = pop(stack)?;

    let mul = first.full_mul(second);
    match mul.checked_rem(third.into()) {
        Some(result) => {
            let result = result.try_into().unwrap_or(0.into());
            stack.push(result);
            Ok(result)
        }
        None => {
            let result = 0.into();
            stack.push(result);
            Ok(result)
        }
    }
}

pub fn exp(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let (result, _) = first.overflowing_pow(second);
    stack.push(result);
    Ok(result)
}

pub fn sign_extend(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let sign = second.byte(first.as_usize());

    let mut data = second.to_little_endian();

    for i in 0..32 {
        if i as usize > first.as_usize() {
            if sign > 0x7f {
                data[i] = 0xFF;
            } else {
                data[i] = 0x00;
            }
        }
    }

    let result = U256::from_little_endian(&data);

    stack.push(result);
    Ok(result)
}

pub fn sdiv(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let mut first = pop(stack)?;
    let mut second = pop(stack)?;

    let is_first_negative = first.bit(255);
    let is_second_negative = second.bit(255);

    if is_first_negative {
        (first, _) = first.overflowing_neg();
    }

    if is_second_negative {
        (second, _) = second.overflowing_neg();
    }

    let mut result = first.checked_div(second).unwrap_or(U256::zero());

    if is_first_negative != is_second_negative {
        (result, _) = result.overflowing_neg();
    }

    stack.push(result);
    Ok(result)
}

pub fn smod(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let mut first = pop(stack)?;
    let mut second = pop(stack)?;

    let is_first_negative = first.bit(255);
    let is_second_negative = second.bit(255);

    if is_first_negative {
        (first, _) = first.overflowing_neg();
    }

    if is_second_negative {
        (second, _) = second.overflowing_neg();
    }

    let mut result = first.checked_rem(second).unwrap_or(U256::zero());

    if is_first_negative && is_second_negative {
        (result, _) = result.overflowing_neg();
    }

    stack.push(result);
    Ok(result)
}

pub fn lt(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let result = first < second;
    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    stack.push(result);
    Ok(result)
}

pub fn gt(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let result = first > second;
    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    stack.push(result);
    Ok(result)
}

pub fn slt(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let is_first_negative = first.bit(255);
    let is_second_negative = second.bit(255);

    let result = match (is_first_negative, is_second_negative) {
        (true, true) => !(first.overflowing_neg() <= second.overflowing_neg()),
        (true, false) => true,
        (false, true) => false,
        (false, false) => first < second,
    };

    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    stack.push(result);
    Ok(result)
}

pub fn sgt(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let is_first_negative = first.bit(255);
    let is_second_negative = second.bit(255);

    let result = match (is_first_negative, is_second_negative) {
        (true, true) => !(first.overflowing_neg() >= second.overflowing_neg()),
        (true, false) => false,
        (false, true) => true,
        (false, false) => first > second,
    };

    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    stack.push(result);
    Ok(result)
}

pub fn eq(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let first = pop(stack)?;
    let second = pop(stack)?;

    let result = first == second;
    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    stack.push(result);
    Ok(result)
}

pub fn iszero(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    let item = pop(stack)?;

    let result = if item.is_zero() {
        U256::one()
    } else {
        U256::zero()
    };

    stack.push(result);
    Ok(result)
}
