use std::fs;

use itertools::Itertools;

fn main() {
    do_file("./input.txt");
}

#[derive(Debug)]
struct Array {
    vec: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Array {
    fn new(scheme: &str) -> Self {
        let width= scheme.lines().next().unwrap().len(); // x
        let height = scheme.lines().count(); // y
        let mut vec = vec![vec!['\0'; width]; height];

        for (y, line) in scheme.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                vec[y][x] = ch;
            }
        }

        Self {
            vec,
            width: width as isize,
            height: height as isize,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        self.vec.get(y as usize).and_then(|e| e.get(x as usize)).copied()
    }
    
    fn cardinals(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        let mut ret = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue }
                if let Some(_) = self.get(x+dx, y+dy) {
                    ret.push((x+dx, y+dy));
                }
            }
        }
        // println!("cardinals: ({x},{y}) => {ret:?}");
        ret
    }

    fn parse_number(&self, x: isize, y: isize) -> Option<u32> {
        // first check that this is even a digit string
        self.get(x, y)?.to_digit(10)?;
        // find first digit in string
        let start_idx = {
            // we can check the one before automatically because
            // it's a given that the current x is a digit
            let mut x = x - 1;
            loop {
                // if we reached a . then the first digit is the x after this x
                if self.get(x, y)? == '.' { break x + 1 }
                // if we hit the start of the line then return anyway
                if x == 0 { break x } // 
                x -= 1;
            }
        };

        let mut acc = 0;
        for x in start_idx..self.width {
            let ch = self.get(x, y);
            // println!("parse: ({x},{y})=>{ch:?}");
            match ch.and_then(|ch| ch.to_digit(10)) {
                Some(n) => acc = 10 * acc + n,
                None => break,
            }
        }

        Some(acc)
    }
}

fn process(scheme: &str) -> u32 {
    let arr = Array::new(scheme);
    (0..arr.height)
        .flat_map(|y| {
            (0..arr.width).map(move |x| (x, y))
        })
        .filter(|(x,y)| { // filter to symbol coords
            let ch = arr.get(*x, *y).unwrap_or('.');
            // check not dot
            if ch == '.' { return false }
            // check not digit
            if ch.is_ascii_digit() { return false }
            
            true
        })
        .flat_map(|(x,y)| arr.cardinals(x, y)) // get each symbol's cardinals
        .unique() // remove duplicate coords
        .filter_map(|(x,y)| arr.parse_number(x, y)) // parse each coord into a number or filter it if not a number
        .unique() // remove duplicate numbers
        .sum()
}

fn do_file(name: &str) -> u32 {
    let input = fs::read_to_string(name).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

#[cfg(test)]
mod tests {
    use crate::do_file;

    #[test]
    fn example() {
        assert_eq!(do_file("./example.txt"), 4361);
    }
}
