// use core::fmt;
use std::fmt;
use std::cmp::Ordering;
use std::fs;
use std::env;

// do i need this?
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum CardLabel {
   //  Unset = 0,
    J = 0, // comment in for part 2
    C2 = 1,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
   //  J, // comment in for part 1
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

impl fmt::Display for CardLabel {
   fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
      match *self {
          CardLabel::C2 => write!(f, "2"),
          CardLabel::C3 => write!(f, "3"),
          CardLabel::C4 => write!(f, "4"),
          CardLabel::C5 => write!(f, "5"),
          CardLabel::C6 => write!(f, "6"),
          CardLabel::C7 => write!(f, "7"),
          CardLabel::C8 => write!(f, "8"),
          CardLabel::C9 => write!(f, "9"),
          CardLabel::T => write!(f, "T"),
          CardLabel::J => write!(f, "J"),
          CardLabel::Q => write!(f, "Q"),
          CardLabel::K => write!(f, "K"),
          CardLabel::A => write!(f, "A"),
      }
   }
}

impl fmt::Display for HandType {
   fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
      match *self {
         HandType::HighCard => write!(f, "High Card"),
         HandType::OnePair => write!(f, "Pair"),
         HandType::TwoPair => write!(f, "Two Pair"),
         HandType::ThreeKind => write!(f, "Three of a Kind"),
         HandType::FullHouse => write!(f, "Full House"),
         HandType::FourKind => write!(f, "Four of a Kind"),
         HandType::FiveKind => write!(f, "Five of a Kind"),
      }
   }
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

        // count the number of duplicate cards in the hand
        let dup_counts: Vec<u8> = Self::count_dups(hand);

        if dup_counts[0] == 5 {
            HandType::FiveKind
        } else if dup_counts[0] == 4 {
            HandType::FourKind
        } else if dup_counts[0] == 3 {
            if dup_counts[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeKind
            }
        } else if dup_counts[0] == 2 {
            if dup_counts[1] == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }

    /// Assumes `input` is a sorted list of five cards such that
    /// any duplicates are right next to each other
    fn count_dups( input: Vec<CardLabel> ) -> Vec<u8> {
        let mut count_vec: Vec<u8> = Vec::with_capacity(Self::HAND_SIZE);
        let mut count_count: u8 = 1;
        for window in input.windows(2) {
         if window[0] == window[1] {
            count_count += 1;
         } else {
            count_vec.push(count_count);
            count_count = 1;
         }
        }
      
        count_vec.push(count_count); // don't forget the last card!

        // put the groupings up front so we can decide more easily later
        count_vec.sort_unstable_by(|a: &u8, b: &u8| b.cmp(a));

        // count the joker card and add their total to the first slot assuming first slot isn't jokers...
        let joker_count: u8 = input.iter().filter(|c| c == &&CardLabel::J).count() as u8;
        if joker_count != count_vec[0] {
         // add the joker count to the winner so we bolster their numbers
         count_vec[0] += joker_count;
        } else if count_vec.len() > 1 {
         // then add the joker count to the runner up and swap positions
         count_vec[1] += joker_count;
         count_vec.swap(0, 1);
        } else {
         // don't add the joker count to anything, we have 5 jokers
        }
        count_vec
    }
}

impl fmt::Display for Hand {
   fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
      let cards_as_strings: Vec<String> = self.cards.iter()
                                                    .map(|card| format!("{}", card))
                                                    .collect();
      write!(f, "[{}] -> {}", cards_as_strings.join(""), self.hand_type)
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
   // println!("{hand_raw}");

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
   for hand in &hands {
      println!("{} | Bid: {}", hand.0, hand.1);
   }
   let ans_1: u32 = hands.iter()
                         .enumerate()
                         .fold(0, |acc, (h_rank, (_h_hand, h_bid))| acc + (h_bid * (h_rank + 1) as u32)); // +1 on the rank since enumerate starts at 0
   println!("The total winnings for part 2 [{ans_1}]");
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_high_card() {
      let hand = Hand::parse("KQ4T2");
      assert_eq!(hand.hand_type, HandType::HighCard);
   }


   #[test]
   fn test_5_kind_all_joker() {
      let hand = Hand::parse("JJJJJ");
      assert_eq!(hand.hand_type, HandType::FiveKind);
   }

   #[test]
   fn test_joker() {
      let hand = Hand::parse("KTJJT");
      assert_eq!(hand.hand_type, HandType::FourKind);

      let hand = Hand::parse("JQJJJ");
      assert_eq!(hand.hand_type, HandType::FiveKind);

      let hand = Hand::parse("JQJQJ");
      assert_eq!(hand.hand_type, HandType::FiveKind);

      let hand = Hand::parse("J2JQJ");
      assert_eq!(hand.hand_type, HandType::FourKind);

      let hand = Hand::parse("J2KQ3");
      assert_eq!(hand.hand_type, HandType::OnePair);
   }

   #[test]
   fn test_full_house_0_joker() {
      let hand = Hand::parse("KKQKQ");
      assert_eq!(hand.hand_type, HandType::FullHouse);
   }

   #[test]
   fn test_ranking_joker_low() {
      let hand_1 = Hand::parse("J3456"); // make this low to pass
      let hand_2 = Hand::parse("33456");
      assert_eq!(hand_1.cmp(&hand_2), Ordering::Less);
   }
}
