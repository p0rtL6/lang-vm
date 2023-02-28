#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: [u8; 3]
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: [u8; 3]) -> Instruction {
        Instruction { opcode, operands }
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

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT, [0; 3]);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
