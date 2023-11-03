use core::panic;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const ABC: &str = "abcdefghijklmnopqrstuvwxyz";

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let n = parse_input!(input_line, usize);

    let mut map = vec![vec!['a'; n]; n];
    let mut path: Vec<Coord> = Vec::new();
    let mut start_points: Vec<Coord> = Vec::new();
    for y in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let m = input_line.trim();

        for (x, c) in m.chars().enumerate() {
            map[x][y] = c;

            if c == 'a' {
                start_points.push(Coord { x, y });
            }
        }
    }

    let path = build_path(&map, &start_points);

    for y in 0..n {
        for x in 0..n {
            let c = if path.iter().find(|co| co.x == x && co.y == y).is_some() {
                map[x][y]
            } else {
                '-'
            };

            print!("{}", c);
        }
        println!();
    }
}

fn build_path(map: &Vec<Vec<char>>, start_points: &Vec<Coord>) -> Vec<Coord> {
    for start in start_points {
        let path_option = build_path_from_coord(map, start.clone());

        if let Some(path) = path_option {
            return path;
        }
    }

    panic!("Could not find path o.O");
}

fn build_path_from_coord(map: &Vec<Vec<char>>, coord: Coord) -> Option<Vec<Coord>> {
    let mut path = Vec::new();

    path.push(coord);
    for search in ABC.chars().skip(1) {
        let last_coord = path.last().unwrap();

        let found = find_matching_coord(search, last_coord, map);
        if let Some(coord) = found {
            path.push(coord);
            continue;
        }

        return None;
    }

    return Some(path);
}

fn find_matching_coord(target: char, current_coord: &Coord, map: &Vec<Vec<char>>) -> Option<Coord> {
    let x = current_coord.x;
    let y = current_coord.y;

    let to_check = [
        Coord { x: x + 1, y },
        Coord { x: x - 1, y },
        Coord { x, y: y + 1 },
        Coord { x, y: y - 1 },
    ];

    for check in to_check {
        let column = map.get(check.x);

        if column.is_none() {
            continue;
        }

        let c = column.unwrap().get(check.y);

        if c.is_none() {
            continue;
        }

        if *c.unwrap() == target {
            return Some(check);
        }
    }

    return None;
}

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}
