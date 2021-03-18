use crate::error::AppError;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}

#[derive(PartialEq, Debug)]
pub struct Mars {
    pub upper_right: (i64, i64),
}

impl Mars {
    pub fn new(co: Coordinate) -> Mars {
        Mars {
            upper_right: (co.x, co.y),
        }
    }
    pub fn move_rover(&self, r: Rover, instruction: Instructions) -> Result<Rover, AppError> {
        let mut res = r.clone();
        match instruction {
            Instructions::F => match r.orientation {
                Orientation::N => res.location.1 = r.location.1 + 1,
                Orientation::E => res.location.0 = r.location.0 + 1,
                Orientation::S => res.location.1 = r.location.1 - 1,
                Orientation::W => res.location.0 = r.location.0 - 1,
            },
            _ => return Err(AppError::BadCoordinates),
        }
        Ok(res)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Rover {
    pub location: (i64, i64),
    pub orientation: Orientation,
}

impl Rover {
    pub fn new(loc: Coordinate, o: Orientation) -> Rover {
        Rover {
            location: (loc.x, loc.y),
            orientation: o,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Orientation {
    N,
    W,
    S,
    E,
}

impl FromStr for Orientation {
    type Err = ();
    fn from_str(input: &str) -> Result<Orientation, Self::Err> {
        match input {
            "N" => Ok(Orientation::N),
            "W" => Ok(Orientation::W),
            "S" => Ok(Orientation::S),
            "E" => Ok(Orientation::E),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Instructions {
    L,
    R,
    F,
}

impl FromStr for Instructions {
    type Err = ();
    fn from_str(input: &str) -> Result<Instructions, Self::Err> {
        match input {
            "L" => Ok(Instructions::L),
            "R" => Ok(Instructions::R),
            "F" => Ok(Instructions::F),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation_n() {
        assert_eq!("N".parse::<Orientation>().unwrap(), Orientation::N);
    }

    #[test]
    fn test_orientation_w() {
        assert_eq!("W".parse::<Orientation>().unwrap(), Orientation::W);
    }

    #[test]
    fn test_orientation_s() {
        assert_eq!("S".parse::<Orientation>().unwrap(), Orientation::S);
    }

    #[test]
    fn test_orientation_e() {
        assert_eq!("E".parse::<Orientation>().unwrap(), Orientation::E);
    }

    #[test]
    fn test_instructions_l() {
        assert_eq!("L".parse::<Instructions>().unwrap(), Instructions::L);
    }

    #[test]
    fn test_instructions_r() {
        assert_eq!("R".parse::<Instructions>().unwrap(), Instructions::R);
    }

    #[test]
    fn test_instructions_f() {
        assert_eq!("F".parse::<Instructions>().unwrap(), Instructions::F);
    }

    #[test]
    fn test_mars_move_rover_forward_n() {
        let m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::N);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(3, 4), Orientation::N));
    }

    #[test]
    fn test_mars_move_rover_forward_e() {
        let m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::E);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(4, 3), Orientation::E));
    }

    #[test]
    fn test_mars_move_rover_forward_s() {
        let m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::S);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(3, 2), Orientation::S));
    }

    #[test]
    fn test_mars_move_rover_forward_w() {
        let m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::W);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(2, 3), Orientation::W));
    }
}
