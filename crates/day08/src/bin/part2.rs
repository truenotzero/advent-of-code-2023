use std::collections::{HashMap, HashSet};

fn main() {
    do_file("./input.txt");
}

type Num = i64;

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
    let mut sieve = vec![true; (max+1) as _];
    // set some non-primes by hand
    if sieve.len() > 0 { sieve[0] = false; }
    if sieve.len() > 1 { sieve[1] = false; }

    for i in 0..sieve.len() {
        let p = sieve[i as usize];
        if !p { continue }
        for j in (i..sieve.len()).step_by(i) {
            if i == j { continue }
            sieve[j as usize] = false;
        }
    }
    
    let mut ret = Vec::new();
    for i in 0..(max+1) {
        if sieve[i as usize] {
            ret.push(i);
        }
    }
    ret
}

fn factorize(n: Num, primes: &Vec<Num>) -> HashSet<Num> {
    let mut factors = HashSet::new();
    for &p in primes {
        if n % p == 0 {
            factors.insert(p);
        }
    }
    factors
}

fn least_common_multiple<I: IntoIterator<Item=Num>>(nums: I) -> Num {
    let nums = nums.into_iter().collect::<Vec<_>>();
    let max = *nums.iter().max().expect("Empty iterator");
    let primes = prime_sieve(max);

    nums
        .into_iter()
        .flat_map(|n| factorize(n, &primes))
        .collect::<HashSet<_>>()
        .into_iter()
        .product()
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

    let cycles = nodes
        .iter()
        .filter_map(|(key, _)| if key.ends_with('A') { Some(*key) } else { None }) // collect starting nodes (**A)
        .map(|node| reach_z(node, &nodes, instructions.chars().cycle()))
        .map(|(node, instructions)| measure_cycle(node, &nodes, instructions))
        ;

    least_common_multiple(cycles)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example3() {
        // first try!
        assert_eq!(crate::do_file("./example3.txt"), 6);
    }
}
