use std::fs;

pub mod vm;
pub mod instruction;
pub mod repl;
pub mod lexer;

fn main() {
    // let mut repl = repl::REPL::new();
    // repl.run();

    let mut vm = vm::VM::new();
    let asm = fs::read_to_string("test.asm").unwrap();
    let mut lexer = lexer::Lexer::new(asm);

    loop {
        match lexer.next_line() {
            Some(instruction) => {
                vm.add_instruction(instruction)
            }
            None => {
                break;
            }
        }
    }
    vm.run();
    for instruction in vm.program {
        print!("{:?} {:?} {} ", instruction.opcode, instruction.registers, instruction.integer_operand);
    }
    println!();
    for register in vm.registers {
        print!("{} ", register);
    }
    println!();
}
