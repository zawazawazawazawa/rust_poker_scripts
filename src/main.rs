use itertools::Itertools;
use rand::{seq::IteratorRandom, thread_rng};
use std::fmt;

fn main() {
    let ranks: [String; 13] = [
        String::from("A"),
        String::from("K"),
        String::from("Q"),
        String::from("J"),
        String::from("10"),
        String::from("9"),
        String::from("8"),
        String::from("7"),
        String::from("6"),
        String::from("5"),
        String::from("4"),
        String::from("3"),
        String::from("2")
    ];

    let suits: [String; 4] = [
        String::from("spade"),
        String::from("heart"),
        String::from("diamond"),
        String::from("club")
    ];

    
    #[derive(Copy, Clone, Debug)]
    struct Card<'a> {
        rank: &'a String,
        suit: &'a String,
        sort_key: i32
    }

    enum Hand {
        RoyalFlush,
        StraightFlush,
        FourOfAKind,
        FullHouse,
        Flush,
        Straight,
        ThreeOfAKind,
        TwoPair, // pairsよりpairがメジャーらしい https://english.stackexchange.com/questions/389000/why-is-the-poker-hand-called-two-pair-and-not-two-pairs
        OnePair,
        HighCard
    }

    impl fmt::Display for Hand {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Hand::RoyalFlush => write!(f, "Royal flush"),
                Hand::StraightFlush => write!(f, "String flush"),
                Hand::FourOfAKind => write!(f, "Four of a kind"),
                Hand::FullHouse => write!(f, "Full house"),
                Hand::Flush => write!(f, "Flush"),
                Hand::Straight => write!(f, "Straight"),
                Hand::ThreeOfAKind => write!(f, "Three of a kind"),
                Hand::TwoPair => write!(f, "Two pair"),
                Hand::OnePair => write!(f, "One pair"),
                Hand::HighCard => write!(f, "HighCard")
            }
        }
    }

    
    let mut decks = Vec::<Card>::new();


    for i in 0..ranks.len() {
        for j in 0..suits.len() {
            decks.push(
                Card {
                    rank: &ranks[i],
                    suit: &suits[j],
                    sort_key: match ranks[i].as_str() {
                        "A" => {
                            14
                        }
                        "K" => {
                            13
                        }
                        "Q" => {
                            12
                        }
                        "J" => {
                            11
                        }
                        _ => {
                            ranks[i].as_str().parse::<i32>().unwrap()
                        }
                    }
                }
            );
        }
    };

    let mut rng = thread_rng();
    let mut hand: Option<Hand> = None;

    'outer: loop {
        let cards = decks.iter().choose_multiple(&mut rng, 7);

        let suit_groups = cards.clone().into_iter().into_group_map_by(|x| x.suit);

        for group in suit_groups.iter() {
            if group.1.len() >= 5 {
                let mut flush_cards = group.1.clone();
                flush_cards.sort_by(|a, b| b.sort_key.cmp(&a.sort_key));

                // straight判定
                let mut straight_flag: bool = false;
                let mut count: usize = 0;
                while count <= flush_cards.len() - 5 {
                    if flush_cards[count].sort_key - flush_cards[count + 4].sort_key == 4 {
                        straight_flag = true;
                    }
                    count += 1;
                }

                // ASC
                flush_cards.reverse();

                if straight_flag == false && flush_cards[0].sort_key - flush_cards[3].sort_key == -3 && flush_cards[0].sort_key == 2 && flush_cards.last().unwrap().sort_key == 14 {
                    straight_flag = true;
                }

                // DESC
                flush_cards.reverse();

                if straight_flag == true {
                    if flush_cards[0].sort_key == 14 && flush_cards[1].sort_key == 13 && flush_cards[2].sort_key == 12 {
                        hand = Some(Hand::RoyalFlush);
                        break 'outer;
                    } else {
                        hand = Some(Hand::StraightFlush);
                        break 'outer;
                    }
                } else {
                    hand = Some(Hand::Flush);
                    break 'outer;
                }
            };
        };

        let rank_groups = cards.clone().into_iter().into_group_map_by(|x| x.rank);
        for rank_group in rank_groups.iter() {
            if rank_group.1.len() == 4 {
                hand = Some(Hand::FourOfAKind);
                break 'outer;
            }
        }
    }

    match &hand {
           Some(v) => {
               println!("finish! Hand is {}", v.to_string());
           }
           None => {
               println!("unknown hand");
           }
       }
}


// TODO
// [] ホールデムの役判定
// [] badugi、27、8 or betterの役判定
