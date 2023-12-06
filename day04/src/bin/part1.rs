
fn main() {
    do_file("./input.txt");
}

fn do_file(filename: &str) -> u32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

// split card into two strings: winning numbers and my numbers
// also get rid of the 'Card X: ' start
fn split_card(line: &str) -> (&str, &str) {
    let (_, numbers) = line.split_once(':').expect("No :");
    let (lucky, pool) = numbers.split_once('|').expect("No |");
    (lucky.trim(), pool.trim())
}

fn card_value((lucky, pool): (&str, &str)) -> u32 {
    let pool = pool.split(' ').collect::<Vec<_>>();
    let exp = lucky.split(' ')
        .filter(|e| !e.is_empty())
        .filter(|e| pool.contains(e))
        .count();
    if exp == 0 { return 0; }
    let exp = exp - 1;
    let ret = 2u32.pow(exp as _);
    ret
}

fn process(input: &str) -> u32 {
   input.lines()
        .map(split_card)
        .map(card_value)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 13);
    }
}
