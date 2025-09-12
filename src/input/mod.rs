use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    String::from(input.trim())
}
