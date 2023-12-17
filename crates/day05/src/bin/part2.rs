
fn main() {
    do_file("./input.txt");
}

fn do_file(filename: &str) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

type Num = i64;

#[derive(Clone, Copy)]
struct Range {
    start: Num,
    end: Num
}

impl Range {
    fn new(start: Num, len: Num) -> Self {
        Self {
            start,
            end: start + len,
        }
    }
}

struct Rule {
    source: Range,
    dest: Range,
}

#[derive(Default)]
struct RuleStep {
    unmapped: Vec<Range>,
    mapped: Vec<Range>,
}

impl RuleStep {
    fn consume(mut self, other: &mut RuleStep) -> RuleStep {
        self.mapped.append(&mut other.mapped);
        self
    }

    fn finalize(self) -> Vec<Range> {
        let Self { mut mapped, mut unmapped } = self;
        mapped.append(&mut unmapped);
        mapped
    }
}

impl From<Range> for RuleStep {
    fn from(value: Range) -> Self {
        Self {
            unmapped: vec![value],
            ..Default::default()
        }
    }
}

impl Rule {
    fn apply(&self, input: &mut RuleStep) -> RuleStep {
        let ret = input.unmapped
            .iter()
            .map(|&r| self.apply_to_range(r))
            .fold(RuleStep::default(), |mut acc, mut e| {
                acc.mapped.append(&mut e.mapped);
                acc.unmapped.append(&mut e.unmapped);
                acc
            })
        ;
        ret.consume(input)
    }

    fn apply_to_range(&self, n: Range) -> RuleStep {
        // brief overview of the algo
        // only n matters
        // so n can be drawn as a diagram like this in respect to self
        // |-n-| |-self-| or |-self-| |-n-| (unrelated)
        // or
        // |-?unmapped-||-mapped-||-?unmapped-|
        // where the ? ranges are optional
        if n.end < self.source.start || n.start > self.source.end {
            // unrelated ranges
            n.into()
        } else { // below it follows that: n.end >= self.source.start
            let mut ret = RuleStep::default();
            // find the pre-map range
            let premap_start = n.start.max(self.source.start);
            let premap_end = n.end.min(self.source.end);
            // now map the range
            let delta = self.dest.start - self.source.start;
            let map_start = premap_start + delta;
            let map_end = premap_end + delta;
            ret.mapped.push(Range { start: map_start, end: map_end });

            // check if the unmapped anterior range exists
            if n.start < self.source.start {
                ret.unmapped.push(Range { start: n.start , end: premap_start });
            }

            // check if the unmapped posterior range exists
            if n.end > self.source.end {
                ret.unmapped.push(Range { start: premap_end, end: n.end });
            }
            ret
        }
    }
}

struct Map {
    ranges: Vec<Rule>,
}

impl Map {
    fn new(slice: &str) -> Self {
        let mut it = slice.lines();
        let _descriptor = it.next();

        let mut ranges = Vec::new();
        for line in it {
            // println!("Parsing line to map: *{line}*");
            let range = {
                let mut it = line.split(' ');
                let val_start = it.next().expect("No val_start");
                let dest_range_start = Num::from_str_radix(val_start, 10).expect("Failed parsing val_start");

                let key_start = it.next().expect("No key_start");
                let source_range_start = Num::from_str_radix(key_start, 10).expect("Failed parsing key_start");

                let range = it.next().expect("No range");
                let range_len = Num::from_str_radix(range, 10).expect("Failed parsing range");

                Rule { 
                    source: Range::new(source_range_start, range_len),
                    dest: Range::new(dest_range_start, range_len),
                }
            };

            ranges.push(range);
        }

        Self { ranges }
    }

    fn apply(&self, n: Range) -> Vec<Range> {
        // FIXME: possible optimization by combining ranges
        let rs = RuleStep::from(n);
        self.ranges
            .iter()
            .fold(rs, |mut acc, e| {
                e.apply(&mut acc).consume(&mut acc)
            })
            .finalize()
    }
}

#[derive(Default)]
struct Chain {
    maps: Vec<Map>,
}

impl Chain {
    fn apply(&self, n: Range) -> Vec<Range> {
        self.maps
            .iter()
            .fold(vec![n], |acc, map| {
                acc
                    .into_iter()
                    .flat_map(|e| map.apply(e))
                    .collect()
            })
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


fn process(input: &str) -> Num {
    let mut input =input.split("\r\n\r\n");
    let seeds = input.next().expect("No seeds"); // with preamble (seeds: ...)
    let (_, seeds) = seeds.split_once(": ").expect("Failed removing seed preamble"); //  raw seed data

    let chain = input
        .map(Map::new)
        .collect::<Chain>();
    

    println!("seeds=[{seeds}]");
    seeds.split(' ')
        .map(|s| Num::from_str_radix(s, 10).expect("Failed parsing seeds"))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| Range::new(chunk[0], chunk[1]))
        .flat_map(|s| chain.apply(s))
        .min_by(|l, r| l.start.cmp(&r.start))
        .unwrap()
        .start
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 46);
    }
}
