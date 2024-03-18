use std::fs;
use std::env;
use itertools::Itertools;
use std::time::Instant;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Condition {
    Good,
    Damaged,
    Unkown
}

#[derive(Debug, Clone)]
struct Record {
    record: Vec<Condition>,
    damage_record: Vec<u32>,
    unknown_count: u32
}

impl Record {
    const GOOD_CHAR: char = '.';
    const DAMAGED_CHAR: char = '#';
    const UNKOWN_CHAR: char = '?';

    /// Parses a spring record from a string input
    pub fn parse( raw: &str ) -> Record {
        let ( raw_record, raw_damage_record ) = raw.split_once(' ').unwrap();

        let mut mystery_count: u32 = 0;
        let parsed_record : Vec<_> = raw_record.chars()
                                               .map( |c: char| match c {
                                                    Record::GOOD_CHAR => Condition::Good,
                                                    Record::DAMAGED_CHAR => Condition::Damaged,
                                                    Record::UNKOWN_CHAR => {
                                                        mystery_count += 1;
                                                        Condition::Unkown 
                                                    },
                                                    _ => unreachable!(),
                                                }).collect();

        let parsed_damage_record : Vec<_> = raw_damage_record.split(',')
                                                             .map(|c| c.parse::<u32>().expect("Damage record not u32.") )
                                                             .collect();
        Record{ record: ( parsed_record ), 
                damage_record: ( parsed_damage_record),
                unknown_count: ( mystery_count ) }
    }

    /// Returns a list of all valid permutations of this record that do not invalidate this record's damage_record
    pub fn get_valid_permutations( &self ) -> usize {

        // calculate number of springs need to be damaged in the unkowns
        let damage_count = self.record.iter()
                                           .filter(|c| **c == Condition::Damaged)
                                           .count();
        let required_damage_nodes = self.count_damaged_springs() as usize - damage_count;

        let all_perms = Record::get_all_permutations(self.record.len(), required_damage_nodes);

        all_perms.iter().filter_map(| permutation | {
            let temp_record = Self::apply_permutation(self.clone(), permutation);
            if temp_record.damage_record == self.damage_record {
                Some( temp_record )
            } else {
                None
            }
        }).count()
    }

    /// Applys a permutation to the passed in record and replaces all '?' values with the permutation
    fn apply_permutation( original_record: Record, permutation: &Vec<Condition>) -> Record {
        let mut new_record = original_record;

        let num_replacements = permutation.len();
        let mut current_replacement: usize = 0;
        for existing_coniditon in new_record.record.iter_mut() {
            if existing_coniditon == &Condition::Unkown {
                *existing_coniditon = permutation[current_replacement];
                current_replacement += 1;

                if num_replacements <= current_replacement {
                    break;
                }
            }
        }

        new_record.damage_record = Self::generate_damage_record(&new_record.record);
        new_record.unknown_count = 0;
        new_record
    }

    /// Returns a list of permutations for a record given the number of needed damaged springs.
    /// 
    /// All returned permutations have all known components. No `?`
    fn get_all_permutations( length: usize, required_damage: usize ) -> Vec<Vec<Condition>> {
        let good_record: Vec<Condition> = vec![Condition::Good; length];

        // generate all possible permutations of Springs given how many we need to have damaged
        (0..length).combinations(required_damage)
                   .map( |perm_mapping: Vec<usize>| {
                       let mut gen_combo: Vec<Condition> = good_record.clone();
                       for index in perm_mapping {
                           gen_combo[index] = Condition::Damaged;
                       }
                       gen_combo
                   } ).collect()
    }

    /// Generates a damage_record from a condition record by counting the sets of contiguous damaged components
    fn generate_damage_record( record: &Vec<Condition> ) -> Vec<u32> {
        assert!( !record.contains(&Condition::Unkown) ); // TODO make Result later

        let mut damage_record: Vec<u32> = Vec::new();
        let mut damage_counter: u32 = 0;
        for spring in record {
            if spring == &Condition::Damaged {
                damage_counter += 1;
            } else {
                if damage_counter > 0 {
                    damage_record.push(damage_counter);
                    damage_counter = 0;
                }
            }
        }

        // dump the last cluster if we ended on a damaged spring
        if damage_counter > 0 {
            damage_record.push(damage_counter);
        }
        
        damage_record
    }

    fn count_damaged_springs( &self ) -> u32 {
        self.damage_record.iter()
                          .fold(0, |acc, e| acc + e )
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

    // println!("{:?}", records);
    let timing_start: Instant = Instant::now();
    let ans1 = records.iter()
                           .map( |r| r.get_valid_permutations() )
                           .fold(0, |acc, total| acc + total );
    let elapsed: std::time::Duration = timing_start.elapsed();
    println!("The total number of valid spring records is {ans1} and it took {:?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_damage_record() {
        let record = Record::parse(".#.#.# 1,1,1");
        assert_eq!(Record::generate_damage_record(&record.record), vec![1,1,1]);
        let record2 = Record::parse(".#.##..###. 1,2,3");
        assert_eq!(Record::generate_damage_record(&record2.record), vec![1,2,3]);
    }

    #[test]
    fn test_get_all_permutations() {
        assert_eq!(Record::get_all_permutations(4, 1).len(), 4);
        assert_eq!(Record::get_all_permutations(4, 2).len(), 6);
        assert_eq!(Record::get_all_permutations(4, 3).len(), 4);
        assert_eq!(Record::get_all_permutations(4, 4).len(), 1);
    }
}
