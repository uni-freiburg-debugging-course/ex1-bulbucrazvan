#![allow(arithmetic_overflow)]
use std::{fs, env};

fn main() {
    let file_path = env::args().nth(1).expect("Expected file path as argument.");
    let contents: Vec<Vec<char>> = fs::read_to_string(&file_path)
        .expect("Error reading file.")
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.chars().collect())
        .collect();

    let result = parse(&contents);
    println!("{result}");
}

fn parse(content: &Vec<Vec<char>>) -> String {
    let mut output = String::new();
    for line in content {
        output.push_str(&parse_stmt(&line[..]));
        output.push('\n');
    }
    output
}

fn parse_stmt(stmt: &[char]) -> String {
    if let Some(expr) = check_parantheses(&stmt) {
        let split_index = expr.iter().position(|x| *x == ' ').unwrap();
        let keyword =  String::from_iter(expr[..split_index].iter());
        parse_expr(&expr[split_index + 1 ..], &keyword)
    } else {
        String::from("Not a statement!")
    }
}

fn parse_expr(expr: &[char], keyword: &str) -> String {
    if keyword != "simplify" {
        return String::from("Invalid keyword");
    }
    if let Some(expr_pars) = check_parantheses(&expr) {
        let parts = expr_pars.split(|x| *x == ' ').collect::<Vec<&[char]>>();
        let operand1 = parts[1].iter().collect::<String>().parse::<i32>();
        if operand1.is_err() { return String::from("Failure parsing operand 1.") }
        let operand2 = parts[2].iter().collect::<String>().parse::<i32>();
        if operand2.is_err() { return String::from("Failure parsing operand 2.") }
        let result = match parts[0] {
            ['+'] => operand1.unwrap().checked_add(operand2.unwrap()),
            ['-'] => operand1.unwrap().checked_sub(operand2.unwrap()),
            ['*'] => operand1.unwrap().checked_mul(operand2.unwrap()),
            _ => return String::from("Invalid operator")
        };
        if let Some(result) = result {
            format!("({result})").to_string()
        }
        else {
            String::from("Overflow")
        }
    }
    else {
        String::from("Not an expression!")
    }   
}


fn check_parantheses(expr: &[char]) -> Option<&[char]> {
    if expr.first().is_some_and(|x| *x == '(') && expr.last().is_some_and(|x| *x == ')') {
        Some(&expr[1 .. expr.len() - 1])
    }
    else {
        None
    }
}