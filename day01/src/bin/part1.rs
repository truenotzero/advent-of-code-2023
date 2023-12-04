use std::fs;


fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let out = process(&input);
    println!("{out}")
}

fn process(cal: &str) -> u32 {
    cal
        .lines()
        .map(|l| {
            let mut iter = l.matches(|c: char| c.is_ascii_digit());
            let tens = iter.next().unwrap();
            let units = iter.last().unwrap_or(tens);
            u32::from_str_radix(tens, 10).unwrap() * 10 + u32::from_str_radix(units, 10).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn example() {
        let cal = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;
        let calc = process(cal);
        assert_eq!(calc, 142);
    }
}
