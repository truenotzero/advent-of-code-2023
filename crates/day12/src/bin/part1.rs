
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

enum Spring {
    Good,
    Bad,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Good,
            '#' => Self::Bad,
            '?' => Self::Unknown,
            _ => panic!("Illegal spring state"),
        }
    }
}

fn process(input: &str) -> Num {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), todo!());
    }
}
