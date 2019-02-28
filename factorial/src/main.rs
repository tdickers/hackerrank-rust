use std::io;
use std::str::FromStr;

fn main() {
    let n:u32 = read_line_parsed();
    println!("{}", factorial(n));
}

fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return n * factorial(n - 1);
}

fn read_line() -> String {
    let mut read_line_contents = String::new();
    let _ = io::stdin().read_line(&mut read_line_contents);
    return read_line_contents.trim().to_string();
}

fn read_line_parsed<T>() -> T where T: FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_line().parse().expect("Read line failed.");
}
