use std::{io, io::prelude::*};

#[derive(Debug, PartialEq)]
enum CardinalDirection {
    N, S, E, W,
}

#[derive(Debug, PartialEq)]
enum RelativeDirection {
    L, R, F,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Cardinal(CardinalDirection), 
    Relative(RelativeDirection),
}

#[derive(Debug)]
struct DirectionParseError {}

impl std::str::FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Direction, DirectionParseError> {
        match s {
            "E" => Ok(Direction::Cardinal(CardinalDirection::E)),
            "W" => Ok(Direction::Cardinal(CardinalDirection::W)),
            "N" => Ok(Direction::Cardinal(CardinalDirection::N)),
            "S" => Ok(Direction::Cardinal(CardinalDirection::S)),
            "L" => Ok(Direction::Relative(RelativeDirection::L)),
            "R" => Ok(Direction::Relative(RelativeDirection::R)),
            "F" => Ok(Direction::Relative(RelativeDirection::F)),
            _ => Err(DirectionParseError {}),
        }
    }
}


fn main() {
    let stdin = io::stdin();

    let mut x: i16 = 10; // Waypoint Easting
    let mut y: i16 = 1;  // Waypoint Northing
    let mut ship_x: i32 = 0;
    let mut ship_y: i32 = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let dir: Direction = line[0..1].parse().unwrap();
        let arg: i16 = line[1..].parse().unwrap();

        let (newx, newy) = match dir {
            Direction::Cardinal(c) => match c {
                CardinalDirection::E => (x + arg, y),
                CardinalDirection::W => (x - arg, y),
                CardinalDirection::N => (x, y + arg),
                CardinalDirection::S => (x, y - arg),
            },
            Direction::Relative(r) => match (r, arg) {
                (RelativeDirection::F, _) => {ship_x += x as i32 * arg as i32; ship_y += y as i32 * arg as i32; (x, y)},
                (RelativeDirection::R, 90) | (RelativeDirection::L, 270) => (y, -x),
                (RelativeDirection::R, 180) | (RelativeDirection::L, 180) => (-x, -y),
                (RelativeDirection::R, 270) | (RelativeDirection::L, 90) => (-y, x),
                (RelativeDirection::R, _) | (RelativeDirection::L, _) => panic!("Bad rotation angle"),
            }
        };
        x = newx; y = newy;
        println!("{} {} {} {} {}", line, x, y, ship_x, ship_y);
    }
    println!("Manhattan Distance: {}", ship_x.abs() + ship_y.abs());
}
