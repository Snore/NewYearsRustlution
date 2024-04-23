use std::fs;
use std::env;
use std::fmt;
use itertools::Itertools;
use std::time::Instant;
use std::io::{self, Write};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Ord, PartialOrd)]
enum Condition {
    Good,
    Damaged,
    Unkown
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Good => write!(f, "."),
            Condition::Damaged => write!(f, "#"),
            Condition::Unkown => write!(f, "?")
        }
    }
}

type DamageRecord = Vec<u32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Record {
    record: Vec<Condition>,
    damage_record: DamageRecord,
    unknown_count: u32
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ record: " )?;
        for rec in &self.record {
            write!(f, "{}", rec )?;
        }
        write!(f, " ]" )?;
        write!(f, " damage: " )?;
        for dam in &self.damage_record {
            write!(f, "{},", dam )?;
        }

        write!(f, " | {}", self.unknown_count)?;

        Ok(())
    }
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

    /// Returns a copy of the Record that has been folded num_fold times
    /// 
    /// For every fold, the original record is duplicated and appended.  The record field itself is concatenated with a '?' section between folds
    pub fn fold( &self, num_fold: usize ) -> Record {
        // repeat the original damage_record num_fold times
        let folded_damage_record: Vec<_> = self.damage_record.iter()
                                                             .cloned()
                                                             .cycle()
                                                             .take(self.damage_record.len() * num_fold)
                                                             .collect();

        // repeat the original record num_fold times but concatenate each reapeat of the sequence with a '?'
        let link = &[Condition::Unkown];             // slice I wanna insert between sequences
        let record_fold_len = ( self.record.len() * num_fold ) // number of times to copy over for original list
                                     + (num_fold - 1);                // number of links to connect the sequences
        let repeated_original_record: Vec<_> = self.record.iter()
                                                          .chain( link )
                                                          .cloned()
                                                          .cycle()
                                                          .take( record_fold_len )
                                                          .collect();
        Record{ record: repeated_original_record, 
                damage_record: folded_damage_record, 
                unknown_count: self.unknown_count * num_fold as u32 + (num_fold - 1) as u32 }
    }

    /// Returns a list of all valid permutations of this record that do not invalidate this record's damage_record
    pub fn get_valid_permutations( &self ) -> usize {

        // calculate number of springs needed to be damaged in the unkowns
        let damage_count = self.record.iter()
                                           .filter(|c| **c == Condition::Damaged)
                                           .count();
        let required_damage_nodes = self.count_damaged_springs() as usize - damage_count;

        let all_perms = Record::get_all_permutations(self.unknown_count as usize, required_damage_nodes);

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

        let debug = factorial(length as u128) / (factorial(required_damage as u128)*factorial((length-required_damage) as u128));
        io::stdout().flush().unwrap();

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

    /// Sum of the values in the damage record
    fn count_damaged_springs( &self ) -> u32 {
        self.damage_record.iter()
                          .fold(0, |acc, e| acc + e )
    }

    /// Sum of the minimum number of springs needed to represent the damage record
    /// 
    /// This is a combination of damaged springs and good springs separating the damaged springs
    /// as we need at least one Good spring to separate two damaged springs.
    fn count_minumum_springs( &self ) -> usize {
        self.count_damaged_springs() as usize + (self.damage_record.len() - 1)
    }

    /// Takes a mystery record and returns the number of permutations that can result in the damage record.
    /// 
    /// A Mystery record is a record made of only '?'
    fn calc_perms_of_groups( mystery_record: &Record, memo: &mut BTreeMap<Record, u128> ) -> u128 {
        debug_assert!( mystery_record.record.iter().all( |s| *s == Condition::Unkown ), 
                       "Record is not mysterious enough." );
        
        println!("Mystery record: {mystery_record}");

        // this catches when there is only one way to have have the damage record match the mystery record
        // ex: ????? 1,1,1 -> there is only way to replace 3(?) with 3(#)
        let min_needed_symbols = mystery_record.count_minumum_springs();
        if min_needed_symbols == mystery_record.record.len() {
            return 1
        }

        // reduces the damage record to only contain 1's
        // ex. ???? 2,1 -> ??? 1,1
        let reduced_record = mystery_record.reduce_mystery_record();
        println!("Reduced record: {reduced_record}");
        let slots = reduced_record.record.len() as u128;
        let groups = reduced_record.damage_record.len() as u128;
        if groups == 2 {
            // terminating condition
            // Num val perm = ?????? 1,1 where num_val_perm=((slots choose num(#)) - (slots - (groups - 1)) ) <-  magic sauce
            // reduces to...
            return n_choose_k(slots, groups) - ( slots - 1)
        }

        // check to see if we already know the answer
        if let Some(known_perms) = memo.get(&reduced_record) {
            return *known_perms;
        }

        // remove all permutations with contiguous #s of size (groups)
        // slots - (groups - 1)
        let mut total_perms = n_choose_k(slots, groups);
        println!("Total Perms: {total_perms}");

        let num_all_touching_perms = slots - (groups - 1); // this is the all 1,1,1,...,1 calculation\
        println!("num_all_touching_perms: {num_all_touching_perms}");


        total_perms -= num_all_touching_perms;
        println!("Total Perms: {total_perms}");

        let mut known_damage_records: BTreeMap<usize, Vec<DamageRecord>> = BTreeMap::new();
        let sub_groups = Record::generate_groups(groups as usize, &mut known_damage_records);
        for sub_group in &sub_groups[1..(sub_groups.len()-1)] {
            let new_record = Record{ record: reduced_record.record.clone(), 
                damage_record: sub_group.clone(), 
                unknown_count: reduced_record.record.len() as u32 };
                println!("new_record: {new_record}");
                
                let perms_to_remove = Record::calc_perms_of_groups(&new_record, memo);
                println!("Removing {perms_to_remove} from {total_perms}");
                total_perms -= perms_to_remove;
        }

        // remember the permutations so we can skip the calculation next time
        memo.insert(reduced_record, total_perms);

        total_perms

    }

    /// Takes a mystery record and reduces all groups of damage records to size 1; then alters the record accordingly
    /// 
    /// Examples:
    /// 
    /// Damage record ???? 1,1 -> ???? 1,1 will be unchanged as there is nothing to reduce.
    /// 
    /// Damage record ???? 2,1 -> ??? 1,1 will be reduced by 1 from both the total size as well as the 2 in the original damage record
    /// as we are effectively squashing the 2 damaged elements into 1 and need to adjust the total size accordingly.
    /// 
    /// Damage record ??????? 2,1,3 -> ???? 1,1,1 will have the group of 2 squashed into 1 and the group of 3 squashed into 1
    /// and the original record size will be adjusted.
    fn reduce_mystery_record( &self ) -> Record {
        debug_assert!( self.record.iter().all( |s| *s == Condition::Unkown ), 
                       "Record is not mysterious enough." );

        // count the number of springs taken away to reduce each group to size of 1
        let reduction_count = self.damage_record.iter().fold(0, |acc, d| acc + (d-1));
        let reduced_record_size = self.record.len() - reduction_count as usize;
        let reduced_damage_record = vec![1; self.damage_record.len()];
        let reduced_record = vec![Condition::Unkown; reduced_record_size];

        Record{ record: reduced_record, 
                damage_record: reduced_damage_record, 
                unknown_count: reduced_record_size as u32 }
    }

    /// Generates all possible damage records for a pool_size where all valid groups can be summed to
    /// pool_size and have at least 2 groups in them
    fn generate_groups( pool_size: usize, lut: &mut BTreeMap<usize, Vec<DamageRecord>> ) -> Vec<DamageRecord> {
        if let Some(known_damage_records) = lut.get(&pool_size) {
            return known_damage_records.to_vec();
        }

        if pool_size == 1 {
            return vec![vec![1]];
        }
        let mut damage_record_combos: Vec<DamageRecord> = Vec::new();
        for idx in (1..=pool_size).rev() {
            // counting down, we want to work with the remainder and make damage records
            // between the outer idx which is the "held" number and the remainder
            let remainder = pool_size - idx;
            for mut subrecord in Record::generate_groups(remainder, lut) {
                subrecord.insert(0, idx as u32);
                damage_record_combos.push(subrecord);
            }

            if remainder == 0 {
                damage_record_combos.push(vec![idx as u32]);
            }
        }

        // record the results for later lookup
        lut.insert(pool_size, damage_record_combos.clone());

        damage_record_combos
        
    }

}

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

pub fn n_choose_k( n: u128, k: u128 ) -> u128 {
    factorial(n) / ( factorial(k) * factorial(n-k) )
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

    // for part 2
    let _big_records: Vec<_> = records.iter()
                                     .map(|r| r.fold(5) )
                                     .collect();

    for record in &records {
        println!("{}", record);
    }

    // TODO the '.' tells us how many contigous blocks are already done

    let timing_start: Instant = Instant::now();
    let ans1 = records.iter()
                             .map( |r| r.get_valid_permutations() )
                             .fold(0, |acc, total| acc + total );
    println!("The total number of valid spring records is {ans1} and it took {:?}", timing_start.elapsed());

    // for record in &big_records {
    //     println!("{}", record);
    // }

    // let timing_start_2 = Instant::now();
    // let ans2 = big_records.iter()
    //                              .map( |r| r.get_valid_permutations() )
    //                              .fold(0, |acc, total| acc + total );
    // println!("The total number of valid spring records for part 2 is {ans2} and it took {:?}", timing_start_2.elapsed());
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

    #[test]
    fn test_help() {
        // Given: ??????? 2,1
        // Num val perm = ?????? 1,1 where num_val_perm=((slots choose num(#)) - (slots - (groups - 1)) ) 
        // do this recursively for all groups until we only get to 1,1
        // let record = Record::parse("??????? 1,1,1,1");
        // assert_eq!(record.get_valid_permutations(), 5);
        // let record = Record::parse("?#?#?#?#?#?#?#? 1,3,1,6");
        // assert_eq!(record.get_valid_permutations(), 1);
        // let record_2 = Record::parse("??????????????? 1,3,1,6");
        // assert_eq!(record_2.get_valid_permutations(), 5);
        // let record_3 = Record::parse("??????????????? 1,3");
        // assert_eq!(record_3.get_valid_permutations(), 66);
        // let record_4 = Record::parse("??????????????? 1,1,1,1,1");
        // assert_eq!(record_4.get_valid_permutations(), 462);
        let record_5 = Record::parse("?????????????? 1,1,1,1");
        assert_eq!(record_5.get_valid_permutations(), 330);
    }

    #[test]
    fn test_reduce_mystery_record() {
        let record_1 = Record::parse("???? 1,1");
        assert_eq!(record_1.reduce_mystery_record(), record_1);

        let record_2 = Record::parse("???? 2,1");
        assert_eq!(record_2.reduce_mystery_record(), Record::parse("??? 1,1"));

        let record_3 = Record::parse("??????? 2,1,3");
        assert_eq!(record_3.reduce_mystery_record(), Record::parse("???? 1,1,1"));
    }

    #[test]
    #[should_panic]
    fn test_reduce_mystery_record_not_mysterious() {
        Record::parse("???#??? 2,1,3").reduce_mystery_record();
    }

    #[test]
    fn test_calc_perms_of_groups() {
        let mut known_record_perms: BTreeMap<Record, u128> = BTreeMap::new();

        let record_1 = Record::parse("???? 1,1");
        assert_eq!(Record::calc_perms_of_groups(&record_1, &mut known_record_perms), 3);

        let record_2 = Record::parse("????? 2,1");
        assert_eq!(Record::calc_perms_of_groups(&record_2, &mut known_record_perms), 3);

        let record_one_solution = Record::parse("????? 1,1,1");
        assert_eq!(Record::calc_perms_of_groups(&record_one_solution, &mut known_record_perms), 1);

        let record_n_plus_1 = Record::parse("????????????? 1,1,1");
        assert_eq!(Record::calc_perms_of_groups(&record_n_plus_1, &mut known_record_perms), 165);

        let record_n_plus_2 = Record::parse("?????????????? 1,1,1,1");
        assert_eq!(Record::calc_perms_of_groups(&record_n_plus_2, &mut known_record_perms), 330);

        let record_n_plus_3 = Record::parse("??????????????? 1,1,1,1,1");
        assert_eq!(Record::calc_perms_of_groups(&record_n_plus_3, &mut known_record_perms), 462);

        let record_n_plus_4 = Record::parse("???????????????????????? 1,1,1,1,1");
        assert_eq!(Record::calc_perms_of_groups(&record_n_plus_4, &mut known_record_perms), 15504); // did not verify by hand
    }
}
