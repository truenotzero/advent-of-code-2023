
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

    fn break_records(self) -> Num {
        // check from start
        let start = (1..self.duration)
            .map(|w| (w, self.simulate(w)))
            .find(|(_w, d)| *d > self.record_distance)
            .map(|(w,_)| w)
            .expect("Failed to break record: no starting value");

        // check from end
        let end = (1..self.duration)
            .rev()
            .map(|w| (w, self.simulate(w)))
            .find(|(_, d)| *d > self.record_distance)
            .map(|(w,_)| w)
            .expect("Failed to break record: no ending value");

        end - start + 1
    }
}

fn process(input: &str) -> Num {
    let mut it = input.lines();
    let (_, time_str) = it.next().expect("No race time").split_once(":").expect("Bad formatting for race time");
    let duration = time_str.split_whitespace()
        .flat_map(|e| e.chars())
        .filter_map(|e| e.to_digit(10))
        .map(|e| e as Num)// needed to prevent overflow
        .fold(0, |acc, e| 10 * acc + e)
        ;

    let (_, dist_str) = it.next().expect("No race distance").split_once(":").expect("Bad race distance formatting");
    let record_distance = dist_str.split_whitespace()
        .flat_map(|e| e.chars())
        .filter_map(|e| e.to_digit(10))
        .map(|e| e as Num)// needed to prevent overflow
        .fold(0, |acc, e| 10 * acc + e)
        ;

    Race { duration, record_distance }.break_records()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 71503);
    }
}
