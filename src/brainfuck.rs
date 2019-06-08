use std::io::stdin;
use std::num::Wrapping;
use std::fs;
use std::io;
use std::io::Write;

enum Instruction {
    Next,
    Prev,
    Incr,
    Decr,
    Input,
    Output,
    JumpOpen(usize),
    JumpClose(usize),
    Comment
}

impl From<char> for Instruction {
    fn from(c: char) -> Instruction {
        match c {
            '>' => Instruction::Next,
            '<' => Instruction::Prev,
            '+' => Instruction::Incr,
            '-' => Instruction::Decr,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::JumpOpen(0),
            ']' => Instruction::JumpClose(0),
            _ => Instruction::Comment
        }
    }
}

pub struct BrainfuckState {
    mem: Vec<Wrapping<u8>>,
    pointer: usize,
    instructions: Vec<Instruction>,
}

impl BrainfuckState {
    pub fn new() -> BrainfuckState {
        BrainfuckState {
            mem: vec![Wrapping(0u8); 30000],
            pointer: 0,
            instructions: Vec::new(),
        }
    }

    pub fn parse(&mut self, filename: &str) {
        let code = fs::read_to_string(filename)
            .expect(&format!("Something went wrong reading the file {}", filename));
        let mut s_brackets = Vec::new();
        let mut instr_id = 0;
        for b in code.bytes() {
            let mut inst: Instruction = Instruction::from(b as char);
            match inst {
                Instruction::Comment => {continue;}
                Instruction::JumpClose(_) => {
                    let i_open = s_brackets.pop().unwrap();
                    self.instructions[i_open] = Instruction::JumpOpen(instr_id - i_open);
                    inst = Instruction::JumpClose(instr_id - i_open);
                },
                Instruction::JumpOpen(_) => {
                    s_brackets.push(instr_id);
                },
                _ => {}
            }
            self.instructions.push(inst);
            instr_id += 1;
        }
    }
    pub fn execute(&mut self) {
        let mut i: usize = 0;
        while i < self.instructions.len() {
            i = self.step_instr(i);
        }
    }

    fn step_instr(&mut self, current_index: usize) -> usize {
        let mut index_out = current_index;
        match self.instructions[current_index] {
            Instruction::Next => self.pointer += 1,
            Instruction::Prev => self.pointer -= 1,
            Instruction::Incr => self.mem[self.pointer] += Wrapping(1u8),
            Instruction::Decr => self.mem[self.pointer] -= Wrapping(1u8),
            Instruction::Output => {
                print!("{}", self.mem[self.pointer].0 as char);
                io::stdout().flush().unwrap();
            },
            Instruction::Input => {
                let mut s: String = String::new();
                print!(":");
                stdin().read_line(&mut s).expect("Did not enter a correct string");
                let b: u8 = s.bytes().nth(0).expect("erreur");
                self.mem[self.pointer] = Wrapping(b);
            },
            Instruction::JumpOpen(offset) => {
                if self.mem[self.pointer].0 == 0 {
                    index_out += offset;
                }
            },
            Instruction::JumpClose(offset) => {
                if self.mem[self.pointer].0 != 0 {
                    index_out -= offset;
                }
            },
            _ => {}
        }
        index_out += 1;
        return index_out;
    }
}