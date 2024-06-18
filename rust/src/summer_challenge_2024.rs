use std::{borrow::Borrow, default, io, iter::repeat};

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
    let player_idx = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nb_games = parse_input!(input_line, usize);

    let mut games = Vec::with_capacity(4);

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

            let registers: Vec<usize> = (1..8).map(|i| parse_input!(inputs[i], usize)).collect();

            if games.len() < nb_games {
                games.push(Game::default());
            }

            if gpu == "GAME_OVER" {
                games[i].is_over = true;
                continue;
            }

            games[i].set_hurdles(&gpu);
            games[i].pos = registers[player_idx];
            games[i].stunned = registers[player_idx + 3] > 0;
        }

        println!("{}", calc_move(&games));
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
    }
}

fn calc_move(games: &Vec<Game>) -> String {
    let commands: Vec<(usize, Command)> = games
        .iter()
        .filter(|g| !g.stunned)
        .map(|g| (g.pos, g.calc_next_move()))
        .collect();

    if commands.is_empty() {
        return Command::Left.get_string();
    }

    if commands.iter().any(|(_, c)| matches!(c, Command::Up)) {
        return Command::Up.get_string();
    }

    return commands
        .iter()
        .max_by_key(|(p, _)| p)
        .unwrap()
        .1
        .get_string();
}

#[derive(Default)]
struct Game {
    pos: usize,
    hurdles: Vec<usize>,
    is_over: bool,
    stunned: bool,
}

impl Game {
    fn set_hurdles(&mut self, field: &str) {
        self.hurdles = field
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '#' => Some(i),
                _ => None,
            })
            .collect();
    }

    fn calc_next_move(&self) -> Command {
        let next_hurdle = self.hurdles.iter().filter(|&&p| p > self.pos).nth(0);

        let dist = match next_hurdle {
            Some(pos) => pos - self.pos,
            None => 0,
        };

        Command::for_dist(dist)
    }
}

enum Command {
    Up,
    Down,
    Right,
    Left,
}

impl Command {
    fn distance(&self) -> usize {
        match self {
            Command::Up => 2,
            Command::Down => 2,
            Command::Right => 3,
            Command::Left => 1,
        }
    }

    pub fn for_dist(dist: usize) -> Command {
        if dist == 1 {
            Command::Up
        } else if dist == 2 {
            Command::Left
        } else if dist == 3 {
            Command::Down
        } else {
            Command::Right
        }
    }

    pub fn to_dist(&self) -> usize {
        match self {
            Command::Up => 2,
            Command::Down => 2,
            Command::Right => 3,
            Command::Left => 1,
        }
    }

    fn get_string(&self) -> String {
        match self {
            Command::Up => "UP",
            Command::Down => "DOWN",
            Command::Right => "RIGHT",
            Command::Left => "LEFT",
        }
        .to_string()
    }
}
