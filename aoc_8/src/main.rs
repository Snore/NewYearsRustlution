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
    sequence: Vec<Turn>,
    counter: usize
}

impl Directions {
    pub fn parse( input: &str ) -> Directions {
        let parsed_turns: Vec<Turn> = input.chars()
                                           .map(|c| match c {
                                                'L' => Turn::Left,
                                                'R' => Turn::Right,
                                                _ => unreachable!(),
                                            }).collect();
        assert!(!parsed_turns.is_empty());
        Directions{ sequence: (parsed_turns), counter : (0) }
    }

    /// Returns the next move the system should do as perscribed by the sequence
    /// 
    /// This will loop indefinitely
    pub fn get_next_step( &mut self ) -> &Turn {
        let next_move: usize = self.counter;
        self.counter = (self.counter + 1) % self.sequence.len();
        &self.sequence[next_move]
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

    /// Returns the destination given the starting junction and the direction to go
    pub fn get_destination( &self, current_loc: &String, direction: &Turn ) -> String {
        let junction = self.paths.get(current_loc).unwrap();
        match direction {
            Turn::Left => junction.left.clone(),
            Turn::Right => junction.right.clone(),
        }
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
    let mut my_directions: Directions = Directions::parse(directions_raw);
    let my_map: Map = Map::parse(map_raw);

   //  println!("{:?}\n{:?}", my_directions.sequence, my_map.paths);

    // solve part 1
    let target_location: String = "ZZZ".to_string();
    let mut current_location: String = "AAA".to_string();
    let mut steps: u32 = 0;
    while current_location != target_location {
       current_location = my_map.get_destination(&current_location, my_directions.get_next_step());
       steps += 1;
    }

    println!("Number of moves needed is [{steps}]");
}
