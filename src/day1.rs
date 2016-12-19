use get_input::get_input;

#[derive(Copy, Clone)]
enum Rotation {
    Left,
    Right,
}

impl Rotation {
    fn from_char(c: char) -> Option<Rotation> {
        match c {
            'L' => Some(Rotation::Left),
            'R' => Some(Rotation::Right),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(self, r: Rotation) -> Direction {
        match (r, self) {
            (Rotation::Left, Direction::North) => Direction::West,
            (Rotation::Left, Direction::East) => Direction::North,
            (Rotation::Left, Direction::South) => Direction::East,
            (Rotation::Left, Direction::West) => Direction::South,
            (Rotation::Right, Direction::North) => Direction::East,
            (Rotation::Right, Direction::East) => Direction::South,
            (Rotation::Right, Direction::South) => Direction::West,
            (Rotation::Right, Direction::West) => Direction::North,
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_by(self, dir: Direction, dist: i32) -> Position {
        match dir {
            Direction::North => Position { x: self.x, y: self.y + dist },
            Direction::East => Position { x: self.x + dist, y: self.y },
            Direction::South => Position { x: self.x, y: self.y - dist },
            Direction::West => Position { x: self.x - dist, y: self.y },
        }
    }
}

fn parse_movement(m: &str) -> (Rotation, i32) {
    let mut m = m.trim().chars();
    let rotation = m.next().expect("Could not read input");
    let rotation = Rotation::from_char(rotation).unwrap();
    let distance = m.as_str().parse::<i32>().expect("Could not read integer in input");
    (rotation, distance)
}

pub fn solve() {
    let input = get_input("assets/day/1/input");
    //let input = "R8, R4, R4, R8";
    let mut position = Position { x: 0, y: 0 };
    let mut direction = Direction::North;

    for movement in input.split(',') {
        let (rotation, distance) = parse_movement(&movement);
        direction = direction.rotate(rotation);
        position = position.move_by(direction, distance);

        // Print Logo commands to draw the path,
        // turned out the first repeated position was easy to visually spot
        // 75, -4, for a distance of 79 for part 2
        println!("{} 90 fd {} -- {}, {}",
             match rotation {
                 Rotation::Left => "lt",
                 Rotation::Right => "rt",
             },
             distance,
             position.x,
             position.y
         );

        /*
        println!("{} x: {}, y: {}, facing {}",
            movement.trim(),
            position.x,
            position.y,
            pretty_orientation(orientation),
        );
        */
    }

    println!("Distance: {}", position.x.abs() + position.y.abs());
}
