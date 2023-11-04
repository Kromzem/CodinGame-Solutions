use core::panic;
use std::{collections::HashMap, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);

    let mut resistances = HashMap::with_capacity(n);
    for i in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let name = inputs[0].trim().to_string();
        let r = parse_input!(inputs[1], f32);

        resistances.insert(name, r);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let parts = input_line
        .trim_matches('\n')
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let circuit = Circuit { resistances, parts };

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!(
        "{:.1}",
        (circuit.calc_total_resistance() * 10.0).round() / 10.0
    );
}

fn calc_res_sum(sub_type: &str, resistances: &Vec<f32>) -> f32 {
    if sub_type == "(" {
        return resistances.iter().copied().sum::<f32>();
    }

    return 1f32 / resistances.iter().map(|f| 1f32 / f).sum::<f32>();
}

struct Circuit {
    resistances: HashMap<String, f32>,
    parts: Vec<String>,
}

impl Circuit {
    fn calc_total_resistance(&self) -> f32 {
        return self.calc_sub_resistance(0).0;
    }

    fn calc_sub_resistance(&self, start_index: usize) -> (f32, usize) {
        let sub_type = &self.parts[start_index];

        let mut res_values = Vec::new();

        let mut index = start_index + 1;
        while index < self.parts.len() {
            let part = &self.parts[index];

            if part == "(" || part == "[" {
                let sub_pair = self.calc_sub_resistance(index);

                res_values.push(sub_pair.0);
                index = sub_pair.1;
            } else if (sub_type == "(" && part == ")") || (sub_type == "[" && part == "]") {
                return (calc_res_sum(sub_type, &res_values), index);
            } else {
                res_values.push(self.resistances[part]);
            }

            index += 1;
        }

        panic!("Cannot complete sub circuit o.O");
    }
}
