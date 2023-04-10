use crate::instruction::{Instruction, Opcode, self};

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pub pc: usize,
    pub program: Vec<Instruction>,
    pub heap: Vec<u8>,
    pub remainder: u32,
    pub equal: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            heap: vec![],
            remainder: 0,
            equal: false,
        }
    }

    pub fn run(&mut self) {
        let mut successful = true;
        while successful {
            let instruction = self.read_next_instruction();
            match instruction {
                Some(instruction) => {
                    successful = self.execute_instruction(instruction);
                }
                None => {
                    successful = false;
                }
            }
        }
    }

    pub fn run_once(&mut self) {
        let instruction = self.read_next_instruction();
        match instruction {
            Some(instruction) => {
                self.execute_instruction(instruction);
            }
            None => {}
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.program.push(instruction);
    }

    pub fn read_next_instruction(&mut self) -> Option<Instruction> {
        if self.pc >= self.program.len() {
            return None;
        }
        let instruction = self.program[self.pc];
        self.pc += 1;
        return Some(instruction);
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> bool {
        match instruction.opcode {
            Opcode::LOAD => {
                let address = instruction.registers[0];
                let number = instruction.integer_operand;
                self.registers[address] = number as i32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::ADD => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];

                let address = instruction.registers[2];
                self.registers[address] = first_number + second_number;
            }
            Opcode::SUB => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];

                let address = instruction.registers[2];
                self.registers[address] = first_number - second_number;
            }
            Opcode::MUL => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];

                let address = instruction.registers[2];
                self.registers[address] = first_number * second_number;
            }
            Opcode::DIV => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];

                let address = instruction.registers[2];
                self.registers[address] = first_number / second_number;
                self.remainder = (first_number % second_number) as u32;
            }
            Opcode::JMP => {
                let address = self.registers[instruction.registers[0]];
                self.pc = address as usize;
            }
            Opcode::JMPF => {
                let address = self.registers[instruction.registers[0]];
                self.pc += address as usize;
            }
            Opcode::JMPB => {
                let address = self.registers[instruction.registers[0]];
                self.pc -= address as usize;
            }
            Opcode::EQ => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];
                self.equal = first_number == second_number;
            }
            Opcode::NEQ => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];
                self.equal = first_number != second_number;
            }
            Opcode::GT => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];
                self.equal = first_number > second_number;
            }
            Opcode::LT => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];
                self.equal = first_number < second_number;
            }
            Opcode::GTQ => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];
                self.equal = first_number >= second_number;
            }
            Opcode::LTQ => {
                let first_number = self.registers[instruction.registers[0]];
                let second_number = self.registers[instruction.registers[1]];
                self.equal = first_number <= second_number;
            }
            Opcode::JEQ => {
                let address = self.registers[instruction.registers[0]];
                if self.equal {
                    self.pc = address as usize;
                }
            }
            Opcode::ALOC => {
                let number_of_bytes = self.registers[instruction.registers[0]];
                let new_len = self.heap.len() + number_of_bytes as usize;
                self.heap.resize(new_len, 0);
            }
            Opcode::INC => {
                self.registers[instruction.registers[0]] += 1;
            }
            Opcode::DEC => {
                self.registers[instruction.registers[0]] -= 1;
            }
            _ => {
                return false;
            }
        }
        true
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_vm() {
//         let test_vm = VM::new();
//         assert_eq!(test_vm.registers[0], 0)
//     }

//     #[test]
//     fn test_opcode_hlt() {
//         let mut test_vm = VM::new();
//         let test_bytes = vec![0, 0, 0, 0];
//         test_vm.program = test_bytes;
//         test_vm.run();
//         assert_eq!(test_vm.pc, 1);
//     }

//     #[test]
//     fn test_opcode_igl() {
//         let mut test_vm = VM::new();
//         let test_bytes = vec![200, 0, 0, 0];
//         test_vm.program = test_bytes;
//         test_vm.run();
//         assert_eq!(test_vm.pc, 1);
//     }
//     #[test]
//     fn test_load_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.program = vec![1, 0, 1, 244];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 500);
//     }
//     #[test]
//     fn test_add_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.program = vec![
//             Opcode::LOAD.into(),
//             0,
//             0,
//             100,
//             Opcode::LOAD.into(),
//             1,
//             0,
//             100,
//             Opcode::ADD.into(),
//             0,
//             1,
//             2,
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[2], 200);
//     }
//     #[test]
//     fn test_sub_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.program = vec![
//             Opcode::LOAD.into(),
//             0,
//             0,
//             200,
//             Opcode::LOAD.into(),
//             1,
//             0,
//             100,
//             Opcode::SUB.into(),
//             0,
//             1,
//             2,
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[2], 100);
//     }
//     #[test]
//     fn test_mul_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.program = vec![
//             Opcode::LOAD.into(),
//             0,
//             0,
//             10,
//             Opcode::LOAD.into(),
//             1,
//             0,
//             10,
//             Opcode::MUL.into(),
//             0,
//             1,
//             2,
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[2], 100);
//     }
//     #[test]
//     fn test_div_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.program = vec![
//             Opcode::LOAD.into(),
//             0,
//             0,
//             100,
//             Opcode::LOAD.into(),
//             1,
//             0,
//             10,
//             Opcode::DIV.into(),
//             0,
//             1,
//             2,
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[2], 10);
//     }
//     #[test]
//     fn test_jmp_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.program = vec![
//             Opcode::LOAD.into(),
//             0,
//             0,
//             100,
//             Opcode::LOAD.into(),
//             1,
//             0,
//             12,
//             Opcode::JMP.into(),
//             1,
//             0,
//             0,
//             Opcode::ADD.into(),
//             0,
//             0,
//             2,
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[2], 200);
//     }
//     #[test]
//     fn test_jmpf_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 4;
//         test_vm.program = vec![Opcode::JMPF.into(), 0, 0, 0, Opcode::JMP.into(), 0, 0, 0];
//         test_vm.run_once();
//         test_vm.run_once();
//         assert_eq!(test_vm.pc, 8);
//     }
//     #[test]
//     fn test_jmpb_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 4;
//         test_vm.program = vec![Opcode::ADD.into(), 0, 0, 0, Opcode::JMPB.into(), 0, 0, 0];
//         test_vm.run_once();
//         test_vm.run_once();
//         assert_eq!(test_vm.pc, 0);
//     }
//     #[test]
// fn test_eq_opcode() {
//     let mut test_vm = VM::new();
//     test_vm.registers[0] = 10;
//     test_vm.registers[1] = 10;
//     test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
//     test_vm.run_once();
//     assert_eq!(test_vm.equal, true);
//     test_vm.registers[1] = 20;
//     test_vm.run_once();
//     assert_eq!(test_vm.equal, false);
// }
// }
