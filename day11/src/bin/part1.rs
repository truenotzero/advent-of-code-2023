
fn main() {
    // first try!
    do_file("./input.txt");
}

type Num = i32;

fn do_file(filename: &str) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
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
    for row in universe.iter_mut() {
        for c in &columns {
            row.insert(*c, '.');
        }
    }
    // rows
    let column_len = universe[0].len();
    let empty_column = vec!['.';column_len];
    for r in &rows {
        universe.insert(*r, empty_column.clone())
    }
}

fn find_galaxies(universe: Universe) -> Vec<Coords> {
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

fn distance(yx1: Coords, yx2: Coords) -> Num {
    let dy = yx1.0 - yx2.0;
    let dx = yx1.1 - yx2.1;
    (dy.abs() + dx.abs()) as _
}

fn process(input: &str) -> Num {
    let mut universe = input.lines().map(|e| e.chars().collect()).collect();
    expand(&mut universe);
    
    let galaxies = find_galaxies(universe);
    pairs(galaxies).into_iter()
        .map(|(a, b)| distance(a,b))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        // first try!
        assert_eq!(crate::do_file("./example.txt"), 374);
    }
}
