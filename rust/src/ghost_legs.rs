use std::{cell::RefCell, io};

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

    let mut lanes: Option<Vec<Lane>> = None;
    let mut lane_endings: Option<Vec<char>> = None;
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    for i in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();

        eprintln!("{}", &line);

        if i == 0 {
            lanes.insert(parse_start_lanes(&line));
        } else if i == (h - 1) {
            lane_endings.insert(parse_end_points(&line));
        } else {
            perform_lane_switches(&line, lanes.as_mut().unwrap());
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    for lane in lanes.unwrap() {
        println!(
            "{}{}",
            lane.name,
            lane_endings.as_ref().unwrap()[lane.end_index]
        );
    }
}

fn parse_start_lanes(line: &str) -> Vec<Lane> {
    return line
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .enumerate()
        .map(|p| Lane {
            name: p.1,
            end_index: p.0,
        })
        .collect();
}

fn parse_end_points(line: &str) -> Vec<char> {
    return line.chars().filter(|c| !c.is_ascii_whitespace()).collect();
}

fn perform_lane_switches(line: &str, lanes: &mut Vec<Lane>) {
    let switches = parse_lane_switches(line);

    for lane in lanes.iter_mut() {
        let finding = switches
            .iter()
            .enumerate()
            .find(|p| *p.1 && (p.0 == lane.end_index || p.0 == lane.end_index - 1));

        let Some(switch) = finding else {
            continue;
        };

        lane.move_position(switch.0);
    }
}

fn parse_lane_switches(line: &str) -> Vec<bool> {
    return line
        .split('|')
        .filter(|s| !s.is_empty())
        .map(|s| s == "--")
        .collect();
}

fn swap_lanes(first: &mut Lane, second: &mut Lane) {
    (first.end_index, second.end_index) = (second.end_index, first.end_index);
}

struct Lane {
    name: char,
    end_index: usize,
}

impl Lane {
    fn move_position(&mut self, switch_position: usize) {
        self.end_index = if switch_position == self.end_index {
            self.end_index + 1
        } else if (switch_position + 1) == self.end_index {
            self.end_index - 1
        } else {
            self.end_index
        }
    }
}
