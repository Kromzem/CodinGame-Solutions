use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const WIDTH: u8 = 17;
const HEIGHT: u8 = 9;
const START_X: u8 = 8;
const START_Y: u8 = 4;
const UP_LEFT: u8 = 0b_00;
const UP_RIGHT: u8 = 0b_01;
const DOWN_LEFT: u8 = 0b_10;
const DOWN_RIGHT: u8 = 0b_11;

const CHARS: [char; 15] = [
    ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^',
];

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let fingerprint = input_line.trim_matches('\n').to_string();

    let mut bishop = spawn_bishop();
    let mut field = (0..WIDTH)
        .map(|_| {
            (0..HEIGHT)
                .map(|_| Tile { visit_count: 0 })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();
    for b_move in parse_moves_from_fingerprint(&fingerprint) {
        bishop.perform_move(b_move);
        let mut tile = &mut field[bishop.x as usize][bishop.y as usize];
        tile.visit();
    }

    let mut field_str = String::new();

    field_str.push_str("+---[CODINGAME]---+\n");
    for y in 0..HEIGHT {
        field_str.push('|');
        for x in 0..WIDTH {
            if x == START_X && y == START_Y {
                field_str.push('S');
                continue;
            }

            if x == bishop.x && y == bishop.y {
                field_str.push('E');
                continue;
            }

            field_str.push((&field[x as usize][y as usize]).get_visit_char());
        }
        field_str.push_str("|\n");
    }
    field_str.push_str("+-----------------+");

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", &field_str);
}

fn parse_moves_from_fingerprint(fingerprint: &str) -> Vec<u8> {
    let mut moves: Vec<u8> = Vec::new();

    for hex in fingerprint.split(':') {
        moves.append(&mut parse_moves_from_hex(hex));
    }

    return moves;
}

fn parse_moves_from_hex(hex: &str) -> Vec<u8> {
    let byte = u8::from_str_radix(hex, 16).unwrap();
    let byte_str = format!("{byte:08b}");

    let mut moves: Vec<u8> = Vec::new();
    for i in 0..(byte_str.len() / 2) {
        moves.push(u8::from_str_radix(&byte_str[(i * 2)..(i * 2 + 2)], 2).unwrap());
    }

    let mut ordered: Vec<u8> = Vec::with_capacity(8);
    ordered.extend(moves[..4].iter().rev());
    ordered.extend(moves[4..].iter().rev());

    return ordered;
}

fn spawn_bishop() -> Bishop {
    return Bishop {
        x: START_X,
        y: START_Y,
    };
}

struct Bishop {
    x: u8,
    y: u8,
}

impl Bishop {
    fn perform_move(&mut self, b: u8) {
        let (x, y): (i8, i8) = match b {
            UP_LEFT => (-1, -1),
            DOWN_LEFT => (-1, 1),
            UP_RIGHT => (1, -1),
            DOWN_RIGHT => (1, 1),
            _ => (0, 0),
        };

        self.x = i8::clamp(x + self.x as i8, 0, WIDTH as i8 - 1) as u8;
        self.y = i8::clamp(y + self.y as i8, 0, HEIGHT as i8 - 1) as u8;
    }
}

struct Tile {
    visit_count: u8,
}

impl Tile {
    fn visit(&mut self) {
        self.visit_count = (self.visit_count + 1) % 15;
    }

    fn get_visit_char(&self) -> char {
        return CHARS[self.visit_count as usize];
    }
}
