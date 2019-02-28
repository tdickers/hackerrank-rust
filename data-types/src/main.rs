use std::io;
use std::str::FromStr;

fn main() {
    let i = 4;
    let d = 4.0;
    let s = "HackerRank ";

    let read_int:i32 = read_line_parsed();
    let read_double:f64 = read_line_parsed();
    let read_string = read_line();

    println!("{}", i + read_int);
    println!("{:.1}", d + read_double);
    println!("{}{}", s, read_string);
}

fn read_line() -> String {
    let mut read_line_contents = String::new();
    let _ = io::stdin().read_line(&mut read_line_contents);
    return read_line_contents.trim().to_string();
}

fn read_line_parsed<T>() -> T where T: FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_line().parse().expect("Read line failed.");
}
