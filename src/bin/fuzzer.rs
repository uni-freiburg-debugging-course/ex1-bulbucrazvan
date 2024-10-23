use std::{fs, io::Write, env};
use rand::{seq::SliceRandom, Rng};

fn main() {
    let mut file = fs::File::create("test.txt").unwrap();
    let total_outputs: i32 = env::args().nth(1)
        .expect("Expected number of outputs as argument.")
        .parse()
        .expect("Expected an integer for number of outputs.");
    let operators = ['+', '-', '*'];

    let mut output = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..total_outputs {
        let operand1 = rng.gen_range(i32::MIN .. i32::MAX);
        let operand2 = rng.gen_range(i32::MIN .. i32::MAX);
        let operator = operators.choose(&mut rng).unwrap();
        output.push_str(&format!("(simplify ({operator} {operand1} {operand2}))\n"));
    }

    file.write_all(output.as_bytes()).unwrap();
}