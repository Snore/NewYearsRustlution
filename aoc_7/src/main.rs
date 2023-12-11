use std::cmp::Ordering;
use std::fs;
use std::env;

// do i need this?
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CardLabel {
    Unset = 0,
    C2,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Unset = 0,
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

struct Hand {
    cards: Vec<CardLabel>,
    hand_type: HandType
}

impl Hand {
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

        assert!( found_cards.len() == 5, "ERROR: Expected 5 cards per hand." );
        Hand{ cards : (found_cards), hand_type : (HandType::Unset) }
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
   let hands: Vec<(_, _)> = hand_raw.split('\n')
                                    .map(|l| {
                                        let (hand_chrs, bid_chars) = l.split_once(' ').unwrap();
                                        (Hand::parse(hand_chrs), bid_chars.parse::<u32>().expect("ERROR: pasring bid"))
                                        })
                                    .collect();

    // then sort 'hands'
   //  hands.sort()
    // then use each hand's index in 'hands' as multiplier with bid
}
