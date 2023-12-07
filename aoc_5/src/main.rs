use core::fmt;
use std::fs;
use std::env;
use std::collections::HashMap;
use std::time::Instant;

struct AlmanacRange {
   destination_start: u32,
   source_start: u32,
   range_len: u32
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

#[derive(Debug)]
enum AlmanacError {
   RangeError
}

impl fmt::Display for AlmanacError {
   fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
      match self {
          AlmanacError::RangeError => write!(f, "An almanac range error has occured!"),
      }
   }
}

impl std::error::Error for AlmanacError {}

impl AlmanacRange {
   pub fn parse( input: &str ) -> AlmanacRange {
      let parsed_values: Vec<u32> = input.split(' ')
                                           .map(|chunk| chunk.parse()
                                                                   .expect("Error: while parsing for range"))
                                           .collect();
      assert!( parsed_values.len() == 3, "ERROR: mismatch on expected range: {parsed_values:?}");
      AlmanacRange { destination_start: ( parsed_values[0] ), 
                     source_start: ( parsed_values[1] ), 
                     range_len: ( parsed_values[2] ) }
   }

   pub fn in_range( &self, input: &u32 ) -> bool {
      let big_input: u64 = *input as u64;
      let big_range_end: u64 = self.source_start as u64 + self.range_len as u64;
      return ( input >= &self.source_start ) && 
             ( big_input <  big_range_end );
   }

   pub fn translate( &self, input: &u32 ) -> Result<u32, AlmanacError> {
      if self.in_range(input) {
         let offset = input - self.source_start;
         return Ok( self.destination_start + offset );
      }

      return Err(AlmanacError::RangeError{});
   }
}

impl AlmanacMapping {
   pub fn parse( input: &str ) -> AlmanacMapping {
      let parsed_maps: Vec<AlmanacRange> = input.split('\n')
                                                .map(|pr| AlmanacRange::parse(pr))
                                                .collect();
      // TODO: check for range overlap?
      AlmanacMapping { range_maps: ( parsed_maps ) }
   }

   pub fn translate( &self, input: &u32 ) -> u32 {
      for range in &self.range_maps {
         if range.in_range(input) {
            return range.translate(input).unwrap();
         }
      }

      // if none of the range maps worked for the input, return the input as is
      *input
   }
}

impl Almanac {
   pub fn parse( input: &str ) -> Almanac {
      let sections_raw: Vec<(&str, &str)> = input.split("\n\n")
                                                 .map(|s| s.split_once('\n').expect("ERROR: Parsing mappings."))
                                                 .collect();
      let mut sections_map: HashMap<&str, AlmanacMapping> = HashMap::new();
      for (section, body) in sections_raw {
         sections_map.insert(section, AlmanacMapping::parse(body)); // returns None if first entry
      }

      Almanac { seed_2_soil: ( sections_map.remove("seed-to-soil map:").unwrap() ), 
                soil_2_fertilizer: ( sections_map.remove("soil-to-fertilizer map:").unwrap() ), 
                fertilizer_2_water: ( sections_map.remove("fertilizer-to-water map:").unwrap() ), 
                water_2_light: ( sections_map.remove("water-to-light map:").unwrap() ), 
                light_2_temp: ( sections_map.remove("light-to-temperature map:").unwrap() ), 
                temp_2_humid: ( sections_map.remove("temperature-to-humidity map:").unwrap() ), 
                humid_2_loc: ( sections_map.remove("humidity-to-location map:").unwrap() ) }
   }

   pub fn seed_to_location( &self, seed: &u32 ) -> u32 {
      let step1: u32 = self.seed_2_soil.translate(seed);
      let step2: u32 = self.soil_2_fertilizer.translate(&step1);
      let step3: u32 = self.fertilizer_2_water.translate(&step2);
      let step4: u32 = self.water_2_light.translate(&step3);
      let step5: u32 = self.light_2_temp.translate(&step4);
      let step6: u32 = self.temp_2_humid.translate(&step5);
      let step7: u32 = self.humid_2_loc.translate(&step6);
      step7
   }
}

pub fn parse_seeds( input: &str) -> Vec<u32> {
   input.split(' ')
        .skip(1) // skip 'seeds:'
        .map(|cap| cap.parse::<u32>().expect("ERROR: Seed parse"))
        .collect()
}

pub fn parse_seeds_as_ranges( input: &str ) -> Vec<u32> {
   input.split(' ')
        .skip(1) // skip 'seeds:'
        .map(|cap: &str| cap.parse::<u32>().expect("ERROR: Seed parse"))
        .collect::<Vec<u32>>()
        .chunks(2).map(|chunk: &[u32]| {
         match chunk {
            [a, b] => (*a..*a+*b).collect::<Vec<u32>>(),
            [a] => (*a..*a+1).collect::<Vec<u32>>(),
            _ => unreachable!(),
         }}).flat_map(|v: Vec<u32>| v).collect()
}

fn main() {
    // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let almanac_raw: String = fs::read_to_string(file_path).unwrap();
   // println!("{almanac_raw}");

   // parse the input
   let seeds_and_almanac_raw: (&str, &str) = almanac_raw.split_once("\n\n").unwrap();
   let seeds: Vec<u32> = parse_seeds(seeds_and_almanac_raw.0);
   let almanac: Almanac = Almanac::parse(seeds_and_almanac_raw.1);

   let ans_1: u32 = seeds.iter()
                         .map(|s| almanac.seed_to_location(s))
                         .min().unwrap();

   println!("Seed mappings:\n---seeds---\n{seeds:?}\n---locations---\n{ans_1}");

   // parse the seeds again for part 2
   let timing_start: Instant = Instant::now();
   let seeds_from_ranges: Vec<u32> = parse_seeds_as_ranges(seeds_and_almanac_raw.0);
   let ans_2: u32 = seeds_from_ranges.iter()
                                     .map(|s| almanac.seed_to_location(s))
                                     .min().unwrap();

   let elapsed: std::time::Duration = timing_start.elapsed();
   println!("location for part 2 [{ans_2}]\nRun time for part 2: [{elapsed:?}]");
}
