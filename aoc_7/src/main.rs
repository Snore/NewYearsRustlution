use std::cmp::Ordering;
use std::fs;
use std::env;

// do i need this?
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum CardLabel {
   //  Unset = 0,
    C2 = 1,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
   //  Unset = 0,
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

#[derive(Debug)]
struct Hand {
    cards: Vec<CardLabel>,
    hand_type: HandType
}

impl Hand {
   const HAND_SIZE: usize = 5;
    pub fn parse( input: &str ) -> Hand {
        let found_cards: Vec<CardLabel> = input.chars().map(|c| match c {
            '2' => CardLabel::C2,
            '3' => CardLabel::C3,
            '4' => CardLabel::C4,
            '5' => CardLabel::C5,
            '6' => CardLabel::C6,
            '7' => CardLabel::C7,
            '8' => CardLabel::C8,
            '9' => CardLabel::C9,
            'T' => CardLabel::T,
            'J' => CardLabel::J,
            'Q' => CardLabel::Q,
            'K' => CardLabel::K,
            'A' => CardLabel::A,
            _ => unreachable!(),
        }).collect();

        assert!( found_cards.len() == Self::HAND_SIZE, "ERROR: Expected 5 cards per hand." );
        let hand_type: HandType = Self::determine_hand_type( found_cards.clone() );

        Hand{ cards : (found_cards), hand_type : (hand_type) }
    }

    fn determine_hand_type( mut hand: Vec<CardLabel> ) -> HandType {
        // first sort the hand for easier hand type identification
        hand.sort();

      //   print!("[{hand:?}]->");
        let dup_counts = Self::count_dups(hand);
      //   println!("{dup_counts:?}");

        if dup_counts[0] == 5 {
         return HandType::FiveKind;
        } else if dup_counts[0] == 4 {
            return HandType::FourKind;
        } else if dup_counts[0] == 3 {
            if dup_counts[1] == 2 {
               return HandType::FullHouse;
            } else {
               return HandType::ThreeKind;
            }
        } else if dup_counts[0] == 2 {
            if dup_counts[1] == 2 {
               return HandType::TwoPair;
            } else {
               return HandType::OnePair;
            }
        } else {
         return HandType::HighCard;
        }
    }

    /// Assumes `input` is a sorted list of five cards such that
    /// any duplicates are right next to each other
    fn count_dups( input: Vec<CardLabel> ) -> Vec<u8> {
        let mut count_vec: Vec<u8> = Vec::with_capacity(Self::HAND_SIZE);
        let mut count_count = 1;
        for window in input.windows(2) {
         if window[0] == window[1] {
            count_count += 1;
         } else {
            count_vec.push(count_count);
            count_count = 1;
         }
        }
      
        count_vec.push(count_count); // don't forget the last card!

        // put the groupings up front so we can decied more easily later
        count_vec.sort_unstable_by(|a, b| b.cmp(a));
        count_vec
    }
}

impl Ord for Hand {
   fn cmp( &self, other: &Self ) -> Ordering {
      if self.hand_type > other.hand_type {
         Ordering::Greater
      }
      else if self.hand_type < other.hand_type {
         Ordering::Less
      }
      else {
         // hands are equal, need to find the high card on the left
         for (my_card_label, your_card_label) in self.cards.iter().zip( other.cards.iter() ) {
            if my_card_label > your_card_label{
               // Q: why do i need to use the return keyword here? should look it up later
               return Ordering::Greater;
            }
            else if my_card_label < your_card_label {
               return Ordering::Less;
            }
            else {
               // no -op
            }
         }

         // If we get here the hands are completely equal
         Ordering::Equal
      }
   }
}

impl PartialEq for Hand {
   fn eq(&self, other: &Self) -> bool {
      self.hand_type == other.hand_type && self.cards == other.cards
   }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       Some(self.cmp(other))
   }
}

fn main() {
    // get input
   let args: Vec<String> = env::args().collect();
   let file_path: &String = &args[1];

   // read
   let hand_raw: String = fs::read_to_string(file_path).unwrap();
   println!("{hand_raw}");

   // parse
   let mut hands: Vec<(_, _)> = hand_raw.split('\n')
                                        .map(|l| {
                                            let (hand_chrs, bid_chars) = l.split_once(' ').unwrap();
                                            (Hand::parse(hand_chrs), bid_chars.parse::<u32>().expect("ERROR: pasring bid"))
                                            })
                                        .collect();

   // sort tht hands by rank
   hands.sort_unstable_by(|(a_hand, _a_bid), (b_hand, _b_bid)| a_hand.cmp(b_hand) );
   // then use each hand's index in 'hands' as multiplier with bid
   let ans_1: u32 = hands.iter()
                         .enumerate()
                         .fold(0, |acc, (h_rank, (_h_hand, h_bid))| acc + (h_bid * (h_rank + 1) as u32)); // +1 on the rank since enumerate starts at 0
   println!("The total winnings for part 1 [{ans_1}]");
}
