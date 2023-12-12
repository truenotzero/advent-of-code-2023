
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

#[derive(Clone, Copy, PartialEq)]
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

fn measure_loop_len(start: Coords, world: &World) -> Num {
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
    
    let mut ret = 1;
    loop {
        if yx == start { break ret; }
        (yx, dir) = step(world, yx, dir);
        ret += 1;
    }
}

fn process(input: &str) -> Num {
    let world = input.lines()
                    .map(|m| m.chars().map(|e| e.try_into().unwrap()).collect())
                    .collect();
    // find the pipeline loop
    let start = find_loop_start(&world);
    // measure the loop
    let loop_len = measure_loop_len(start, &world);
    // apply clever math
    loop_len / 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        // first try!
        assert_eq!(crate::do_file("./example1.txt"), 4);
    }

    #[test]
    fn example2() {
        // first try!
        assert_eq!(crate::do_file("./example2.txt"), 8);
    }
}
