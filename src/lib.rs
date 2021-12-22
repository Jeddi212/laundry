use std::io::{stdin, stdout, Write};

pub fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}