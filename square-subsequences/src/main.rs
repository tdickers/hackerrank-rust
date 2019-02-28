use std::io::{self, BufRead, BufReader};

fn read_line<R: BufRead>(mut reader: R) {
    let mut read_value = String::new();
    let _ = reader.read_line(&mut read_value).expect("Unable to read");
    println!("{}", read_value)
}

fn main() {
    let stdin = io::stdin();
    read_line(stdin.lock());
}

#[test]
fn test_read_line() {
    let input = "abc123";
    let x:i32 = input;
    read_line(BufReader::new(input.as_bytes()));
}
