
fn main() {
    do_file("./input.txt");
}

fn do_file(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

struct Range {
    source_range_start: usize,
    dest_range_start: usize,
    range_len: usize,
}

impl Range {
    // if n in range then maps it
    // otherwise returns none
    fn map(&self, n: usize) -> Option<usize> {
        let source_range_end = self.source_range_start + self.range_len;
        if (self.source_range_start..source_range_end).contains(&n) {
            let delta = n - self.source_range_start;
            Some(self.dest_range_start + delta)
        } else {
            None
        }
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(slice: &str) -> Self {
        let mut it = slice.split("\r\n");
        let _descriptor = it.next();

        let mut ranges = Vec::new();
        for line in it {
            println!("Parsing line to map: *{line}*");
            let range = {
                let mut it = line.split(' ');
                let val_start = it.next().expect("No val_start");
                let dest_range_start = usize::from_str_radix(val_start, 10).expect("Failed parsing val_start");

                let key_start = it.next().expect("No key_start");
                let source_range_start = usize::from_str_radix(key_start, 10).expect("Failed parsing key_start");

                let range = it.next().expect("No range");
                let range_len = usize::from_str_radix(range, 10).expect("Failed parsing range");

                Range {dest_range_start, source_range_start, range_len}
            };

            ranges.push(range);
        }

        Self { ranges }
    }

    fn apply(&self, n: usize) -> usize {
        self.ranges.iter()
            .filter_map(|range| range.map(n))
            .next()
            .unwrap_or(n)
    }
}

#[derive(Default)]
struct Chain {
    maps: Vec<Map>,
}

impl Chain {
    fn apply(&self, n: usize) -> usize {
        self.maps.iter().fold(n, |acc, map| { map.apply(acc) })
    }

    fn add(&mut self, map: Map) {
        self.maps.push(map);
    }
}

impl FromIterator<Map> for Chain {
    fn from_iter<T: IntoIterator<Item = Map>>(iter: T) -> Self {
        let mut chain = Chain::default();
        iter.into_iter().for_each(|e| chain.add(e));
        chain
    }
}


fn process(input: &str) -> usize {
    let mut input =input.split("\r\n\r\n");
    let seeds = input.next().expect("No seeds"); // with preamble (seeds: ...)
    let (_, seeds) = seeds.split_once(": ").expect("Failed removing seed preamble"); //  raw seed data

    let chain = input
        .map(Map::new)
        .collect::<Chain>();
    

    println!("seeds=[{seeds}]");
    seeds.split(' ')
        .map(|s| usize::from_str_radix(s, 10).expect("Failed parsing seeds"))
        .map(|s| chain.apply(s))
        .min().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 35);
    }
}
