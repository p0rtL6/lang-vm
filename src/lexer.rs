use std::string;

use crate::instruction::{self, Instruction, Opcode};

pub struct Lexer {
    lines: Vec<String>,
    lc: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            lines: input.split("\n").map(|line| {line.to_owned()}).collect(),
            lc: 0,
        }
    }

    pub fn next_line(&mut self) -> Option<Instruction> {
        if self.lc >= self.lines.len() {
            return None;
        }
        let mut opcode = Opcode::IGL;
        let mut registers = [0usize; 3];
        let mut integer_operand = 0;

        let string_tokens: Vec<&str> = self.lines[self.lc].split_whitespace().collect();
        for (index, string_token) in string_tokens.into_iter().enumerate() {
            if string_token.starts_with('$') {
                registers[index - 1] = string_token.strip_prefix('$').unwrap().parse().unwrap();
            } else if string_token.starts_with('#') {
                integer_operand = string_token.strip_prefix('#').unwrap().parse().unwrap();
            } else if string_token.starts_with('.') {
            } else if string_token.starts_with('@') {
            } else if string_token.ends_with(':') {
            } else {
                opcode = string_token.into();
            }
        }
        self.lc += 1;
        Some(Instruction { opcode, registers, integer_operand })
    }
}

pub enum Token {
    Instruction(Instruction),
}

pub fn parse_program(program: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    let lines = program.split("\n");
    for line in lines {
        let mut instruction_pieces: Vec<&str> = line.split(" ").collect();
        let opcode: Opcode = instruction_pieces.remove(0).into();

        let mut registers = [0usize; 3];
        let mut integer_operand: i32 = 0;

        for (index, instruction_piece) in instruction_pieces.iter().enumerate() {
            let mut instruction_chars = instruction_piece.chars();
            let prefix = instruction_chars.nth(0).unwrap();
            let instruction_piece: String = instruction_chars.collect();

            if prefix == '$' {
                registers[index] = instruction_piece.parse().unwrap();
            } else if prefix == '#' {
                integer_operand = instruction_piece.parse().unwrap();
            } else {
                continue;
            }
        }

        let instruction = Instruction {
            opcode,
            registers,
            integer_operand,
        };

        instructions.push(instruction);
    }

    return instructions;
}
