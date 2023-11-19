use itertools::Itertools;
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashMap;
use std::fmt;

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
    TwoPair, 
    OnePair,
    HighCard
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::RoyalFlush => write!(f, "Royal flush"),
            Hand::StraightFlush => write!(f, "Straight flush"),
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

// straight判定
fn judge_straight(mut cards: Vec::<&Card>) -> bool {
    let mut count: usize = 0;
    let mut sort_keys: Vec<i32> = cards.iter().map(|x| x.sort_key).unique().collect();
    if sort_keys.len() < 5 {
        return false
    }
    while count <= sort_keys.len() - 5 {
        if sort_keys[count] == sort_keys[count + 1] + 1 && sort_keys[count] == sort_keys[count + 2] + 2 && sort_keys[count] == sort_keys[count + 3] + 3 && sort_keys[count] == sort_keys[count + 4] + 4  {
            return true;
        }

        count += 1;
    }

    if sort_keys[(sort_keys.len() - 5)..] == [5,4,3,2] && sort_keys[0] == 14 {
        return true;
    } else {
        return false;
    }
}

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
        let cards: Vec::<&Card> = decks.iter().choose_multiple(&mut rng, 7);

        // flushを判定
        let suit_groups: HashMap<&String, Vec::<&Card>> = cards.clone().into_iter().into_group_map_by(|x| x.suit);
        for group in suit_groups.iter() {
            if group.1.len() >= 5 {
                // straight flush, royal flushを判定
                let mut flush_cards = group.1.clone();
                flush_cards.sort_by(|a, b| b.sort_key.cmp(&a.sort_key));
                let mut straight_flag: bool = judge_straight(flush_cards.clone());
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

        // quadsを判定
        let rank_groups = cards.clone().into_iter().into_group_map_by(|x| x.rank);
        for rank_group in rank_groups.iter() {
            if rank_group.1.len() == 4 {
                hand = Some(Hand::FourOfAKind);
                break 'outer;
            }
        }

        let mut cards2: Vec::<&Card> = cards.clone();
        cards2.sort_by(|a, b| b.sort_key.cmp(&a.sort_key));
        if judge_straight(cards2.clone()) {
            hand = Some(Hand::Straight);
            break 'outer;
        }

        // TODO: quads判定とtrips判定で2回回しているので一回で判定できるか検討
        let mut trips_flag: bool = false;
        let mut pair_flag: bool = false;
        for rank_group in rank_groups.iter() {
            if rank_group.1.len() == 3 {
                trips_flag = true;
            } else if rank_group.1.len() == 2 {
                pair_flag = true;
            }
        }

            
        if trips_flag {
            if pair_flag {
                hand = Some(Hand::FullHouse);
                break 'outer;
            } else {
                hand = Some(Hand::ThreeOfAKind);
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
