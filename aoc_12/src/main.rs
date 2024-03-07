use std::fs;
use std::env;

#[derive(Debug)]
enum Condition {
    Good,
    Damaged,
    Unkown
}

#[derive(Debug)]
struct Record {
    record: Vec<Condition>,
    damage_record: Vec<usize>
}

impl Record {
    const GOOD_CHAR: char = '.';
    const DAMAGED_CHAR: char = '#';
    const UNKOWN_CHAR: char = '?';

    pub fn parse( raw: &str ) -> Record {
        let ( raw_record, raw_damage_record ) = raw.split_once(' ').unwrap();

        let parsed_record : Vec<_> = raw_record.chars()
                                               .map( |c| match c {
                                                    Record::GOOD_CHAR => Condition::Good,
                                                    Record::DAMAGED_CHAR => Condition::Damaged,
                                                    Record::UNKOWN_CHAR => Condition::Unkown,
                                                    _ => unreachable!(),
                                                }).collect();

        let parsed_damage_record : Vec<_> = raw_damage_record.split(',')
                                                             .map(|c| c.parse::<usize>().expect("Damage record not usize.") )
                                                             .collect();
        Record{ record: ( parsed_record ), 
                damage_record: ( parsed_damage_record) }
    }

    pub fn get_valid_permutations( &self ) -> u32 {
        // TODO: Find the number of ways the record could match the damage record
        todo!();
    }

}

fn main() {
    // get input
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    // read
    let records_raw: String = fs::read_to_string(file_path).unwrap();
    println!("{records_raw}");
    println!("-----------");

    let records: Vec<_> = records_raw.split('\n')
                                     .map( |record_row| Record::parse(record_row) )
                                     .collect();

    println!("{:?}", records);

    let ans1 = records.iter()
                           .map( |r| r.get_valid_permutations() )
                           .fold(0, |acc, total| acc + total );
    println!("The total number of valid spring records is {ans1}");
}
