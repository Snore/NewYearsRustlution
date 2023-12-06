use std::fs;
use std::env;
use std::collections::HashMap;

struct AlmanacRange {
   destination_start: usize,
   source_start: usize,
   range_len: usize
}

struct AlmanacMapping {
   range_maps: Vec<AlmanacRange>
}

struct Almanac {
   seed_2_soil: AlmanacMapping,
   soil_2_fertilizer: AlmanacMapping,
   fertilizer_2_water: AlmanacMapping,
   water_2_light: AlmanacMapping,
   light_2_temp: AlmanacMapping,
   temp_2_humid: AlmanacMapping,
   humid_2_loc: AlmanacMapping
}

impl AlmanacRange {
   pub fn parse( input: &str ) -> AlmanacRange {
      let parsed_values: Vec<usize> = input.split('\n')
                                           .map(|chunk| chunk.parse()
                                                                   .expect("Error: range"))
                                           .collect();
      assert!( parsed_values.len() == 3, "ERROR: mismatch on expected range: {parsed_values:?}");
      AlmanacRange { destination_start: ( parsed_values[0] ), 
                     source_start: ( parsed_values[1] ), 
                     range_len: ( parsed_values[2] ) }
   }
}

impl AlmanacMapping {
   pub fn parse( input: &str ) -> AlmanacMapping {
      let parsed_maps: Vec<AlmanacRange> = input.split('\n')
                                                .map(|pr| AlmanacRange::parse(pr))
                                                .collect();
      AlmanacMapping { range_maps: ( parsed_maps ) }
   }
}

impl Almanac {
   pub fn parse( input: &str ) -> Almanac {
      let sections_raw: Vec<(&str, &str)> = input.split("\n\n")
                                                 .map(|s| s.split_once('\n').expect("ERROR: Parsing mappings."))
                                                 .collect(); // TODO
      let mut sections_map: HashMap<&str, AlmanacMapping> = HashMap::new();
      for (section, body) in sections_raw {
         sections_map.insert(section, AlmanacMapping::parse(body)); // returns None if first entry
      }

      Almanac { seed_2_soil: ( sections_map.get("seed-to-soil map:") ), 
                soil_2_fertilizer: (), 
                fertilizer_2_water: (), 
                water_2_light: (), 
                light_2_temp: (), 
                temp_2_humid: (), 
                humid_2_loc: () }
   }
}

pub fn parse_seeds( input: &str) -> Vec<u32> {
   input.split(' ')
        .skip(1) // skip 'seeds:'
        .map(|cap| cap.parse::<u32>().expect("ERROR: Seed parse"))
        .collect()
}

fn main() {
    // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let almanac_raw: String = fs::read_to_string(file_path).unwrap();
   println!("{almanac_raw}");

   // parse the input
   let seeds_and_almanac_raw: (&str, &str) = almanac_raw.split_once("\n\n").unwrap();
   let seeds: Vec<u32> = parse_seeds(seeds_and_almanac_raw.0);
   let almanac: Almanac = Almanac::parse(seeds_and_almanac_raw.1);
}
