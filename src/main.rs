use std::io::stdin;
use std::num::Wrapping;
use std::fs;

fn main() {
    let mut bf: BrainfuckState = BrainfuckState::new();
    bf.execute("../samples/bench.bf");
}


struct BrainfuckState {
    mem: Vec<Wrapping<u8>>,
    pointer: usize,
}

impl BrainfuckState {
    fn new() -> BrainfuckState {
        BrainfuckState {
            mem: vec![Wrapping(0u8); 30000],
            pointer: 0
        }
    }

    fn execute(&mut self, filename: &str) {
        let code = fs::read_to_string(filename)
            .expect(&format!("Something went wrong reading the file {}", filename));
        let mut i: usize = 0;
        while i < code.len() {
            match code.as_bytes()[i] as char {
                '>' => self.pointer += 1,
                '<' => self.pointer -= 1,
                '+' => self.mem[self.pointer] += Wrapping(1u8),
                '-' => self.mem[self.pointer] -= Wrapping(1u8),
                '.' => {
                    print!("{}", self.mem[self.pointer].0 as char);
                },
                ',' => {
                    let mut s: String = String::new();
                    print!(":");
                    stdin().read_line(&mut s).expect("Did not enter a correct string");
                    let b: u8 = s.bytes().nth(0).expect("erreur");
                    self.mem[self.pointer] = Wrapping(b);
                }
                '[' => {
                    if self.mem[self.pointer].0 == 0 {
                        let mut counter = 0;
                        while i < code.len() {
                            i += 1;
                            match code.as_bytes()[i] as char {
                                '[' => {
                                    counter += 1;
                                }
                                ']' => {
                                    if counter == 0 {
                                        break;
                                    }
                                    counter -= 1;
                                }
                                _ => {}
                            }
                        }
                    }
                },
                ']' => {
                    if self.mem[self.pointer].0 != 0 {
                        let mut counter = 0;
                        loop {
                            i -= 1;
                            match code.as_bytes()[i] as char {
                                ']' => {
                                    counter += 1;
                                }
                                '[' => {
                                    if counter == 0 {
                                        break;
                                    }
                                    counter -= 1;
                                }
                                _ => {}
                            }
                        }
                        i -= 1;
                    }
                },
                _ => {}
            }
            i += 1;
        }
    }
}