use std::fs;
use std::env;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    number: u32,
    symbols: Vec<(char, usize)>,
}

impl Part {
    fn has_symbols( &self ) -> bool {
        !self.symbols.is_empty()
    }
}

struct Schematic {
    raw: String,
    row_len: usize
}

impl Schematic {
    fn new( raw: String, row_len: usize ) -> Schematic {
        Schematic { raw: (raw), row_len: (row_len) }
    }

    fn scan_for_parts( &self ) -> Vec<Part> {
        let re_digit_finder: Regex = Regex::new(r"\d+").unwrap();
        let re_symbol_finder: Regex = Regex::new(r"[^0-9.]").unwrap();
        return re_digit_finder.find_iter(self.raw.as_str()).map(|m| {
            // convert the match to a part number
            let found_number: u32 = m.as_str().parse().expect("I made oopsie");

            // find all non '.' symbols around the match
            // get the range above, below, and the character to the left and right
            let left_bound: usize = m.start() - 1;
            let right_bound: usize = m.end() + 1;

            let top_row_start: usize = left_bound - self.row_len;
            let top_row_str: &str = &self.raw[top_row_start..(right_bound - self.row_len)];

            let bottom_row_start: usize = left_bound + self.row_len;
            let bottom_row_str: &str = &self.raw[bottom_row_start..(right_bound + self.row_len)];

            let mid_row_str: &str = &self.raw[left_bound..right_bound];

            // let parimeter: String = top_row_str.to_string() + bottom_row_str + mid_row_str;
            let found_symbols_top: Vec<(char, usize)> = re_symbol_finder.find_iter(&top_row_str).map(|sm| (sm.as_str().chars().next().unwrap(), sm.start() + top_row_start)).collect();
            let found_symbols_mid: Vec<(char, usize)> = re_symbol_finder.find_iter(&mid_row_str).map(|sm| (sm.as_str().chars().next().unwrap(), sm.start() + left_bound)).collect();
            let found_symbols_bot: Vec<(char, usize)> = re_symbol_finder.find_iter(&bottom_row_str).map(|sm| (sm.as_str().chars().next().unwrap(), sm.start() + bottom_row_start)).collect();

            // println!("Matched [{found_number}]\n[{top_row_str}]\n[{mid_row_str}]\n[{bottom_row_str}]");

            let found_symbols: Vec<(char, usize)> = found_symbols_top.into_iter().chain(found_symbols_mid).into_iter().chain(found_symbols_bot).collect();

            // make the Part
            Part{ number: (found_number), symbols: (found_symbols)  }
        }).collect::<Vec<Part>>();
    }
}

fn add_padding( orignal: &str ) -> Schematic {
    // find line length, then make padded version of schematic
    const SCHEMATIC_PADDING: usize = 2;
    let schematic_row_len: usize = orignal.find("\n").unwrap() + SCHEMATIC_PADDING;

    // regenerate the schematic with padding
    let mut padded: String = String::new();
    padded.push_str(&".".repeat(schematic_row_len));

    for line in orignal.split("\n") {
        padded.push('.');
        padded.push_str(&line);
        padded.push('.');
    }
    padded.push_str(&".".repeat(schematic_row_len));

    Schematic::new( padded, schematic_row_len )
}

fn main() {
    // get input
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    // read
    let schematic_raw: String = fs::read_to_string(file_path).unwrap();
    println!("{schematic_raw}");

    let schematic: Schematic = add_padding(&schematic_raw);

    // scan for part number, then find all adjacnt symbols.
    let parts: Vec<Part> = schematic.scan_for_parts();

    let ans_1: u32 = parts.iter()
                          .filter(|p| p.has_symbols())
                          .fold(0, |acc, pn| acc + pn.number);

    // println!("{parts:?}");

    println!("The sum of all part numbers is [{ans_1}]");

    // make the hash map of all of the gears and populate it
    let mut gear_map: HashMap<usize, Vec<u32>> = HashMap::new();

    for part in parts {
        for g_symbol in part.symbols.iter().filter(|s| s.0 == '*') {
            gear_map.entry(g_symbol.1).or_insert(Vec::new()).push(part.number);
        }
    }

    // println!("{gear_map:?}");

    // use the hash map to find the answer
    let ans_2: u32 = gear_map.iter()
                             .filter(|gear| gear.1.len() == 2)
                             .map(|gear| gear.1[0] * gear.1[1])
                             .fold(0, |acc, sum| acc + sum );
    println!("The sum of the gear ratios is [{ans_2}]");
}
