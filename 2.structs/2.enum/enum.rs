// un enum è simile ad uno Struct: può avere i suoi metodi e i suoi trait
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self { // *self has type Direction
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right"
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}



fn main() {
    let start = Direction::Left;

    let mut d = start;
    for _ in 0..8 {
        println!("d {}", d.as_str());
        d = d.next();
    }
}