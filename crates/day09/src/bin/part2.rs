
fn main() {
    // FIRST TRY!!!!!!!!!!!!
    do_file("./input.txt");
}

type Num = i32;

fn do_file(filename: &str) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

type Value = Num;
type ValueOverTime = Vec<Value>;

fn is_zero_sequence(vot: &ValueOverTime) -> bool {
    vot.iter()
        .fold(true, |acc, e| acc && *e == 0)
}

fn difference_sequence(vot: &ValueOverTime) -> ValueOverTime {
    vot.windows(2)
        .map(|w| w[1] - w[0])
        .collect()
}

fn extrapolate(vot: ValueOverTime) -> Value {
    // first create sequences of differences
    let mut seqs = vec![vot];
    loop {
        let seq = seqs.last().unwrap();
        let next_seq = difference_sequence(&seq);
        if is_zero_sequence(&next_seq) { break }
        seqs.push(next_seq);
    }

    // extrapolate
    seqs
        .into_iter()
        .rev()
        .fold(0, |acc, seq| {
            let val = seq[0];
            val - acc
        })
}

fn to_value_over_time(s: &str) -> ValueOverTime {
    s.split(' ')
     .filter_map(|e| Num::from_str_radix(e, 10).ok())
     .collect::<ValueOverTime>()
}

fn process(input: &str) -> Num {
    input
        .lines()
        .map(to_value_over_time)
        .map(extrapolate)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        // FIRST TRY!!!!!!!!!!
        assert_eq!(crate::do_file("./example.txt"), 2);
    }
}
