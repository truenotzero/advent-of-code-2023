use std::{collections::HashSet, fmt::Display};


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

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Start,
    Ground,
    Pipe(Pipe),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "S"),
            Tile::Ground => write!(f, "."),
            Tile::Pipe(p) => write!(f, "{}", p),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error=char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile as T;
        match value {
            'S' => Ok(T::Start),
            '.' => Ok(T::Ground),
            e => Pipe::try_from(e).map(|p| T::Pipe(p)),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Pipe {
    Horizontal, // -
    Vertical, // |
    NorthEast, // L
    NorthWest, // J
    SouthEast, // F
    SouthWest // 7
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Horizontal => write!(f, "-"),
            Pipe::Vertical   => write!(f, "|"),
            Pipe::NorthEast  => write!(f, "L"),
            Pipe::NorthWest  => write!(f, "J"),
            Pipe::SouthEast  => write!(f, "F"),
            Pipe::SouthWest  => write!(f, "7"),
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error=char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Pipe as P;
        match value {
            '-' => Ok(P::Horizontal),
            '|' => Ok(P::Vertical),
            'L' => Ok(P::NorthEast),
            'J' => Ok(P::NorthWest),
            'F' => Ok(P::SouthEast),
            '7' => Ok(P::SouthWest),
            e => Err(e),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        use Direction as D;
        [D::North, D::South, D::East, D::West]
    }

    fn rev(self) -> Direction {
        use Direction as D;
        match self {
            D::North => D::South,
            D::South => D::North,
            D::East => D::West,
            D::West => D::East,
        }
    }
}

impl From<Direction> for Coords {
    fn from(value: Direction) -> Self {
        use Direction as D;
        match value {
            D::North => (-1, 0),
            D::South => (1, 0),
            D::East => (0, 1),
            D::West => (0, -1),
        }
    }
}

impl Pipe {
    fn all() -> [Pipe; 6] {
        use Pipe as P;
        [P::Horizontal, P::Vertical, P::NorthEast,
        P::NorthWest, P::SouthEast, P::SouthWest]
    }

    fn connects_at(self) -> [Direction; 2] {
        use Pipe as P;
        use Direction as D;
        match self {
            P::Horizontal => [D::East, D::West],
            P::Vertical => [D::North, D::South],
            P::NorthEast => [D::North, D::East],
            P::NorthWest => [D::North, D::West],
            P::SouthEast => [D::South, D::East],
            P::SouthWest => [D::South, D::West],
        }
    }

    // fn can_connect(self, other: Pipe, direction: Direction) -> bool {
    //     self.connects_at().contains(&direction)
    //     && other.connects_at().contains(&direction.rev())
    // }
}

type World = Vec<Vec<Tile>>;    // 0,0 is top left
type Coords = (Num, Num);       //  y,x ordering

fn find_loop_start(world: &World) -> Coords {
    for (y, rank) in world.iter().enumerate() {
        for (x, tile) in rank.iter().enumerate() {
            if Tile::Start.eq(tile) {
                return (y as _, x as _)
            }
        }
    }

    panic!("Start not found")
}

fn step(world: &World, location: Coords, came_from: Direction) -> (Coords, Direction) {
    let tile = world[location.0 as usize][location.1 as usize];
    if let Tile::Pipe(pipe) = tile {
        let direction = {
            let dirs = pipe.connects_at();
            if dirs[0] == came_from.rev() {
                dirs[1]
            } else {
                dirs[0]
            }
        };
        let delta: Coords = direction.into();
        let location = (location.0 + delta.0, location.1 + delta.1);

        (location, direction)
    } else { panic!() }
}

fn get_loop(start: Coords, world: &World) -> (Pipe, HashSet<Coords>) {
    // find first step location + direction
    let (mut yx, mut dir) = Direction::all()
        .into_iter()
        .find_map(|dir| {
            let coords: Coords = dir.into();
            let y = start.0 + coords.0;
            let x =  start.1 + coords.1;
            let adj = world[y as usize][x as usize];

            if let Tile::Pipe(pipe) = adj {
                if pipe.connects_at().contains(&dir.rev()) {
                    return Some(((y, x), dir))
                }
            }
            None
        })
        .expect("No steppable direction adjacent starting coord")
        ;
    
    let init_dir = dir;
    let mut coords = HashSet::new();
    loop {
        coords.insert(yx);
        if yx == start { 
            let mut dirs = [init_dir, dir.rev()];
            dirs.sort();

            let pipe = Pipe::all()
                .into_iter()
                .find_map(|p| {
                    let mut ret = p.connects_at();
                    ret.sort();

                    if ret == dirs {
                        Some(p)
                    } else {
                        None
                    }
                })
                .unwrap()
                ;

            break (pipe, coords);
        }
        (yx, dir) = step(world, yx, dir);
    }
}

fn raycast(world: &World, (y, x): (usize, usize)) -> Num {
    let row = &world[y];

    let mut hits = 0;
    for tile in &row[x..] {
        if let Tile::Pipe(p) = tile {
            // any pipe with a vertical element counts as a hit
            // assume the pipe is *just* a tiny bit below to avoid 
            // it being in line with a horizontal vertex
            // that means that J and L also count as horizontal
            match p {
                Pipe::Horizontal | Pipe::NorthEast | Pipe::NorthWest => (),
                _ => hits += 1,
            }
        }
    }
    hits
}

fn cleanup(world: &mut World, the_loop: HashSet<Coords>, new_start: Pipe) {
    for (y, row) in world.iter_mut().enumerate() {
        for (x, e) in row.iter_mut().enumerate() {
            match e {
                Tile::Start => *e = Tile::Pipe(new_start),
                Tile::Pipe(_) => if !the_loop.contains(&(y as _ ,x as _)) {
                    *e = Tile::Ground;
                },
                Tile::Ground => continue,
            }
        }
    }
}

fn process(input: &str) -> Num {
    let mut world = input.lines()
                    .map(|m| m.chars().map(|e| Tile::try_from(e).unwrap()).collect::<Vec<_>>())
                    .collect::<Vec<_>>();

    let start = find_loop_start(&world);
    let (new_start, the_loop) = get_loop(start, &world);
    cleanup(&mut world, the_loop, new_start);

    let mut ground_tiles_inside_loop = 0;
    for (y, row) in world.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            print!("{p}");
            if let Tile::Ground = p {
                // using the raycasting algorithm
                // if the number of hits with the polygon's borders
                // is odd, then the point lies in the polygon
                let result = raycast(&world, (y, x));
                if result % 2 != 0 {
                    ground_tiles_inside_loop += 1;
                }
            }
        }
        println!();
    }

    ground_tiles_inside_loop
}

#[cfg(test)]
mod tests {
    #[test]
    fn example3() {
        assert_eq!(crate::do_file("./example3.txt"), 4);
    }

    #[test]
    fn example4() {
        // first try!
        assert_eq!(crate::do_file("./example4.txt"), 8);
    }
}
