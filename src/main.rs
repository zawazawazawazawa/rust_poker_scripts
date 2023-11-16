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
    }
    
    let mut decks = Vec::<Card>::new();


    for i in 0..ranks.len() {
        for j in 0..suits.len() {
            decks.push(
                Card {
                    rank: &ranks[i],
                    suit: &suits[j]
                }
            );
        }
    };

    let mut rng = thread_rng();

    loop {
        println!("================================");
        let cards = decks.iter().choose_multiple(&mut rng, 7);

        let groups = cards.into_iter().into_group_map_by(|x| x.suit);

        let mut flush_flag: bool = false;
        for group in groups.iter() {
            println!("{:?}: {}", group.0, group.1.len());
            if group.1.len() >= 5 {
                flush_flag = true;
            };
        };
        if flush_flag == true {
            println!("flush!!");
            break;
        }
    }
}


// TODO
// [x] deckからramdomな7枚を取り出せる
// [] flushの判定
// [] straightの判定
// [] ホールデムの役判定
// [] badugi、2-7、8 or betterの役判定
