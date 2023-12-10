use std::fs;
use std::env;

fn main() {
    // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let hand_raw: String = fs::read_to_string(file_path).unwrap();
   println!("{hand_raw}");
}
