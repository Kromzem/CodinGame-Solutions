use std::{
    collections::{HashMap, HashSet},
    io,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Score points by scanning valuable fish faster than your opponent.
 **/
fn main() {
    let mut creatures: HashMap<i32, Creature> = HashMap::new();
    let mut drones: HashMap<i32, Drone> = HashMap::new();
    let mut scanned: HashSet<i32> = HashSet::new();
    let mut visible: HashSet<i32> = HashSet::new();
    let mut radar_count: HashMap<String, i32> = HashMap::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let creature_count = parse_input!(input_line, i32);
    for i in 0..creature_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let creature_id = parse_input!(inputs[0], i32);
        let color = parse_input!(inputs[1], i32);
        let _type = parse_input!(inputs[2], i32);

        creatures.insert(
            creature_id,
            Creature {
                position: Position { x: 0, y: 0 },
                color,
                _type,
            },
        );
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_score = parse_input!(input_line, i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let foe_score = parse_input!(input_line, i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_scan_count = parse_input!(input_line, i32);
        for i in 0..my_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let creature_id = parse_input!(input_line, i32);

            scanned.insert(creature_id);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let foe_scan_count = parse_input!(input_line, i32);
        for i in 0..foe_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let creature_id = parse_input!(input_line, i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_drone_count = parse_input!(input_line, i32);
        for i in 0..my_drone_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let drone_x = parse_input!(inputs[1], i32);
            let drone_y = parse_input!(inputs[2], i32);
            let emergency = parse_input!(inputs[3], i32);
            let battery = parse_input!(inputs[4], i32);

            let entry = drones.entry(drone_id).or_insert(Drone::default());
            entry.battery = battery;
            entry.position.x = drone_x;
            entry.position.y = drone_y;
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let foe_drone_count = parse_input!(input_line, i32);
        for i in 0..foe_drone_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let drone_x = parse_input!(inputs[1], i32);
            let drone_y = parse_input!(inputs[2], i32);
            let emergency = parse_input!(inputs[3], i32);
            let battery = parse_input!(inputs[4], i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let drone_scan_count = parse_input!(input_line, i32);
        for i in 0..drone_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let creature_id = parse_input!(inputs[1], i32);

            if !drones.contains_key(&drone_id) {
                continue;
            }

            scanned.insert(creature_id);
        }

        visible.clear();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_creature_count = parse_input!(input_line, i32);
        for i in 0..visible_creature_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let creature_id = parse_input!(inputs[0], i32);
            let creature_x = parse_input!(inputs[1], i32);
            let creature_y = parse_input!(inputs[2], i32);
            let creature_vx = parse_input!(inputs[3], i32);
            let creature_vy = parse_input!(inputs[4], i32);

            creatures.entry(creature_id).and_modify(|c| {
                c.position.x = creature_x;
                c.position.y = creature_y;
            });
            visible.insert(creature_id);
        }

        radar_count.clear();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let radar_blip_count = parse_input!(input_line, i32);
        for i in 0..radar_blip_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let creature_id = parse_input!(inputs[1], i32);
            let radar = inputs[2].trim().to_string();

            if scanned.contains(&creature_id) || visible.contains(&creature_id) {
                continue;
            }

            let count = radar_count.entry(radar).or_insert(0);
            *count += 1;
        }
        for drone in drones.values() {
            let creature = creatures
                .iter()
                .filter(|p| !scanned.contains(p.0) && visible.contains(p.0))
                .min_by(|p1, p2| {
                    p1.1.position
                        .dst(&drone.position)
                        .cmp(&p2.1.position.dst(&drone.position))
                });

            if let Some(creature) = creature {
                println!("MOVE {} {} 0", creature.1.position.x, creature.1.position.y);
                continue;
            }

            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");

            let radar = radar_count.iter().max_by_key(|e| e.1);
            if let Some((radar, _)) = radar {
                let pos = match radar.as_str() {
                    "TR" => (drone.position.x + 1000, drone.position.y - 1000),
                    "TL" => (drone.position.x - 1000, drone.position.y - 1000),
                    "BR" => (drone.position.x + 1000, drone.position.y + 1000),
                    "BL" => (drone.position.x - 1000, drone.position.y + 1000),
                    _ => unreachable!(),
                };

                println!("MOVE {} {} 1", pos.0, pos.1);
                continue;
            }
            // println!("WAIT 0"); // MOVE <x> <y> <light (1|0)> | WAIT <light (1|0)>
            println!("MOVE {} 0 0", drone.position.x);
        }
    }
}

#[derive(Default)]
struct Drone {
    position: Position,
    battery: i32,
}

#[derive(Default)]
struct Creature {
    position: Position,
    color: i32,
    _type: i32,
}

#[derive(Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn dst(&self, other: &Position) -> i32 {
        let (x1, y1, x2, y2) = (self.x as f32, self.y as f32, other.x as f32, other.y as f32);

        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() as i32
    }
}
