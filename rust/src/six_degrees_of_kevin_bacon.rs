use std::{
    collections::{HashMap, HashSet},
    io,
};

type MovieMap = HashMap<String, HashSet<String>>;
type ActorMap = HashMap<String, HashSet<String>>;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * 6 Degrees of Kevin Bacon!
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let actor_name = input_line.trim_matches('\n').to_string();
    if actor_name == "Kevin Bacon" {
        println!("0");
        return;
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut movies: MovieMap = HashMap::new();
    let mut actors: ActorMap = HashMap::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let movie_cast = input_line.trim_matches('\n').to_string();

        parse_and_insert(&movie_cast, &mut movies, &mut actors);
    }

    let empty_set: HashSet<String> = HashSet::new();
    let mut iteration = 0;
    let mut visible_movies: HashSet<&str> = HashSet::new();
    visible_movies.extend(
        movies
            .iter()
            .filter(|m| m.1.contains(&actor_name))
            .map(|m| m.0.as_str()),
    );
    loop {
        iteration += 1;

        let visible_actors: HashSet<&str> = visible_movies
            .iter()
            .flat_map(|&m| {
                movies
                    .get(m)
                    .unwrap_or(&empty_set)
                    .iter()
                    .map(String::as_str)
            })
            .collect();

        if visible_actors.contains("Kevin Bacon") {
            break;
        }

        visible_movies = visible_actors
            .iter()
            .flat_map(|&a| {
                actors
                    .get(a)
                    .unwrap_or(&empty_set)
                    .iter()
                    .map(String::as_str)
            })
            .collect();
    }

    println!("{}", iteration);
}

pub fn parse_and_insert(line: &str, movies: &mut MovieMap, actors: &mut ActorMap) {
    let (movie, cast_line) = line.split_once(':').unwrap();

    let cast: HashSet<String> = cast_line.split(',').map(|s| s.trim().to_string()).collect();

    for actor in cast.iter() {
        let actor_movies = actors.entry(actor.to_string()).or_default();
        actor_movies.insert(movie.to_string());
    }

    movies.insert(movie.to_string(), cast);
}
