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


// posso definirli anche nello stile C, i valori seguenti si incrementano di 1
// de devo confrontarli e ordinare devo aggiungere i trait
#[derive(PartialEq,PartialOrd)]
enum Speed {
    Slow = 1,
    Medium,
    Fast,
}



fn main() {
    let start = Direction::Left;
    let mut d = start;
    for _ in 0..8 {
        println!("d {}", d.as_str());
        d = d.next();
    }

    // siccome questo sono numero, posso fare il cast a u32
    let turtle = Speed::Slow;
    let speed = turtle as u32;
    println!("speed {}", speed);

    // ordinamento
    let hyppo = Speed::Medium;
    let rabbit = Speed::Fast;
    if rabbit > hyppo {
        println!("Rabbit is faster than hyppo");
    }
}