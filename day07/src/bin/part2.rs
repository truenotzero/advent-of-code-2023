use std::{collections::HashMap, fmt::Debug};


fn main() {
    // TODO: unsolved
    do_file("./input.txt");
}

type Num = i32;

fn do_file(filename: &str) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    val: Num,
    face: char
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.face)
    }
}

impl Card {
    fn new(ch: char) -> Option<Self> {
        let allowed_values = ['J','2','3','4','5','6','7','8','9','T','Q','K','A'];
        if allowed_values.contains(&ch) { 
            Some(Self {
                val: allowed_values
                        .iter()
                        .position(|e| e == &ch)
                        .expect("Can't get value") as _,
                face: ch,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    r#type: Type,
    cards: [Card; 5],
}

impl From<&str> for Hand {
    fn from(value: &str) -> Hand {
        let cards = value
            .chars()
            .filter_map(Card::new)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Can't parse cards");

        let mut counts = HashMap::new();
        let mut joker = None;
        for card in cards {
            let card: Card = card;
            if card.face == 'J' {
                joker = Some(card);
            }
            let val = counts.get(&card).unwrap_or(&0) + 1;
            counts.insert(card, val);
        }

        // removing max
        // finds the card that has the most appearances
        // and gives back said card + number of appearances
        // and removes it from the map
        fn rem_max(map: &mut HashMap<Card, Num>) -> (Card, Num) {
            let ret = {
                let (r1, r2) = map
                .iter()
                .max_by(|(_, tl), (_, tr)| tl.cmp(tr))
                .expect("No max card");
                (*r1, *r2)
            };
            map.remove(&ret.0);
            ret
        }

        let (mut card, mut times) = rem_max(&mut counts);
        if card.face == 'J' && counts.len() > 0 {
            let (cc, tt) = rem_max(&mut counts);
            card = cc;
            times += tt;
        } else if times > 1 {
            if let Some(joker) = joker {
                times += counts.get(&joker).map(|e| *e).unwrap_or_default();
            }
        }

        use Type as T;
        let r#type = match times {
            1 => T::HighCard,
            2 => {
                let (_, lotimes) = rem_max(&mut counts);
                if lotimes == 2 {
                    T::TwoPair
                } else {
                    T::Pair
                }
            },
            3 => {
                let (_,lotimes) = rem_max(&mut counts);
                if lotimes == 2 {
                    // fullhouse!
                    T::FullHouse
                } else {
                    T::ThreeOfAKind
                }
            },
            4 => T::FourOfAKind,
            5 => T::FiveOfAKind,
            _ => panic!("Bad card repetition count"),
        };
        Self { r#type, cards }
    }
}

fn process(input: &str) -> Num {
    let mut hands = input
        .split("\r\n")
        .flat_map(|l| l.split(' '))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .map(|(hand, bet)| (hand.try_into(), Num::from_str_radix(bet, 10)))
        .map(|(hand, bet)| (hand.expect("Failed parsing hand"), bet.expect("Failed parsing bet")))
        .collect::<Vec<(Hand, Num)>>();

    hands.sort(); // crucial!

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (hand, bet))| {
            let rank = (i as Num) + 1;
            let win = bet * rank;
            println!("hand={hand:?} bet={bet} rank={rank} -> win={win}");
            acc + win
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 5905);
    }
}
