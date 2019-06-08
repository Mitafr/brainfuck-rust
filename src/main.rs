mod brainfuck;

use crate::brainfuck::BrainfuckState;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut bf: BrainfuckState = BrainfuckState::new();
    bf.parse(&args[1]);
    bf.execute();
}
