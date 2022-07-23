use std::{fs, io::Read};

use brainfuck_inter::parser;

fn main() {
    let mut file = fs::File::open(std::env::args().nth(1).expect("Expected a file path.")).expect("Could not open file.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read the file.");

    // Standard Brainfuck says the array should have 30000 1 byte elements.
    let mut arr = vec![0; 30000];
    let mut index = 0;
    for elem in parser::parse(&input) {
        elem.interpret(&mut arr, &mut index);
    }
}
