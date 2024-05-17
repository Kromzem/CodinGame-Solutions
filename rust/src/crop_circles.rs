use std::{io, ops::Index};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const COORD_MAP: &'static str = "abcdefghijklmnopqrstuvwxy";

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let instructions = input_line.trim_matches('\n').split_whitespace();

    let mut field = Field::new();
    for i in instructions {
        field.perform_instruction(i);
    }
    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    field.output();
}

fn coord_to_index(coord: &str) -> (usize, usize) {
    (
        char_to_index(coord.chars().nth(0).expect("x missing")),
        char_to_index(coord.chars().nth(1).expect("y missing")),
    )
}

fn char_to_index(c: char) -> usize {
    COORD_MAP
        .find(c)
        .expect(&format!("char '{}' not found!", c))
}

struct Field {
    grid: [[bool; 19]; 25],
}

impl Field {
    pub fn new() -> Field {
        Field {
            grid: [[true; 19]; 25],
        }
    }

    pub fn perform_instruction(&mut self, instruction: &str) {
        let (operation, circle) = if instruction.starts_with("PLANTMOW") {
            (
                Operation::PlantMow,
                Circle::parse(&instruction.replace("PLANTMOW", "")),
            )
        } else if instruction.starts_with("PLANT") {
            (
                Operation::Plant,
                Circle::parse(&instruction.replace("PLANT", "")),
            )
        } else {
            (Operation::Mow, Circle::parse(instruction))
        };

        for (y, columns) in self.grid.iter_mut().enumerate() {
            for (x, tile) in columns.iter_mut().enumerate() {
                if !circle.is_in(x, y) {
                    continue;
                }

                *tile = match operation {
                    Operation::Plant => true,
                    Operation::Mow => false,
                    Operation::PlantMow => !*tile,
                };
            }
        }
    }

    pub fn output(&self) {
        for columns in self.grid {
            for tile in columns {
                let output = if tile { "{}" } else { "  " };

                print!("{}", output);
            }
            println!("")
        }
    }
}

struct Circle {
    x: usize,
    y: usize,
    d: usize,
}

impl Circle {
    pub fn parse(input: &str) -> Circle {
        let (coord, diameter) = input.split_at(2);

        let coord_indices = coord_to_index(coord);

        Circle {
            x: coord_indices.0,
            y: coord_indices.1,
            d: usize::from_str_radix(diameter, 10).expect("invalid diameter"),
        }
    }

    fn is_in(&self, x: usize, y: usize) -> bool {
        (((x - self.x).pow(2) + (y - self.y).pow(2)) as f32) < ((self.d as f32) / 2.0).powi(2)
    }
}

enum Operation {
    Plant,
    Mow,
    PlantMow,
}
