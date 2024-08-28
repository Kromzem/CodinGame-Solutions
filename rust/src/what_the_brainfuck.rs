use core::panic;
use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

fn main() {
    if let Err(err) = perform() {
        println!("{err}");
    }
}

fn perform() -> Result<(), &'static str> {
    let mut program = load()?;
    program.run()?;

    Ok(())
}

fn load() -> Result<Program, &'static str> {
    let (line_count, register_count, input_count) = read_meta_data();

    let commands = read_commands(line_count);
    let connections = search_portal_connections(&commands)?;

    Ok(Program {
        commands,
        portal_connections: connections,
        register: vec![0; register_count],
        inputs: VecDeque::from(read_input(input_count)),
        ..Default::default()
    })
}

fn read_meta_data() -> (usize, usize, usize) {
    let mut input_line = String::new();
    stdin().read_line(&mut input_line).unwrap();

    let values: Vec<usize> = input_line
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    (values[0], values[1], values[2])
}

fn read_commands(line_count: usize) -> Vec<Command> {
    let mut input = String::new();

    for _ in 0..line_count {
        stdin().read_line(&mut input).unwrap();
    }
    eprintln!("{input}");
    input
        .chars()
        .into_iter()
        .filter_map(|c| match c {
            '>' => Some(Command::IncrementPointer),
            '<' => Some(Command::DecrementPointer),
            '+' => Some(Command::IncrementValue),
            '-' => Some(Command::DecrementValue),
            '.' => Some(Command::OutputValue),
            ',' => Some(Command::ReadInput),
            '[' => Some(Command::BluePortal),
            ']' => Some(Command::OrangePortal),
            _ => None,
        })
        .collect()
}

fn read_input(input_count: usize) -> Vec<u8> {
    let mut inputs = Vec::new();

    for _ in 0..input_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        inputs.push(input.trim().parse().unwrap());
    }

    inputs
}

fn search_portal_connections(
    commands: &Vec<Command>,
) -> Result<HashMap<usize, usize>, &'static str> {
    let portals: Vec<(usize, &Command)> = commands
        .iter()
        .enumerate()
        .filter(|c| matches!(c.1, Command::OrangePortal) || matches!(c.1, Command::BluePortal))
        .collect();

    let mut portal_connections = HashMap::new();
    let mut pos_stack = VecDeque::new();
    for p in portals {
        match p.1 {
            Command::BluePortal => pos_stack.push_back(p.0),
            Command::OrangePortal => {
                let Some(connection) = pos_stack.pop_back() else {
                    return Err("SYNTAX ERROR");
                };

                portal_connections.insert(connection, p.0);
                portal_connections.insert(p.0, connection);
            }
            _ => panic!("Invalid command"),
        }
    }

    if !pos_stack.is_empty() {
        return Err("SYNTAX ERROR");
    }

    Ok(portal_connections)
}

#[derive(Default)]
struct Program {
    command_pointer: usize,
    commands: Vec<Command>,
    portal_connections: HashMap<usize, usize>,
    inputs: VecDeque<u8>,
    register: Vec<u8>,
    register_pointer: usize,
}

impl Program {
    fn run(&mut self) -> Result<(), &'static str> {
        while self.command_pointer < self.commands.len() {
            let command = self.commands[self.command_pointer].clone();

            self.perform_command(command)?;

            self.command_pointer += 1;
        }

        Ok(())
    }

    fn perform_command(&mut self, command: Command) -> Result<(), &'static str> {
        match command {
            Command::IncrementPointer => self.increase_register_pointer(),
            Command::DecrementPointer => self.decrease_register_pointer(),
            Command::IncrementValue => self.increase_value(),
            Command::DecrementValue => self.decrease_value(),
            Command::OutputValue => Ok(self.output_current_register()),
            Command::ReadInput => self.add_input_to_value(),
            Command::BluePortal => Ok(self.use_blue_portal()),
            Command::OrangePortal => Ok(self.use_orange_portal()),
        }
    }

    fn increase_value(&mut self) -> Result<(), &'static str> {
        self.update_value(|x| x.checked_add(1))
    }

    fn decrease_value(&mut self) -> Result<(), &'static str> {
        self.update_value(|x| x.checked_sub(1))
    }

    fn add_input_to_value(&mut self) -> Result<(), &'static str> {
        let input = self.inputs.pop_front().unwrap();

        self.update_value(|x| x.checked_add(input))
    }

    fn update_value(&mut self, logic: impl FnOnce(&u8) -> Option<u8>) -> Result<(), &'static str> {
        let value = &mut self.register[self.register_pointer];

        let Some(new_value) = logic(value) else {
            return Err("INCORRECT VALUE");
        };

        *value = new_value;

        Ok(())
    }

    fn increase_register_pointer(&mut self) -> Result<(), &'static str> {
        self.update_register_pointer(|x| x.checked_add(1))
    }

    fn decrease_register_pointer(&mut self) -> Result<(), &'static str> {
        self.update_register_pointer(|x| x.checked_sub(1))
    }

    fn update_register_pointer(
        &mut self,
        logic: impl FnOnce(usize) -> Option<usize>,
    ) -> Result<(), &'static str> {
        self.register_pointer = match logic(self.register_pointer) {
            Some(value) if value < self.register.len() => value,
            _ => return Err("POINTER OUT OF BOUNDS"),
        };

        Ok(())
    }

    fn output_current_register(&self) {
        print!("{}", self.register[self.register_pointer] as char)
    }

    fn use_blue_portal(&mut self) {
        if self.register[self.register_pointer] != 0 {
            return;
        }

        self.command_pointer = self.portal_connections[&self.command_pointer];
    }

    fn use_orange_portal(&mut self) {
        if self.register[self.register_pointer] == 0 {
            return;
        }

        self.command_pointer = self.portal_connections[&self.command_pointer];
    }
}

#[derive(Clone, Debug)]
enum Command {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    OutputValue,
    ReadInput,
    BluePortal,
    OrangePortal,
}
