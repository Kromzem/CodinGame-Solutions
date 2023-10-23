use std::io;

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
    let n = parse_input!(input_line, i32);

    let mut points: Vec<Point> = (0..n).map(|_| read_point()).collect();
    let first = points.remove(0);
    let mut total_distance = 0.0;

    let mut last: Option<Point> = None;
    while !points.is_empty() {
        let (index, distance) = get_smallest_distance(last.unwrap_or(first), &points);

        last = Some(points.remove(index));
        total_distance += distance;
    }

    total_distance += last.unwrap().get_distance_to(&first);

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", total_distance.round() as i32);
}

fn read_point() -> Point {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x = parse_input!(inputs[0], i32) as f32;
    let y = parse_input!(inputs[1], i32) as f32;

    return Point { x, y };
}

fn get_smallest_distance(last_point: Point, points: &Vec<Point>) -> (usize, f32) {
    let mut smallest_index = 0;
    let mut smallest_distance = f32::MAX;

    for (i, point) in points.iter().enumerate() {
        let distance = point.get_distance_to(&last_point);

        if distance >= smallest_distance {
            continue;
        }

        smallest_index = i;
        smallest_distance = distance;
    }

    return (smallest_index, smallest_distance);
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn get_distance_to(&self, other: &Point) -> f32 {
        return ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    }
}
