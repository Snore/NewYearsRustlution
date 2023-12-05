use std::fs;
use std::env;
use std::collections::HashMap;

struct LottoTicket{
   winning_numbers: Vec<u32>,
   numbers: Vec<u32>
}

impl LottoTicket {
   pub fn parse( input: &str ) -> LottoTicket {
      let name_body: (&str, &str) = input.split_once(':').unwrap();
      let wn_n: (&str, &str) = name_body.1.split_once('|').unwrap();

      let winning_numbers: Vec<u32> = wn_n.0.split_ascii_whitespace()
                                            .map(|ds| ds.parse().expect("ERROR: WN"))
                                            .collect();
      let other_numbers: Vec<u32> = wn_n.1.split_ascii_whitespace()
                                          .map(|ds| ds.parse().expect("ERROR: Numbers"))
                                          .collect();
      LottoTicket { winning_numbers: (winning_numbers), numbers: (other_numbers) }
   }

   pub fn count_winning_numbers( &self ) -> u32 {
      let mut counts: HashMap<u32, i32> = HashMap::new();
      for num in self.winning_numbers.iter() {
         *counts.entry(*num).or_insert(0) += 1;
      }

      let mut intersection: Vec<u32> = Vec::new();
      for num in self.numbers.iter() {
         if let Some(count) = counts.get_mut(&num) {
            if *count > 0 {
               intersection.push(*num);
               *count -= 1;
            }
         }
      }

      intersection.len() as u32
   }
}

fn calc_score( points: u32 ) -> u32 {
   // 0 1 2 4 8 16 ...
   if points == 0 {
      0
   } else {
      1 << (points - 1)
   }
}

fn main() {
   // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let tickets_raw: String = fs::read_to_string(file_path).unwrap();
   println!("{tickets_raw}");

   // load tickets
   let my_tickets: Vec<LottoTicket> = tickets_raw.split('\n')
                                                 .map(|raw_ticket| LottoTicket::parse(raw_ticket))
                                                 .collect();

   // count points
   let ans_1: u32 = my_tickets.iter()
                              .map(|lt| calc_score(lt.count_winning_numbers())) // TODO Bug
                              .fold(0, |acc, tot| acc + tot);

   println!("Total points [{ans_1}]");

   // make vec of card quantities that mirror the vec of cards
   let mut quantities: Vec<u64> = Vec::new();
   quantities.resize(my_tickets.len(), 1);

   // iter through the vec of cards, count winnings(Y), add 1 * (X qaunt[ind]) to the next Y indecies in the quntitiy Vec
   for (idx, ticket) in my_tickets.iter().enumerate() {
      let win_count_on_ticket: u32 = ticket.count_winning_numbers();

      let cascade_start_idx: usize = idx + 1;
      let cascade_end_idx: usize = std::cmp::min( cascade_start_idx + win_count_on_ticket as usize, 
                                                  quantities.len());
      for sub_idx in cascade_start_idx..cascade_end_idx {
         quantities[sub_idx] += quantities[idx];
      }
   }

   // println!("{quantities:?}");

   // fold sum of quantity vec
   let ans_2: u64 = quantities.iter().fold(0, |acc, l| acc + l);
   println!("The total number of tickets made is [{ans_2}]");
}
