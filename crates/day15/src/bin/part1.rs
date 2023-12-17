use std::num::Wrapping;


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

fn hash(input: &str) -> Wrapping<u8> {
    input
        .as_bytes()
        .into_iter()
        .fold(Wrapping(0), |mut acc, &b| {
            acc += b;
            acc *= 17;
            acc
        })
}

fn process(input: &str) -> Num {
    input
        .split(",")
        .map(hash)
        .map(|m| m.0 as Num)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    #[test]
    fn test_hash() {
        let hash = crate::hash("HASH");
        assert_eq!(hash, Wrapping(52));
    }

    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 1320);
    }
}
