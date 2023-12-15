use std::fs;
use std::env;
use std::fmt;

#[derive(Debug)]
struct EcoReading {
   readings: Vec<i32>
}

impl fmt::Display for EcoReading {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "[ ")?;
      for reading in &self.readings {
         write!(f, "{} ", reading)?;
      }
      write!(f, "]")
   }
}

impl EcoReading {
   pub fn parse( input: &str ) -> EcoReading {
      let parsed_readings: Vec<i32> = input.split_ascii_whitespace()
                                           .map(|rr| rr.parse().expect("ERROR: Parsing reading."))
                                           .collect();
      EcoReading{ readings: (parsed_readings) }
   }

   pub fn extrapolate_next_reading( &self ) -> i32 {
      Self::extrapolate_recurse(&self.readings) + self.readings.last().unwrap()
   }

   fn extrapolate_recurse( readings: &Vec<i32> ) -> i32 {
      let mut sub_readings: Vec<i32> = Vec::with_capacity(readings.len());
      for delta_pair in readings.windows(2) {
         sub_readings.push(delta_pair[1] - delta_pair[0]);
      }

      if sub_readings.iter().all(|sr| *sr == 0 ) {
         return 0;
      } else {
         let last_reading: i32 = *sub_readings.last().unwrap();
         return last_reading + Self::extrapolate_recurse(&sub_readings);
      }
   }
}

fn main() {
   // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let ecological_readings_raw: String = fs::read_to_string(file_path).unwrap();
   println!("{ecological_readings_raw}");

   // parse readings
   let eco_readings: Vec<EcoReading> = ecological_readings_raw.split('\n')
                                                              .map( |rr| EcoReading::parse(rr) )
                                                              .collect();

   for read in &eco_readings {
      println!("Exo for {read} = {}", read.extrapolate_next_reading());
   }
   // extrapolate the readings and sum the answer
   let ans_1: i32 = eco_readings.iter()
                                .fold(0, |acc, e| acc + e.extrapolate_next_reading());
   println!("The sum of the extrapolated readings is [{ans_1}]");
}
