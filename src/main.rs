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
fn judge_straight(cards: Vec::<&Card>) -> bool {
    let mut count: usize = 0;
    let sort_keys: Vec<i32> = cards.iter().map(|x| x.sort_key).unique().collect();
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

fn judge_hand_strength(hand: &Hand) -> i32 {
    match hand {
        Hand::RoyalFlush => 1,
        Hand::StraightFlush => 2,
        Hand::FourOfAKind => 3,
        Hand::FullHouse => 4,
        Hand::Flush => 5,
        Hand::Straight => 6,
        Hand::ThreeOfAKind => 7,
        Hand::TwoPair => 8, 
        Hand::OnePair => 9,
        Hand::HighCard => 10,
    }
}

fn judge_hand(cards: &Vec<&Card>) -> Hand {
    // flushを判定
    let suit_groups: HashMap<&String, Vec::<&Card>> = cards.clone().into_iter().into_group_map_by(|x| x.suit);
    for group in suit_groups.iter() {
        if group.1.len() >= 5 {
            // straight flush, royal flushを判定
            let mut flush_cards = group.1.clone();
            flush_cards.sort_by(|a, b| b.sort_key.cmp(&a.sort_key));
            let straight_flag: bool = judge_straight(flush_cards.clone());
            if straight_flag == true {
                if flush_cards[0].sort_key == 14 && flush_cards[1].sort_key == 13 && flush_cards[2].sort_key == 12 {
                    return Hand::RoyalFlush;
                } else {
                    return Hand::StraightFlush;
                }
            } else {
                return Hand::Flush;
            }
        };
    };

    // quadsを判定
    let rank_groups = cards.clone().into_iter().into_group_map_by(|x| x.rank);
    for rank_group in rank_groups.iter() {
        if rank_group.1.len() == 4 {
            return Hand::FourOfAKind;
        }
    }

    let mut cards2: Vec::<&Card> = cards.clone();
    cards2.sort_by(|a, b| b.sort_key.cmp(&a.sort_key));
    if judge_straight(cards2.clone()) {
        return Hand::Straight;
    }

    // full house と tripsを判定
    // TODO: quads判定とtrips判定で2回回しているので一回で判定できるか検討
    let mut trips_flag: bool = false;
    let mut pair_flag: bool = false;
    for rank_group in rank_groups.iter() {
        if rank_group.1.len() == 3 {
            trips_flag = true
        } else if rank_group.1.len() == 2 {
            pair_flag = true;
        }
    }

            
    if trips_flag {
        if pair_flag {
            return Hand::FullHouse;
        } else {
            return Hand::ThreeOfAKind;
        }
    }

    // two pair, pair, highcardを判定
    let rank_counts: Vec<usize> = rank_groups.into_iter().map(|x| x.1.len()).collect();

    let pair_count = rank_counts.into_iter().filter(|&x| x == 2).collect::<Vec<usize>>().len();

    if pair_count >= 2 {
        return Hand::TwoPair;
    } else if pair_count == 1 {
        return Hand::OnePair;
    } else {
        return Hand::HighCard;
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

    // deckを準備
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
    let player_1_hand: Hand;
    let player_2_hand: Hand;
    let player_1_cards: Vec::<&Card> = decks.iter().choose_multiple(&mut rng, 7);
    let player_2_cards: Vec::<&Card> = decks.iter().choose_multiple(&mut rng, 7);

    player_1_hand = judge_hand(&player_1_cards);
    player_2_hand = judge_hand(&player_2_cards);

    if judge_hand_strength(&player_1_hand) < judge_hand_strength(&player_2_hand) {
        println!("Win player 1")
    } else if judge_hand_strength(&player_1_hand) > judge_hand_strength(&player_2_hand) {
        println!("Win player 2")
    } else {
        // TODO implementation
        println!("draw")
    }

    for card in player_1_cards {
        println!("{:?}", card);
    };
    println!("hand strength: {}", judge_hand_strength(&player_1_hand));
    println!("finish! Hand is {}", player_1_hand.to_string());
    
    for card in player_2_cards {
        println!("{:?}", card);
    };
    println!("hand strength: {}", judge_hand_strength(&player_2_hand));
    println!("finish! Hand is {}", player_2_hand.to_string());

    
}


// TODO
// card7枚を二組用意してどっちが勝ったかの判定
// test
// badugi、27、8 or
