use core::panic;
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
    games.push(Game::new(player_idx, Box::new(HurdleGameLogic::default())));
    games.push(Game::new(player_idx, Box::new(ArcheryGameLogic {})));
    games.push(Game::new(
        player_idx,
        Box::new(RollerSpeedSkatingGameLogic {}),
    ));
    games.push(Game::new(player_idx, Box::new(DivinGameLogic {})));

    // game loop
    loop {
        for i in 0..3 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let score_info = input_line.trim_matches('\n').to_string();
        }
        for i in 0..nb_games as usize {
            games[i].update();
        }

        println!("{}", calc_move(&games));
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
    }
}

fn calc_move(games: &Vec<Game>) -> String {
    return games[0].logic.calc_best_move().get_string();

    // let commands: Vec<Command> = games
    //     .iter()
    //     .map(|g| g.logic.calc_best_move())
    //     .filter(|c| !matches!(c, Command::None))
    //     .collect();

    // if commands.is_empty() {
    //     return Command::Left.get_string();
    // }

    // return commands.first().unwrap_or(Command::Left).
    // if commands.iter().any(|(_, c)| matches!(c, Command::Up)) {
    //     return Command::Up.get_string();
    // }

    // return commands
    //     .iter()
    //     .max_by_key(|(p, _)| p)
    //     .unwrap()
    //     .1
    //     .get_string();
}

trait GameLogic {
    fn update(&mut self, data: GameData);
    fn calc_best_move(&self) -> Command;
}

struct GameData {
    field: String,
    player_data: (i32, i32),
    enemy_data: Vec<(i32, i32)>,
    extra_data: i32,
}

impl GameData {
    fn receive(player_index: usize) -> GameData {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let gpu = inputs[0].trim().to_string();

        let mut registers: Vec<i32> = (1..8).map(|i| parse_input!(inputs[i], i32)).collect();
        let mut data_pairs: Vec<(i32, i32)> =
            (0..3).map(|i| (registers[i], registers[i + 3])).collect();

        let player_data = data_pairs.remove(player_index);

        GameData {
            field: gpu.to_string(),
            player_data,
            enemy_data: data_pairs,
            extra_data: registers.pop().unwrap(),
        }
    }
}

struct Game {
    // pos: usize,
    // hurdles: Vec<usize>,
    player_index: usize,
    is_over: bool,
    // stunned: bool,
    logic: Box<dyn GameLogic>,
}

impl Game {
    fn new(player_index: usize, logic: Box<dyn GameLogic>) -> Game {
        Game {
            player_index,
            logic,
            is_over: false,
        }
    }

    fn update(&mut self) {
        let data = GameData::receive(self.player_index);

        if data.field == "GAME_OVER" {
            self.is_over = true;
            return;
        }

        self.is_over = false;

        self.logic.update(data);
    }
}

#[derive(Default)]
struct HurdleGameLogic {
    pos: usize,
    hurdles: Vec<usize>,
    stunned: bool,
}

impl GameLogic for HurdleGameLogic {
    fn update(&mut self, data: GameData) {
        (self.pos, self.stunned) = (data.player_data.0 as usize, data.player_data.1 > 0);

        self.hurdles = data
            .field
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '#' => Some(i),
                _ => None,
            })
            .collect();
    }

    fn calc_best_move(&self) -> Command {
        let next_hurdle = self.hurdles.iter().filter(|&&p| p > self.pos).nth(0);

        let dist = match next_hurdle {
            Some(pos) => pos - self.pos,
            None => 0,
        };

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
}

struct ArcheryGameLogic {}

impl GameLogic for ArcheryGameLogic {
    fn update(&mut self, data: GameData) {
        //todo
    }

    fn calc_best_move(&self) -> Command {
        Command::None
    }
}

struct RollerSpeedSkatingGameLogic {}

impl GameLogic for RollerSpeedSkatingGameLogic {
    fn update(&mut self, data: GameData) {
        //todo
    }

    fn calc_best_move(&self) -> Command {
        Command::None
    }
}

struct DivinGameLogic {}

impl GameLogic for DivinGameLogic {
    fn update(&mut self, data: GameData) {
        //todo
    }

    fn calc_best_move(&self) -> Command {
        Command::None
    }
}

enum Command {
    Up,
    Down,
    Right,
    Left,
    None,
}

impl Command {
    fn get_string(&self) -> String {
        match self {
            Command::Up => "UP",
            Command::Down => "DOWN",
            Command::Right => "RIGHT",
            Command::Left => "LEFT",
            Command::None => panic!("None should not be printed!"),
        }
        .to_string()
    }
}
