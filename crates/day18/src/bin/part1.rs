use std::ops::Add;


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

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [Self::Up, Self::Left, Self::Right, Self::Down]
    }

    fn as_delta(self) -> Pos {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }.into()
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Bad direction"),
        }
    }
}

#[derive(Clone, Copy)]
struct Command {
    direction: Direction,
    times: Num,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let mut it = value.split(' ');
        let direction = it.next().expect("No direction").chars().next().unwrap().into();
        let times = Num::from_str_radix(it.next().expect("No repetition"), 10).expect("Can't parse repetition");

        Self {
            direction,
            times,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Pos {
    y: Num,
    x: Num,
}

impl From<(Num, Num)> for Pos {
    fn from(value: (Num, Num)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

impl From<Pos> for (Num, Num) {
    fn from(value: Pos) -> Self {
        (value.y, value.x)
    }
}

impl Add for Pos {
    type Output=Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Default)]
struct Excavator {
    pos: Pos,
    vertices: Vec<Pos>,
    directions: Vec<Direction>,
    area: Num,
}

impl Excavator {
    fn execute(mut self, cmd: Command) -> Self {
        let Pos { y, x } = cmd.direction.as_delta();

        // push vertex and direction to next vertex
        self.vertices.push(self.pos);
        self.directions.push(cmd.direction);

        // update position
        self.pos.x += x * cmd.times;
        self.pos.y += y * cmd.times;

        // update area (which is just the perimeter at this stage)
        self.area += cmd.times;

        if self.vertices.len() > 1 && self.pos == (0, 0).into() {
            // finished drawing
            // so, for commodity:
            self.vertices.push(self.vertices[0]);
            self.directions.push(self.directions[0]);
        }

        self
    }

    fn fill(mut self) -> Self {
        let seed = self.find_seed();
        // seed assumes: dirt + inside

        let mut visited = Vec::new();
        let mut stack = Vec::new();
        stack.push(seed);
        
        loop {
            if let Some(pos) = stack.pop() {
                // dig current block and mark as visited
                self.area += 1;
                visited.push(pos);
                // for each surrounding tile
                // check that we haven't been there
                // and then make sure it's inside the poly
                // then add them to the stack so they all get checked
                Direction::all()
                    .into_iter()
                    .map(Direction::as_delta)
                    .map(|d| pos + d)
                    .filter(|&new_pos| !visited.contains(&new_pos) && self.is_inside_poly(new_pos))
                    .for_each(|new_pos| {
                        if !stack.contains(&new_pos) {
                            stack.push(new_pos)
                        }
                    })
                    ;
            } else { break self }
        }
    }

    fn find_seed(&self) -> Pos {
        loop {
            let dirt = self.find_dirt_pos();
            if self.is_inside_poly(dirt) {
                break dirt;
            }
        }
    }

    fn find_dirt_pos(&self) -> Pos {
        (1, 1).into()
    }

    // check if inside the polygon by casting a ray
    fn is_inside_poly(&self, pos: Pos) -> bool {
        let mut hits = 0;
        for v1v2 in self.vertices.windows(2) {
            let (y1, x1) = v1v2[0].into();
            let (y2, x2) = v1v2[1].into();

            // the ray is cast to the right
            // (so if the point is to the right of both vertices we can skip)
            if pos.x > x1.max(x2) { continue }
            // the ray 'rides' on the top side of the tile
            // now check that it actually intersects the vertex
            // (this is done to prevent a bug that happens when the ray passes through both vertices)
            if y1.min(y2) < pos.y && pos.y <= y1.max(y2) {
                hits += 1
            }
        }

        hits % 2 != 0
    }
}

fn process(input: &str) -> Num {
    input
        .lines()
        .map(Command::from)
        .fold(Excavator::default(), |ex, cmd| ex.execute(cmd)) // dig the outline
        .fill()
        .area
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 62);
    }
}
