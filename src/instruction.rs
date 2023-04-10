#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    ALOC,
    INC,
    DEC
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub registers: [usize; 3],
    pub integer_operand: i32,
}

impl Instruction {
    pub fn new(opcode: Opcode, registers: [usize; 3], integer_operand: i32) -> Instruction {
        Instruction { opcode, registers, integer_operand }
    }
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::GT,
            12 => return Opcode::LT,
            13 => return Opcode::GTQ,
            14 => return Opcode::LTQ,
            15 => return Opcode::JEQ,
            16 => return Opcode::ALOC,
            17 => return Opcode::INC,
            18 => return Opcode::DEC,
            _ => return Opcode::IGL
        }
    }
}

impl From<String> for Opcode {
    fn from(string: String) -> Self {
        match string.as_str() {
            "HLT" => return Opcode::HLT,
            "LOAD" => return Opcode::LOAD,
            "ADD" => return Opcode::ADD,
            "SUB" => return Opcode::SUB,
            "MUL" => return Opcode::MUL,
            "DIV" => return Opcode::DIV,
            "JMP" => return Opcode::JMP,
            "JMPF" => return Opcode::JMPF,
            "JMPB" => return Opcode::JMPB,
            "EQ" => return Opcode::EQ,
            "NEQ" => return Opcode::NEQ,
            "GT" => return Opcode::GT,
            "LT" => return Opcode::LT,
            "GTQ" => return Opcode::GTQ,
            "LTQ" => return Opcode::LTQ,
            "JEQ" => return Opcode::JEQ,
            "ALOC" => return Opcode::ALOC,
            "INC" => return Opcode::INC,
            "DEC" => return Opcode::DEC,
            _ => return Opcode::IGL
        }
    }
}

impl From<&str> for Opcode {
    fn from(str: &str) -> Self {
        match str {
            "HLT" => return Opcode::HLT,
            "LOAD" => return Opcode::LOAD,
            "ADD" => return Opcode::ADD,
            "SUB" => return Opcode::SUB,
            "MUL" => return Opcode::MUL,
            "DIV" => return Opcode::DIV,
            "JMP" => return Opcode::JMP,
            "JMPF" => return Opcode::JMPF,
            "JMPB" => return Opcode::JMPB,
            "EQ" => return Opcode::EQ,
            "NEQ" => return Opcode::NEQ,
            "GT" => return Opcode::GT,
            "LT" => return Opcode::LT,
            "GTQ" => return Opcode::GTQ,
            "LTQ" => return Opcode::LTQ,
            "JEQ" => return Opcode::JEQ,
            "ALOC" => return Opcode::ALOC,
            "INC" => return Opcode::INC,
            "DEC" => return Opcode::DEC,
            _ => return Opcode::IGL
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::HLT => return 0,
            Opcode::LOAD => return 1,
            Opcode::ADD => return 2,
            Opcode::SUB => return 3,
            Opcode::MUL => return 4,
            Opcode::DIV => return 5,
            Opcode::JMP => return 6,
            Opcode::JMPF => return 7,
            Opcode::JMPB => return 8,
            Opcode::EQ => return 9,
            Opcode::NEQ => return 10,
            Opcode::GT => return 11,
            Opcode::LT => return 12,
            Opcode::GTQ => return 13,
            Opcode::LTQ => return 14,
            Opcode::JEQ => return 15,
            Opcode::ALOC => return 16,
            Opcode::INC => return 17,
            Opcode::DEC => return 18,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    // #[test]
    // fn test_create_instruction() {
    //     let instruction = Instruction::new(Opcode::HLT, [0; 3]);
    //     assert_eq!(instruction.opcode, Opcode::HLT);
    // }
}
