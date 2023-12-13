use std::collections::HashMap;

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

fn process(input: &str) -> Num {
    let mut it = input.split("\r\n\r\n");
    let instructions = it.next().expect("No instructions found");
    let nodes = it
        .next()
        .expect("No nodes found")
        .split("\r\n")
        .map(|node| {
            let (node, links) = node.split_once(" = ").expect("Bad node formatting");
            let (left, right) = links.split_once(", ").expect("Bad links formatting");
            let left = &left[1..]; // trim '('
            let right = &right[..(right.len() - 1)]; // trim ')'
            (node, (left, right))
        })
        .collect::<HashMap<_, _>>();

    instructions
        .chars()
        .cycle() // creates an infinite, looping iterator
        .enumerate()
        .try_fold("AAA", |node, (stepno, step)| {
            // abuse try_fold's short-circuit mechanic
            // allows us to use fold, and allows us to return whenever we please
            // the side effect is that the desired output is stored
            // as the error variant of the returned Result
            if node == "ZZZ" {
                Err(stepno as _)
            } else {
                let choice = nodes[node];
                let next = match step {
                    'L' => choice.0,
                    'R' => choice.1,
                    _ => panic!(),
                };
                Ok(next)
            }
        })
        .unwrap_err() // retrieve the value from try_fold hack
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(crate::do_file("./example1.txt"), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(crate::do_file("./example2.txt"), 6);
    }
}
