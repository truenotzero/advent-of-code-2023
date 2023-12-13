use std::fs;


fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let out = process(&input);
    println!("{out}")
}

fn match_digit_by_sigil_or_name(s: &str) -> Option<u32> {
    // first check if current char matches a digit
    match s.chars().next().map(|e: char| (e.is_ascii_digit(), e)) {
        Some((true, n)) => return char::to_digit(n, 10),
        _ => (),
    };

    // if not try matching by name
    let digit_names = [ "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

    for (idx, &name) in digit_names.iter().enumerate() {
        if s.starts_with(name) {
            return Some(idx as _);
        }
    };

    // Not found
    None
}

fn recurse_from_start<T>(s: &str, pred: fn(&str) -> Option<T>) -> Option<T> {
    match pred(s) {
        None => recurse_from_start(&s[1..], pred),
        n => n,
    }
}

fn recurse_from_end<T>(s: &str, pred: fn(&str) -> Option<T>) -> Option<T> {
    for i in (1..s.len()).rev() {
        match pred(&s[i..]) {
            None => continue,
            n => return n,
        }
    }

    None
}

fn process(cal: &str) -> u32 {
    cal.lines()
        .map(|l| {
            let tens = recurse_from_start(l, match_digit_by_sigil_or_name).unwrap();
            let units = recurse_from_end(l, match_digit_by_sigil_or_name).unwrap_or(tens);
            10 * tens + units
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn example() {
        let cal = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let calc = process(cal);
        assert_eq!(calc, 281);
    }
}
