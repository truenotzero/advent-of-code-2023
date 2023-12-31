use std::fs;

fn main() {
        let bag = Set {
            red_cubes: 12,
            green_cubes: 13,
            blue_cubes: 14,
        };
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = process(&input, &bag);
        println!("{output}");
}

#[derive(Default, Debug)]
struct Set {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl Set {
    fn is_legal(&self, bag: &Self) -> bool {
        self.red_cubes <= bag.red_cubes
            && self.green_cubes <= bag.green_cubes
            && self.blue_cubes <= bag.blue_cubes
    }
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn is_legal(&self, bag: &Set) -> bool {
        self.sets
            .iter()
            .map(|set| set.is_legal(bag))
            .reduce(|a, e| a && e)
            .unwrap()
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (idhalf, sethalf) = value.split_once(':').expect("No colon in string");
        let (_, id) = idhalf.split_once(' ').expect("Bad format: 'Game #'");
        let id = u32::from_str_radix(id, 10).expect("Can't convert game id to num");

        let sets = sethalf
            .split(';') // split into sets by ;
            .map(|set| {
                let mut ret = Set::default();
                set.split(',')
                    .map(|colnum| {
                        let (num, col) = colnum.trim().split_once(' ').unwrap();
                        let num = u32::from_str_radix(num, 10).unwrap();
                        (num, col)
                    })
                    .for_each(|(num, col)| match col {
                        "red" => ret.red_cubes = num,
                        "green" => ret.green_cubes = num,
                        "blue" => ret.blue_cubes = num,
                        _ => panic!("Bad color"),
                    });
                ret
            })
            .collect();
        Game { id, sets }
    }
}

fn process(record: &str, bag: &Set) -> u32 {
    record
        .lines()
        .map(Game::from)
        .filter(|g| g.is_legal(bag))
        .map(|g| g.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{process, Set};

    #[test]
    fn example() {
        let bag = Set {
            red_cubes: 12,
            green_cubes: 13,
            blue_cubes: 14,
        };
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let output = process(input, &bag);
        assert_eq!(output, 8);
    }
}
