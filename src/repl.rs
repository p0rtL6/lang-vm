use crate::{
    instruction::{self, Instruction},
    lexer::{self, Lexer},
    vm::VM,
};
use std::{
    self,
    io::{self, Write},
    num::ParseIntError,
};

pub struct REPL {
    command_history: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            vm: VM::new(),
            command_history: vec![],
        }
    }
    pub fn run(&mut self) {
        let mut buffer = String::new();

        println!("[LANG-VM REPL]");
        loop {
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            buffer = buffer.trim().to_string();

            self.command_history.push(buffer.clone());

            match buffer.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "history" => {
                    for command in &self.command_history {
                        println!("{}", command);
                    }
                }
                "program" => {
                    // for instruction in &self.vm.program {
                    //     println!("{}", instruction);
                    // }
                }
                "registers" => {
                    for register in self.vm.registers {
                        print!("{} ", register);
                    }
                    println!();
                }
                _ => {
                    let instruction = Lexer::new(buffer.clone()).next_line().unwrap();
                    self.vm.add_instruction(instruction);
                    self.vm.run_once();
                }
            }
            buffer.clear();
        }
    }
    // pub fn parse_hex(&mut self, hex: &str) -> Result<Instruction, ParseIntError> {
    //     let hex_bytes: Vec<&str> = hex.split(" ").collect();
    //     let mut bytes = [0u8; 4];
    //     for (index, hex_byte) in hex_bytes.iter().enumerate() {
    //         match u8::from_str_radix(hex_byte, 16) {
    //             Ok(byte) => {
    //                 bytes[index] = byte;
    //             },
    //             Err(e) => {
    //                 return Err(e);
    //             }
    //         }
    //     }
    //     return Ok(bytes.into());
    // }
}
