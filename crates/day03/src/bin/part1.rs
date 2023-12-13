use std::collections::HashSet;


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

type Schematic = Vec<Vec<char>>;
type Coords = (Num, Num); // yx

fn find_symbols(schematic: &Schematic) -> Vec<Coords> {
    let mut ret = Vec::new();
    for (y, row) in schematic.iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if '.'.ne(e) && !e.is_digit(10) {
                ret.push((y as _, x as _))
            }
        }
    }
    ret
}

fn adjacents(yx: Coords, bounds: Coords) -> Vec<Coords> {
    let mut ret = Vec::new();
    let offsets = -1..=1;
    for dy in offsets.clone() {
        let y = yx.0 + dy;
        if y < 0 || y >= bounds.0 { continue }
        for dx in offsets.clone() {
            if dx == 0 && dy == 0 { continue }
            let x = yx.1 + dx;
            if x < 0 || x >= bounds.1 { continue }
            ret.push((y, x));
        }
    }
    ret
}

fn try_parse_num(yx: Coords, schematic: &Schematic) -> Option<(Num, Coords)> {
    if !schematic[yx.0 as usize][yx.1 as usize].is_digit(10) { return None }
    let start = {
        let mut x = yx.1;
        let y = yx.0;
        loop {
            // break before non-digit idx
            let nx = x - 1;
            if nx < 0 { break }
            let next = schematic[y as usize][nx as usize];
            if !next.is_digit(10) { break }
            x = nx;
        }
        x
    };

    let mut digits = Vec::new();
    for e in schematic[yx.0 as usize][(start as usize)..].iter() {
        if let Some(digit) = e.to_digit(10) {
            digits.push(digit as _);
        } else {
            break;
        }
    }
    
    let ret = digits.iter().fold(0, |acc, e| 10 * acc + e);
    Some((ret, (yx.0, start)))
}

fn process(input: &str) -> Num {
    let schematic = input.lines().map(|e| e.chars().collect()).collect();
    find_symbols(&schematic).into_iter()
        .flat_map(|symbol| adjacents(symbol, (schematic.len() as _, schematic[0].len() as _)))
        .filter_map(|adj| try_parse_num(adj, &schematic))
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|(num, _yx)| num)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 4361);
    }
}
