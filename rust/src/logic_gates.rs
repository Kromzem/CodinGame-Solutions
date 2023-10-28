use std::{collections::HashMap, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const ZERO: char = '_';
const ONE: char = '-';

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let mut signals: HashMap<String, String> = HashMap::new();

    let m = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let input_name = inputs[0].trim().to_string();
        let input_signal = inputs[1].trim().to_string();

        signals.insert(input_name, input_signal);
    }

    for i in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let output_name = inputs[0].trim().to_string();
        let _type = inputs[1].trim().to_string();
        let input_name_1 = inputs[2].trim().to_string();
        let input_name_2 = inputs[3].trim().to_string();

        let first = signals.get(&input_name_1).unwrap();
        let second = signals.get(&input_name_2).unwrap();

        println!("{} {}", output_name, process(first, second, &_type));
    }
}

fn process(first: &str, second: &str, operator: &str) -> String {
    return first
        .chars()
        .zip(second.chars())
        .map(|p| logic_compare(p.0, p.1, operator))
        .collect();
}

fn logic_compare(first: char, second: char, operator: &str) -> char {
    let first_bool = char_to_bool(first);
    let second_bool = char_to_bool(second);

    let mut result = match operator {
        "AND" | "NAND" => first_bool && second_bool,
        "OR" | "NOR" => first_bool || second_bool,
        "XOR" | "NXOR" => (first_bool || second_bool) && first_bool != second_bool,
        _ => panic!("Invalid operator {}", operator),
    };

    if operator.starts_with('N') {
        result = !result;
    }

    return bool_to_char(result);
}

fn char_to_bool(c: char) -> bool {
    return c == ONE;
}

fn bool_to_char(b: bool) -> char {
    return if b { ONE } else { ZERO };
}
