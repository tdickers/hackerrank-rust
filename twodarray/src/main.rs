use std::io;
use std::io::prelude::*;

fn main() {
    let mut matrix: Vec<Vec<i8>> = Vec::new();
    read_array(&mut matrix);

    println!("{}", calc_hourglass_sums(&mut matrix).iter().max().unwrap());
}

// TODO try and determine size from just stdin
fn read_array(matrix: &mut Vec<Vec<i8>>) -> () {
    let stdin = io::stdin();
    let mut num_read: usize = 0;
    let mut matrix_dim: usize = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let row: Vec<i8> = line.split(char::is_whitespace)
            .map(|c| c.parse().unwrap())
            .collect();

        if matrix_dim == 0 {
            matrix_dim = row.len();
        }

        matrix.push(row);
        num_read += 1;

        if num_read >= matrix_dim {
            break;
        }
    }
}

fn calc_hourglass_sums(matrix: &mut Vec<Vec<i8>>) -> Vec<i8> {
    let mut sums = Vec::<i8>::new();

    for y in 0..(matrix.len() - 2) {
        for x in 0..(matrix.len() - 2) {
            let mut sum: i8 = matrix[y][x..x + 3]
                .iter()
                .fold(0, |accum, &int| accum + int);

            sum += matrix[y + 1][x + 1];

            sum += matrix[y + 2][x..x + 3]
                .iter()
                .fold(0, |accum, &int| accum + int);

            sums.push(sum);
        }
    }

    return sums;
}
