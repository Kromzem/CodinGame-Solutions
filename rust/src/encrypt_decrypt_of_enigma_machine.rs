use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let is_encode = input_line.trim_matches('\n') == "ENCODE";
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let mut rotors = Vec::with_capacity(3);
    let pseudo_random_number = parse_input!(input_line, i32);
    for i in 0..3 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let rotor = input_line.trim_matches('\n').to_string();

        rotors.push((ABC, rotor));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut message = input_line.trim_matches('\n').to_string();

    if is_encode {
        message = rotate(&message, pseudo_random_number, 1);

        for (source, target) in rotors {
            message = perform_crypto_algorithm(&message, source, &target);
        }
    } else {
        for (target, source) in rotors.iter().rev() {
            message = perform_crypto_algorithm(&message, source, &target);
        }

        message = rotate(&message, pseudo_random_number, -1);
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", message);
}

fn rotate(to_rotate: &str, number: i32, direction: i32) -> String {
    return to_rotate
        .chars()
        .enumerate()
        .map(|c| rotate_char(c.1, (number + c.0 as i32) * direction))
        .collect();
}

fn rotate_char(c: char, number: i32) -> char {
    let mut index = ABC.chars().position(|x| c == x).unwrap() as i32;

    index = (index + number).rem_euclid(ABC.len() as i32);

    return ABC.chars().nth(index as usize).unwrap();
}

fn perform_crypto_algorithm(to_crypt: &str, source_abc: &str, target_abc: &str) -> String {
    return to_crypt
        .chars()
        .map(|c| transform_char(c, source_abc, target_abc))
        .collect();
}

fn transform_char(c: char, source_abc: &str, target_abc: &str) -> char {
    let index = source_abc.chars().position(|x| c == x).unwrap();

    return target_abc.chars().nth(index).unwrap();
}
