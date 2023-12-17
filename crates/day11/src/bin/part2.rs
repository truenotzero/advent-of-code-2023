
fn main() {
    // first try!
    do_file("./input.txt", 1_000_000);
}

type Num = i64;

fn do_file(filename: &str, factor: Num) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input, factor);
    println!("{output}");
    output
}

type Universe = Vec<Vec<char>>;
type Coords = (isize, isize); //yx

fn expand(universe: &mut Universe) {
    // first check columns
    let columns = {
        let mut ret = Vec::new();
        for x in 0..universe[0].len() {
            let mut galaxy_found = false;
            for y in 0..universe.len() {
                if universe[y][x] == '#' {
                    galaxy_found = true;
                    break;
                }
            }
            if !galaxy_found {
                ret.push(x);
            }
        }
        ret.reverse();
        ret
    };
    // then check rows
    let rows = universe.iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.contains(&'#') { None } else { Some(idx) }
        })
        .rev()
        .collect::<Vec<_>>();

    // now expand
    // columns
    for y in rows {
        for x in 0..universe[0].len() {
            universe[y][x] = 'X';
        }
    }
   // rows
   for y in 0..universe.len() {
    for &x in &columns {
        universe[y][x] = 'X';
    }
   }
}

fn find_galaxies(universe: &Universe) -> Vec<Coords> {
    let mut ret = Vec::new();
    for (y, row) in universe.iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if '#'.eq(e) {
                ret.push((y as _, x as _));
            }
        }
    }
    ret
}

fn pairs<T: Clone + PartialEq>(vec: Vec<T>) -> Vec<(T, T)> {
    let mut ret = Vec::new();
    for (idx, e) in vec.iter().enumerate() {
        for f in vec.iter().skip(idx+1) { // skip to one-past current element (e)
            ret.push((e.clone(), f.clone()));
        }
    }
    ret
}

fn distance(universe: &Universe, factor: Num, yx1: Coords, yx2: Coords) -> Num {
    let y_start = 1 + yx1.0.min(yx2.0);
    let y_end = 1 + yx1.0.max(yx2.0);
    let mut y_dist = 0;
    for y in y_start..y_end {
        match universe[y as usize][0] {
            'X' => y_dist += factor,
            _ => y_dist += 1,
        }
    }

    let x_start = 1 + yx1.1.min(yx2.1);
    let x_end = 1 + yx1.1.max(yx2.1);
    let mut x_dist = 0;
    for x in x_start..x_end {
        match universe[0][x as usize] {
            'X' => x_dist += factor,
            _ => y_dist += 1,
        }
    }

    y_dist + x_dist
}

fn process(input: &str, factor: Num) -> Num {
    let mut universe = input.lines().map(|e| e.chars().collect()).collect();
    expand(&mut universe);
    
    let galaxies = find_galaxies(&universe);
    pairs(galaxies).into_iter()
        .map(|(a, b)| distance(&universe, factor,a,b))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_factor2() {
        assert_eq!(crate::do_file("./example.txt", 2), 374);
    }

    #[test]
    fn example_factor10() {
        // first try!
        assert_eq!(crate::do_file("./example.txt", 10), 1030);
    }

    #[test]
    fn example_factor100() {
        // first try!
        assert_eq!(crate::do_file("./example.txt", 100), 8410);
    }
}
