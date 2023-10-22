use core::str;
use std::{io, iter};

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
    let t = input_line.trim_matches('\n').to_string();

    for command in t.split(' ') {
        print!("{}", parse_command(command));
    }
}

fn parse_command(command: &str) -> String {
    let (count, mut text) = get_parts(command);

    text = match (text) {
        "nl" => "\n",
        "bS" => "\\",
        "sQ" => "'",
        "sp" => " ",
        _ => text,
    };

    return build_string(text, count);
}

fn get_parts(command: &str) -> (usize, &str) {
    if command == "nl" {
        return (1, "nl");
    }

    let value_index = command
        .chars()
        .position(|c| !char::is_digit(c, 10))
        .unwrap_or(command.len() - 1);

    return (
        command[..value_index].parse::<usize>().unwrap(),
        &command[value_index..],
    );
}

fn build_string(str: &str, count: usize) -> String {
    return iter::repeat(str).take(count).collect::<String>();
}
