use std::collections::{HashMap, HashSet};

fn main() {
    // do_file("./input.txt");
    do_file(r#"C:\Users\true\Documents\dev\rust\advent-of-code-2023\day08\input.txt"#);
}

type Num = i32;

fn do_file(filename: &str) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

fn step<'a>(node: &str, nodes: &'a HashMap<&'a str, (&str, &str)>, step: char) -> &'a str {
    let options = nodes[node];
    match step {
        'L' => options.0,
        'R' => options.1,
        _ => panic!("Bad step"),
    }
}

fn reach_z<'a, I: Iterator<Item=char>>(mut node: &'a str, nodes: &'a HashMap<&str, (&str, &str)>, mut instructions: I) -> (&'a str, I) {
    loop {
        if node.ends_with('Z') {
            return (node, instructions);
        } else {
            let next = step(&node, &nodes, instructions.next().expect("Iterator exhauster; call .cycle()"));
            node = next;
        }
    }
}

fn measure_cycle<I: Iterator<Item=char>>(initial_node: &str, nodes: &HashMap<&str, (&str, &str)>, mut instructions: I) -> Num {
    let stepped_node = step(initial_node, nodes, instructions.next().expect("Iterator exhausted; call .cycle()"));
    // the line above is to actually have the cycle start
    // we add one below to compensate for the step taken above
    1 + instructions
        .enumerate()
        .try_fold(stepped_node, |node, (idx, s)| {
            // abuse try_fold's short-circuit mechanic
            // allows us to use fold, and allows us to return whenever we please
            // the side effect is that the desired output is stored
            // as the error variant of the returned Result 
            if node == initial_node {
                Err(idx as Num)
            } else {
                Ok(step(node, nodes, s))
            }
        })
        .unwrap_err()
}

fn prime_sieve(max: Num) -> Vec<Num> {
    let mut ret = vec![true; (max+1) as _];
    // set some non-primes by hand
    if ret.len() > 0 { ret[0] = false; }
    if ret.len() > 1 { ret[1] = false; }

    for i in 0..ret.len() {
        let p = ret[i as usize];
        if !p { continue }
        for j in (i..ret.len()).step_by(i) {
            if i == j { continue }
            ret[j as usize] = false;
        }
    }

    ret
        .into_iter()
        .enumerate()
        .filter_map(|(idx, e)| if e { Some(idx as _) } else { None })
        .collect()
}

fn factorize(n: Num) -> HashSet<Num> {
    let mut factors = HashSet::new();
    for p in prime_sieve(n) {
        if p % n == 0 {
            factors.insert(p);
        }
    }
    factors
}

fn least_common_multiple(acc: Num, e: Num) -> Num {
    let acc = factorize(acc);
    let e = factorize(e);
    acc.into_iter().chain(e.into_iter()).product()
}

fn process(input: &str) -> Num {
    let (instructions, nodes) = input
        .split_once("\r\n\r\n")
        .expect("Bad formatting for instructions/nodes");
    let nodes = nodes
        .split("\r\n")
        .map(|node| {
            let (node, links) = node.split_once(" = ").expect("Bad node formatting");
            let (left, right) = links.split_once(", ").expect("Bad links formatting");
            let left = &left[1..]; // trim '('
            let right = &right[..(right.len() - 1)]; // trim ')'
            (node, (left, right))
        })
        .collect::<HashMap<_, _>>();

    nodes
        .iter()
        .filter_map(|(key, _)| if key.ends_with('A') { Some(*key) } else { None }) // collect starting nodes (**A)
        .map(|node| reach_z(node, &nodes, instructions.chars().cycle()))
        .map(|(node, instructions)| measure_cycle(node, &nodes, instructions))
        .reduce(least_common_multiple)
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example3() {
        // first try!
        assert_eq!(crate::do_file("./example3.txt"), 6);
    }
}
