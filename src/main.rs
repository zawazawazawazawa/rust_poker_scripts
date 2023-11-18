use itertools::Itertools;
use rand::{seq::IteratorRandom, thread_rng};

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

    loop {
        let cards = decks.iter().choose_multiple(&mut rng, 7);

        let groups = cards.into_iter().into_group_map_by(|x| x.suit);

        let mut straight_flag: bool = false;
        let mut finish_flag:bool = false;
        for group in groups.iter() {
            if group.1.len() >= 5 {
                let mut flush_cards = group.1.clone();
                flush_cards.sort_by(|a, b| b.sort_key.cmp(&a.sort_key));
                println!("================================");
                println!("Flush!!");

                // straight判定
                let mut count: usize = 0;
                while count <= flush_cards.len() - 5 {
                    if flush_cards[count].sort_key - flush_cards[count + 4].sort_key == 4 {
                        println!("Straight Flush !!!! : {:?}", flush_cards);
                        straight_flag = true;
                    }
                    count += 1;
                }

                // ASC
                flush_cards.reverse();

                if straight_flag == false && flush_cards[0].sort_key - flush_cards[3].sort_key == -3 && flush_cards[0].sort_key == 2 && flush_cards.last().unwrap().sort_key == 14 {
                    println!("Straight Flush !!!! : {:?}", flush_cards);
                    straight_flag = true;
                }

                // DESC
                flush_cards.reverse();

                if straight_flag == true && flush_cards[0].sort_key == 14 && flush_cards[1].sort_key == 13 && flush_cards[2].sort_key == 12 {
                    println!("Royal Flush !!!! {:?}", flush_cards);
                    finish_flag = true;
                }
            };
        };
        if finish_flag == true {
            println!("finish!!");
            break;
        }
    }
}


// TODO
// [x] deckからramdomな7枚を取り出せる
// [x] flushの判定
//rustc --explain E0716/ [] straight flushの判定
// [] straightの判定
// [] ホールデムの役判定
// [] badugi、2-7、8 or betterの役判定
