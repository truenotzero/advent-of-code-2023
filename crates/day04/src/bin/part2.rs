
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

fn card_wins((lucky, pool): (&str, &str)) -> u32 {
    let pool = pool.split(' ').collect::<Vec<_>>();
    lucky.split(' ')
        .filter(|e| !e.is_empty())
        .filter(|e| pool.contains(e))
        .count() as _
}

fn calculate_total_cards(card_wins: Vec<u32>) -> u32 {
    let mut copies = vec![1; card_wins.len()];

    for (i, wins) in card_wins.iter().enumerate() {
        print!("#{} has {wins} match nums, so you win: ", i + 1);
        for j in 1..=*wins {
            let copy = copies[i];
            copies[i + j as usize] += copy;
            print!("{copy} of #{}, ", j+1);
        }
        println!();
    };

    copies.iter().sum()
}

fn process(input: &str) -> u32 {
   let card_wins = input.lines()
        .map(split_card)
        .map(card_wins)
        .collect::<Vec<_>>();

    calculate_total_cards(card_wins)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 30);
    }
}
