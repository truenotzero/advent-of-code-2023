
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

#[derive(Debug)]
struct Race {
    record_distance: Num,
    duration: Num,
}

impl Race {
    fn simulate(&self, windup_time: Num) -> Num {
        let accel = 1;
        let v = windup_time * accel;
        let race_time = self.duration - windup_time;
        v * race_time // distance
    }

    fn break_records(self) -> (Num, Num) {
        // check from start
        let start = (1..self.duration)
            .map(|w| (w, self.simulate(w)))
            .find(|(w, d)| *d > self.record_distance)
            .map(|(w,_)| w)
            .expect("Failed to break record: no starting value");

        // check from end
        let end = (1..self.duration)
            .rev()
            .map(|w| (w, self.simulate(w)))
            .find(|(_, d)| *d > self.record_distance)
            .map(|(w,_)| w)
            .expect("Failed to break record: no ending value");

        (start, end)
    }
}

fn process(input: &str) -> Num {
    let mut it = input.split('\n'); // split by newlines
    let race_durations = it
        .next() // get first line
        .expect("No race times") // unwrap Option
        .split_whitespace() // split by whitespace
        .skip(1); // skip Time:
    let race_record_distances = it
        .next() // get second line
        .expect("No race record distances") 
        .split_whitespace() 
        .skip(1); // skip Distance: 

    race_durations
        .zip(race_record_distances)
        .map(|(x, y)| {
            (Num::from_str_radix(x, 10).expect("Failed to parse race duration"),
            Num::from_str_radix(y, 10).expect("Failed to parse record distance"))
        })
        .map(|(duration, record_distance)| Race { record_distance, duration })
        .map(Race::break_records)
        .map(|(start, end)| (end - start + 1))
        .product()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 288);
    }
}
