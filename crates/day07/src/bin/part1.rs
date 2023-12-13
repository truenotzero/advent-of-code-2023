use std::{collections::HashMap, fmt::Debug};


fn main() {
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
        let allowed_values = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
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
        for card in cards {
            let val = counts.get(&card).unwrap_or(&0) + 1;
            counts.insert(card, val);
        }

        // removing max
        // finds the card that has the most appearances
        // and gives back said number of appearances
        // and removes it from the map
        fn rem_max(map: &mut HashMap<Card, Num>) -> Num {
            let ret = {
                let (r1, r2) = map
                .iter()
                .max_by(|(_, tl), (_, tr)| tl.cmp(tr))
                .expect("No max card");
                (*r1, *r2)
            };
            map.remove(&ret.0);
            ret.1
        }

        let times = rem_max(&mut counts);

        use Type as T;
        let r#type = match times {
            1 => T::HighCard,
            2 => {
                let lotimes = rem_max(&mut counts);
                if lotimes == 2 {
                    T::TwoPair
                } else {
                    T::Pair
                }
            },
            3 => {
                let lotimes = rem_max(&mut counts);
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
        .fold(0, |acc, (i, (_hand, bet))| {
            let rank = (i as Num) + 1;
            let win = bet * rank;
            acc + win
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 6440);
    }
}
