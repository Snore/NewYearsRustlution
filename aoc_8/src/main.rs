use std::fs;
use std::env;
use std::collections::HashMap;
use std::time::Instant;

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

    pub fn reset( &mut self ) {
        self.counter = 0;
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
    pub fn get_destination( &self, current_loc: &String, direction: &Turn ) -> Option<String> {
        let junction = self.paths.get(current_loc)?;
        match direction {
            Turn::Left => Some(junction.left.clone()),
            Turn::Right => Some(junction.right.clone()),
        }
    }

    pub fn get_all_nodes( &self ) -> Vec<String> {
        self.paths.keys().map(|n| n.clone()).collect()
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

    // solve part 1
    let target_location: String = "ZZZ".to_string();
    let mut current_location: String = "AAA".to_string();
    let mut steps: u64 = 0;
    while current_location != target_location {
        let destination = my_map.get_destination(&current_location, my_directions.get_next_step());
        match destination {
            Some(destination) => {
                current_location = destination;
                steps += 1;
            },
            None => {
                steps = 0;
                break; // unsolvable. just break out
            },
        }
    }

    println!("Number of moves needed for part 1 is [{steps}]");

    // solve part 2
    steps = 0;
    my_directions.reset();
    let timing_start: Instant = Instant::now();
    let mut current_locations: Vec<String> = my_map.get_all_nodes()
                                                   .into_iter()
                                                   .filter(|n| n.ends_with('A'))
                                                   .collect();
    while !current_locations.iter()
                            .all(|l| l.ends_with('Z')) {
        // note the turn to take so that all nodes can use the same direction in the same step
        let next_turn: &Turn = my_directions.get_next_step();
        current_locations.iter_mut()
                         .for_each(|l| *l = my_map.get_destination(l, next_turn).unwrap());
        steps += 1;
    }

    // Idea:
    // this takes too long.  maybe i can identify cycles and once i establish the cycles of all starting nodes
    // I can just find the cycle they all share by multiplying them all together.

    // run single runner two steps
    // compare first half of array and back half
    // repeat until above is satisfied
    // cut recording in half, don't need the duplicate.
    // find the indecies that end in 'Z'
    // ???

    let elapsed: std::time::Duration = timing_start.elapsed();
    println!("Number of ghost moves needed for part 2 is [{steps}] spooky steps.\nThis took {elapsed:?} to complete.");
}
