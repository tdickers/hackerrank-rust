#[macro_use]
extern crate text_io;

mod inheritance;
use inheritance::Printable;

fn main() {
    let first_name: String;
    let last_name: String;
    let id: usize;
    let _num_scores: usize;
    let scores_line: String;
    let scores: Vec<u8>;

    scan!("{} {} {}\n", first_name, last_name, id);
    _num_scores = read!("{}\n");
    scores_line = read!("{}\n");

    scores = scores_line.split(" ").map(|s| s.parse().unwrap()).collect();

    let student = inheritance::Student::new(first_name, last_name, id, scores);
    student.print();
}
