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

    let mut heading: CardinalDirection = CardinalDirection::E;
    let mut x: i16 = 0;  // Easting
    let mut y: i16 = 0;  // Northing

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let dir: Direction = line[0..1].parse().unwrap();
        let arg: i16 = line[1..].parse().unwrap();
        match dir {
            Direction::Cardinal(c) => match c {
                CardinalDirection::E => x += arg,
                CardinalDirection::W => x -= arg,
                CardinalDirection::N => y += arg,
                CardinalDirection::S => y -= arg,
            },
            Direction::Relative(r) => match (r, arg) {
                (RelativeDirection::F, _) => match heading {
                    CardinalDirection::E => x += arg,
                    CardinalDirection::W => x -= arg,
                    CardinalDirection::N => y += arg,
                    CardinalDirection::S => y -= arg,
                },
                (RelativeDirection::R, 90) | (RelativeDirection::L, 270) => match heading {
                    CardinalDirection::E => heading = CardinalDirection::S,
                    CardinalDirection::W => heading = CardinalDirection::N,
                    CardinalDirection::N => heading = CardinalDirection::E,
                    CardinalDirection::S => heading = CardinalDirection::W,
                },
                (RelativeDirection::R, 180) | (RelativeDirection::L, 180) => match heading {
                    CardinalDirection::E => heading = CardinalDirection::W,
                    CardinalDirection::W => heading = CardinalDirection::E,
                    CardinalDirection::N => heading = CardinalDirection::S,
                    CardinalDirection::S => heading = CardinalDirection::N,
                },
                (RelativeDirection::R, 270) | (RelativeDirection::L, 90) => match heading {
                    CardinalDirection::E => heading = CardinalDirection::N,
                    CardinalDirection::W => heading = CardinalDirection::S,
                    CardinalDirection::N => heading = CardinalDirection::W,
                    CardinalDirection::S => heading = CardinalDirection::E,
                },
                (RelativeDirection::R, _) | (RelativeDirection::L, _) => panic!("Bad rotation angle"),
            }
        }
        println!("{} {} {} {:?}", line, x, y, heading);
    }
    println!("Manhattan Distance: {}", x.abs() + y.abs());
}
