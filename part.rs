
fn main() {
    do_file("./input.txt");
}

fn do_file(filename: &str) -> u32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

fn process(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), todo!());
    }
}
