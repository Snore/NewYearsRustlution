use std::fs;
use std::env;
use std::collections::HashMap;

#[derive(Debug)]

enum Turn {
    Right,
    Left
}

#[derive(Debug)]

struct Directions {
    sequence: Vec<Turn>
}

impl Directions {
    pub fn parse( input: &str ) -> Directions {
        let parsed_turns: Vec<Turn> = input.chars()
                                           .map(|c| match c {
                                                'L' => Turn::Left,
                                                'R' => Turn::Right,
                                                _ => unreachable!(),
                                            }).collect();
        Directions{ sequence: (parsed_turns) }
    }
}

#[derive(Debug)]

struct Junction {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    paths: HashMap<String, Junction>
}

impl Map {
    pub fn parse( input: &str ) -> Map {
        let mut path_map : HashMap<String, Junction> = HashMap::new();
        for line in input.split('\n') {
            let (junction_name, junction) = Self::parse_line(line);
            path_map.insert(junction_name, junction);
        }
        Map { paths: (path_map) }
    }

    /// Parses a line from the map section of the input
    /// 
    /// Expects the input to be in "AAA = (BBB, CCC)" form
    fn parse_line( input: &str ) -> (String, Junction) {
        let (name_raw, junction_raw) = input.split_once(" = ").unwrap();
        let (left_raw, right_raw) = junction_raw[1..junction_raw.len()-1].split_once(", ").unwrap();
        (name_raw.to_string(), Junction{ left: (left_raw.to_string()), right: (right_raw.to_string())})
    }
}

fn main() {
    // get input
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    // read
    let map_directions_raw: String = fs::read_to_string(file_path).unwrap();
    println!("{map_directions_raw}");

    // split directions from map
    let (directions_raw, map_raw) = map_directions_raw.split_once("\n\n").unwrap();

    // parse input
    let my_directions: Directions = Directions::parse(directions_raw);
    let my_map: Map = Map::parse(map_raw);

    println!("{:?}\n{:?}", my_directions.sequence, my_map.paths);
}
