use std::fs;
use std::env;

#[derive(Debug)]
struct RaceRecord {
    time_ms: u32,
    distance_mm: u32,
}

// impl RaceRecord {
// }

fn parse_races( input: &str ) -> Vec<RaceRecord> {
    let mut parsed_times: Vec<u32> = Vec::new();
    let mut parsed_distances: Vec<u32> = Vec::new();

    // parse the numbers for Times and Distances into their vectors
    for line in input.split('\n') {
        let (line_name, line_numbers_raw) = line.split_once(':').unwrap();
        println!("!! {line_name} {line_numbers_raw}");
        match line_name {
            "Time" => {
                // time
                parsed_times = line_numbers_raw.trim()
                                               .split(' ')
                                               .map(|s| s.parse()
                                                         .expect("ERROR: Parse time"))
                                               .collect();
            },
            "Distance" => {
                // distance
                parsed_distances = line_numbers_raw.trim()
                                                   .split(' ')
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

fn main() {
    // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let race_raw: String = fs::read_to_string(file_path).unwrap();
   println!("{race_raw}");

   let races: Vec<RaceRecord> = parse_races( race_raw.as_str() );
   println!("{races:?}");
}
