use std::{borrow::Borrow, io};

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
    let player_idx = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nb_games = parse_input!(input_line, i32);

    // game loop
    loop {
        for i in 0..3 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let score_info = input_line.trim_matches('\n').to_string();
        }
        for i in 0..nb_games as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let gpu = inputs[0].trim().to_string();
            let reg_0 = parse_input!(inputs[1], usize);
            let reg_1 = parse_input!(inputs[2], i32);
            let reg_2 = parse_input!(inputs[3], i32);
            let reg_3 = parse_input!(inputs[4], i32);
            let reg_4 = parse_input!(inputs[5], i32);
            let reg_5 = parse_input!(inputs[6], i32);
            let reg_6 = parse_input!(inputs[7], i32);

            if gpu == "GAME_OVER" {
                println!("LEFT");
                continue;
            }

            let next_hurdle = gpu
                .chars()
                .enumerate()
                .skip(reg_0)
                .filter_map(|(i, c)| match c {
                    '#' => Some(i),
                    _ => None,
                })
                .nth(0);

            let command = if let Some(pos) = next_hurdle {
                let dist = pos - reg_0;
                eprintln!("{} - {} = {}", pos, reg_0, dist);

                if dist == 1 {
                    "UP"
                } else if dist == 2 {
                    "LEFT"
                } else if dist == 3 {
                    "DOWN"
                } else {
                    "RIGHT"
                }
            } else {
                "RIGHT"
            };

            println!("{}", command);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
    }
}
