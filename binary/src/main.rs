use std::io;
use std::str::FromStr;

fn main() {
    let n:usize = read_line_parsed();
    let (_, bin_vec) = binary_rep(n);

    let mut longest_1s_run = 0;
    let mut current_1s_run = 0;
    for i in bin_vec.iter() {
        match i {
            &0 => {
                if current_1s_run > longest_1s_run {
                    longest_1s_run = current_1s_run;
                }
                current_1s_run = 0;
            },
            &1 => {
                current_1s_run += 1;
            },
            _ => panic!("I think I saw a 2!")
        }
        if current_1s_run > longest_1s_run {
            longest_1s_run = current_1s_run;
        }
    }
    println!("{}", longest_1s_run);
}

fn binary_rep(mut n: usize) -> (usize, Vec<usize>) {
    let mut n_base_2:Vec<usize> = Vec::new();
    while n > 0 {
        n_base_2.insert(0, n % 2);
        n /= 2;
    }

    // represent binary using a usize with multiples of 10
    let base_2_rep = n_base_2.iter().enumerate().fold(0, |accum, (index, int)| {
        let len = n_base_2.len();
        let pow:usize = len - index - 1;
        accum + int * 10_usize.pow(pow as u32)
    });
    return (base_2_rep, n_base_2)
}

fn read_line() -> String {
    let mut read_line_contents = String::new();
    let _ = io::stdin().read_line(&mut read_line_contents);
    return read_line_contents.trim().to_string();
}

fn read_line_parsed<T>() -> T where T: FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_line().parse().expect("Read line failed.");
}
