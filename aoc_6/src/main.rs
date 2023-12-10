use std::fs;
use std::env;

#[derive(Debug)]
struct RaceRecord {
    time_ms: u64,
    distance_mm: u64,
}

impl RaceRecord {
    /// Count the ways to win for this RaceRecord
    pub fn ways_to_win( &self ) -> u64 {
        let mut ways_to_win: u64 = 0;
        for test_time_ms in 0..self.time_ms {
            let remaining_time_ms: u64 = self.time_ms - test_time_ms;
            if (test_time_ms * remaining_time_ms) > self.distance_mm {
                ways_to_win += 1;
            }
        }

        return ways_to_win;
    }
}

fn parse_races( input: &str ) -> Vec<RaceRecord> {
    let mut parsed_times: Vec<u64> = Vec::new();
    let mut parsed_distances: Vec<u64> = Vec::new();

    // parse the numbers for Times and Distances into their vectors
    for line in input.split('\n') {
        let (line_name, line_numbers_raw) = line.split_once(':').unwrap();
        match line_name {
            "Time" => {
                // time
                parsed_times = line_numbers_raw.trim()
                                               .split_ascii_whitespace()
                                               .map(|s| s.parse()
                                                               .expect("ERROR: Parse time"))
                                               .collect();
            },
            "Distance" => {
                // distance
                parsed_distances = line_numbers_raw.trim()
                                                   .split_ascii_whitespace()
                                                   .map(|s| s.parse()
                                                                   .expect("ERROR: Parse distance"))
                                                   .collect();
            },
            _ => unreachable!(),
        }
    }

    // create a vector of RaceRecords by zipping the two parsed components
    let race_records: Vec<RaceRecord> = parsed_times.iter()
                                                    .zip(parsed_distances.iter())
                                                    .map(|(t, d)| RaceRecord{ time_ms: (*t), distance_mm : (*d) })
                                                    .collect();
    return race_records;
}

fn parse_super_race( input: &str ) -> RaceRecord {
    let mut parsed_time_ms: u64 = 0;
    let mut parsed_distance_mm: u64 = 0;

    // parse the numbers for Times and Distances into their vectors
    for line in input.split('\n') {
        let (line_name, line_numbers_raw) = line.split_once(':').unwrap();
        match line_name {
            "Time" => {
                // time
                parsed_time_ms = line_numbers_raw.split_ascii_whitespace()
                                                 .collect::<String>()
                                                 .parse()
                                                 .expect("ERROR: Parsing super time.");
            },
            "Distance" => {
                // distance
                parsed_distance_mm = line_numbers_raw.split_ascii_whitespace()
                                                     .collect::<String>()
                                                     .parse()
                                                     .expect("ERROR: Parsing super distance.");
            },
            _ => unreachable!(),
        }
    }

    RaceRecord { time_ms: (parsed_time_ms), distance_mm: (parsed_distance_mm) }
}

fn main() {
    // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let race_raw: String = fs::read_to_string(file_path).unwrap();
   println!("{race_raw}");

   // parse the races
   let races: Vec<RaceRecord> = parse_races( race_raw.as_str() );
   println!("{races:?}");

   // calculate the answer
   let ans_1: u64 = races.iter()
                         .fold(1, | acc, r | acc * r.ways_to_win() );
    println!("Ways to win multiplied = [{ans_1}]");

    // time for part 2
    // parse the races again for the super race
    let ans_2: u64 = parse_super_race(race_raw.as_str() ).ways_to_win();
    println!("Ways to win the super race = [{ans_2}]");
}
