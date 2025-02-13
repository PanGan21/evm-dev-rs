macro_rules! opcodes {
    ($($name:ident($number:expr),)*) => {
        #[derive(Debug)]
        pub enum OpCode {
            $($name = $number,)*
        }

        impl TryFrom<u8> for OpCode {
            type Error = String;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(OpCode::$name),)*
                    _ => Err("Unknown opcode".to_string()),
                }
            }
        }
    };
}

opcodes! {
    Stop(0),
    Add(1),
    Mul(2),
    Sub(3),
    Div(4),
    Sdiv(5),
    Mod(6),
    Smod(7),
    AddMod(8),
    MulMod(9),
    Exp(10),
    Signextend(11),
    Lt(16),
    Gt(17),
    Slt(18),
    Sgt(19),
    Eq(20),
    Iszero(21),
    And(22),
    Or(23),
    Xor(24),
    Not(25),
    Byte(26),
    Shl(27),
    Shr(28),
    Sar(29),
    Pop(80),
    Push0(95),
    Push1(96),
    Push2(97),
    Push3(98),
    Push4(99),
    Push5(100),
    Push6(101),
    Push7(102),
    Push8(103),
    Push9(104),
    Push10(105),
    Push11(106),
    Push12(107),
    Push13(108),
    Push14(109),
    Push15(110),
    Push16(111),
    Push17(112),
    Push18(113),
    Push19(114),
    Push20(115),
    Push21(116),
    Push22(117),
    Push23(118),
    Push24(119),
    Push25(120),
    Push26(121),
    Push27(122),
    Push28(123),
    Push29(124),
    Push30(125),
    Push31(126),
    Push32(127),
    Dup1(128),
    Dup2(129),
    Dup3(130),
    Dup4(131),
    Dup5(132),
    Dup6(133),
    Dup7(134),
    Dup8(135),
    Dup9(136),
    Dup10(137),
    Dup11(138),
}

impl OpCode {
    pub fn new(opcode: u8) -> Option<Self> {
        if let Ok(opcode) = opcode.try_into() {
            Some(opcode)
        } else {
            None
        }
    }

    /// Helper function to determine the push data size for each `Push` opcode
    pub fn push_data_size(&self) -> usize {
        match self {
            OpCode::Push0 => 0,
            OpCode::Push1 => 1,
            OpCode::Push2 => 2,
            OpCode::Push3 => 3,
            OpCode::Push4 => 4,
            OpCode::Push5 => 5,
            OpCode::Push6 => 6,
            OpCode::Push7 => 7,
            OpCode::Push8 => 8,
            OpCode::Push9 => 9,
            OpCode::Push10 => 10,
            OpCode::Push11 => 11,
            OpCode::Push12 => 12,
            OpCode::Push13 => 13,
            OpCode::Push14 => 14,
            OpCode::Push15 => 15,
            OpCode::Push16 => 16,
            OpCode::Push17 => 17,
            OpCode::Push18 => 18,
            OpCode::Push19 => 19,
            OpCode::Push20 => 20,
            OpCode::Push21 => 21,
            OpCode::Push22 => 22,
            OpCode::Push23 => 23,
            OpCode::Push24 => 24,
            OpCode::Push25 => 25,
            OpCode::Push26 => 26,
            OpCode::Push27 => 27,
            OpCode::Push28 => 28,
            OpCode::Push29 => 29,
            OpCode::Push30 => 30,
            OpCode::Push31 => 31,
            OpCode::Push32 => 32,
            _ => 0,
        }
    }

    /// Helper function to determine the data to be duplicated for each `Dup` and `Swap`` opcode
    pub fn data_index(&self) -> usize {
        match self {
            OpCode::Dup1 => 1,
            OpCode::Dup2 => 2,
            OpCode::Dup3 => 3,
            OpCode::Dup4 => 4,
            OpCode::Dup5 => 5,
            OpCode::Dup6 => 6,
            OpCode::Dup7 => 7,
            OpCode::Dup8 => 8,
            OpCode::Dup9 => 9,
            OpCode::Dup10 => 10,
            OpCode::Dup11 => 11,
            _ => 0,
        }
    }
}
