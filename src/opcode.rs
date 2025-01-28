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
}
