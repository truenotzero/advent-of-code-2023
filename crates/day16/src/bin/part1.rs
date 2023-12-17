use std::collections::HashSet;


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

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Beam {
    Up,
    Down,
    Left,
    Right,
}

impl Beam {
    fn step(self, yx: (Num, Num)) -> (Num, Num) {
        let delta = match self {
            Beam::Up => (-1, 0),
            Beam::Down => (1, 0),
            Beam::Left => (0, -1),
            Beam::Right => (0, 1),
        };

        (yx.0 + delta.0, yx.1 + delta.1)
    }
}

enum BeamResult {
    Single(Beam),
    Split(Beam, Beam),
}

impl Beam {
    fn pass(self) -> BeamResult {
        BeamResult::Single(self)
    }

    fn split(self) -> BeamResult {
        use Beam as B;
        use BeamResult as R;
        match self {
            B::Up | B::Down => R::Split(B::Left, B::Right),
            B::Left | B::Right => R::Split(B::Up, B::Down),
        }
    }

    fn rot_cw(self) -> BeamResult {
        use Beam as B;
        use BeamResult as R;
        match self {
            B::Up => R::Single(B::Right),
            B::Right => R::Single(B::Down),
            B::Down => R::Single(B::Left),
            B::Left => R::Single(B::Up),
        }
    }

    fn rot_ccw(self) -> BeamResult {
        use Beam as B;
        use BeamResult as R;
        match self {
            B::Up => R::Single(B::Left),
            B::Left => R::Single(B::Down),
            B::Down => R::Single(B::Right),
            B::Right => R::Single(B::Up),
        }
    }
}

enum Tile {
    Empty,      // .
    HSplit,     // -
    VSplit,     // |
    FMirror,    // /
    BMirror,    // \
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile as T;
        Ok(match value {
            '.'  => T::Empty,
            '-'  => T::HSplit,
            '|'  => T::VSplit,
            '/'  => T::FMirror,
            '\\' => T::BMirror,
            t => Err(t)?,
        })
    }
}

impl Tile {
    fn apply(&self, beam: Beam) -> BeamResult {
        use BeamResult as R;
        use Beam as B;
        match self {
            Tile::Empty => R::Single(beam),
            Tile::FMirror => match beam {
                B::Right | B::Left => beam.rot_ccw(),
                B::Up | B::Down => beam.rot_cw(),   
            },
            Tile::BMirror => match beam {
                B::Right | B::Left => beam.rot_cw(),
                B::Up | B::Down => beam.rot_ccw(),   
            },
            Tile::HSplit => match beam {
                B::Left | B::Right => beam.pass(),
                B::Up | B::Down => beam.split(),
            },
            Tile::VSplit => match beam {
                B::Left | B::Right => beam.split(),
                B::Up | B::Down => beam.pass(),
            },
        }
    }
}

fn do_beam(beam: Beam, (y, x): (Num, Num), bounds: (Num, Num), array: &Vec<Vec<Tile>>, mem: &mut Vec<Vec<HashSet<Beam>>>) {
    // base case
    if mem[y as usize][x as usize].contains(&beam) { return }
    mem[y as usize][x as usize].insert(beam);

    match array[y as usize][x as usize].apply(beam) {
        BeamResult::Single(e) => {
            let (ny, nx) = e.step((y, x));
            if nx >= 0 && ny >= 0 && nx < bounds.1 && ny < bounds.1 {
                do_beam(e, (ny, nx), bounds, array, mem);
            }
        }

        BeamResult::Split(l, r) => {
            let (ly, lx) = l.step((y, x));
            let (ry, rx) = r.step((y, x));
            if lx >= 0 && ly >= 0 && lx < bounds.1 && ly < bounds.1 { 
                do_beam(l, (ly, lx), bounds, array, mem);
            }
            if rx >= 0 && ry >= 0 && rx < bounds.1 && ry < bounds.1 { 
                do_beam(r, (ry, rx), bounds, array, mem);
            }
        },
    }
}

fn process(input: &str) -> Num {
    let array = input
        .lines()
        .map(|line| line
            .chars()
            .map(|e| e.try_into().unwrap())
            .collect::<Vec<Tile>>())
        .collect::<Vec<_>>()
        ;

    let width = array[0].len();
    let height = array.len();
    let mut mem = vec![vec![HashSet::new(); width]; height];
    do_beam(Beam::Right, (0, 0), (height as _, width as _), &array, &mut mem);

    // for row in &mem {
    //     for e in row {
    //         if e.is_empty() {
    //             print!(".");
    //         } else {
    //             print!("X");
    //         }
    //     }
    //     println!();
    // }

    mem
        .into_iter()
        .flat_map(|e| e.into_iter())
        .filter(|e| !e.is_empty())
        .count()
        as _
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        // first try!
        assert_eq!(crate::do_file("./example.txt"), 46);
    }
}
