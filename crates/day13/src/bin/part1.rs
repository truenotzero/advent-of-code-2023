
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

fn transpose<T: Clone>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let height = vec.len();
    let width = vec[0].len();
    let mut ret = vec![Vec::with_capacity(height); width];
    
    for x in 0..width {
        for y in 0..height {
            ret[x].push(vec[y][x].clone());
        }
    }

    ret
}

type Pattern = Vec<Vec<char>>;

fn count_rows_before_reflection_axis(vec: &Pattern) -> Option<Num> {
    vec.windows(2)
    .enumerate()
    // check if two rows are the same (potential reflection axis)
    .filter_map(|(idx, slice)| {
        if slice[0] == slice[1] { Some(idx as _) } else { None }
    })
    // check that the potential axis actually reflects
    // by checking that all rows before the axis and after it are the same
    .find_map(|idx| {
        vec[..=idx]
            .iter()
            .rev()
            // make sure that both ranges have equal size!!!!
            // zip stops producing elements when the shorter iterator stops producing elements
            .zip(vec[idx+1..].iter())
            .try_fold(1 + idx as Num, |acc, (l, r)| {
                if l == r { Some(acc) } else { None }
            })
    })
}

fn process(input: &str) -> Num {
    let patterns: Vec<Pattern> = input
        .split("\r\n\r\n")
        .map(|pattern| {
            pattern.lines()
            .map(|line| line.chars().collect())
            .collect()
        })
        .collect()
        ;
    let horizontal_axes = patterns.iter()
        .filter_map(count_rows_before_reflection_axis)
        // .inspect(|e| println!("horizontal axis: {e}"))
        .sum::<Num>();
    let vertical_axes = patterns.iter()
        .map(transpose)
        .filter_map(|e| count_rows_before_reflection_axis(&e))
        // .inspect(|e| println!("vertical axis: {e}"))
        .sum::<Num>();
    vertical_axes + 100 * horizontal_axes
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 405);
    }
}
