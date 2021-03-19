use crate::error::AppError;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(PartialEq, Debug)]
pub struct Mars {
    pub upper_right: Coordinate,
    scent_map: BTreeMap<Coordinate, bool>,
}

impl Mars {
    pub fn new(co: Coordinate) -> Mars {
        Mars {
            upper_right: co,
            scent_map: BTreeMap::new(),
        }
    }
    pub fn move_rover(&mut self, r: Rover, instruction: Instructions) -> Result<Rover, AppError> {
        let mut res = r.clone();
        match instruction {
            Instructions::F => match r.orientation {
                Orientation::N => res.location.y = r.location.y + 1,
                Orientation::E => res.location.x = r.location.x + 1,
                Orientation::S => res.location.y = r.location.y - 1,
                Orientation::W => res.location.x = r.location.x - 1,
            },
            Instructions::L => res.orientation = r.orientation.rotate_left(),
            Instructions::R => res.orientation = r.orientation.rotate_right(),
        }
        if res.location.x <= self.upper_right.x && res.location.y <= self.upper_right.y {
            return Ok(res);
        }
        match self.scent_map.insert(r.location.clone(), true) {
            None => Err(AppError::RoverLost),
            _ => Ok(r),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Rover {
    pub location: Coordinate,
    pub orientation: Orientation,
}

impl Rover {
    pub fn new(loc: Coordinate, o: Orientation) -> Rover {
        Rover {
            location: loc,
            orientation: o,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Orientation {
    N,
    E,
    S,
    W,
}

impl Orientation {
    fn rotate_left(&self) -> Orientation {
        match self {
            Orientation::N => Orientation::W,
            Orientation::E => Orientation::N,
            Orientation::S => Orientation::E,
            Orientation::W => Orientation::S,
        }
    }
    fn rotate_right(&self) -> Orientation {
        match self {
            Orientation::N => Orientation::E,
            Orientation::E => Orientation::S,
            Orientation::S => Orientation::W,
            Orientation::W => Orientation::N,
        }
    }
}

impl FromStr for Orientation {
    type Err = AppError;
    fn from_str(input: &str) -> Result<Orientation, AppError> {
        match input {
            "N" => Ok(Orientation::N),
            "W" => Ok(Orientation::W),
            "S" => Ok(Orientation::S),
            "E" => Ok(Orientation::E),
            _ => Err(AppError::BadOrientation),
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
    type Err = AppError;
    fn from_str(input: &str) -> Result<Instructions, AppError> {
        match input {
            "L" => Ok(Instructions::L),
            "R" => Ok(Instructions::R),
            "F" => Ok(Instructions::F),
            _ => Err(AppError::BadInput),
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
        let mut m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::N);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(3, 4), Orientation::N));
    }

    #[test]
    fn test_mars_move_rover_forward_e() {
        let mut m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::E);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(4, 3), Orientation::E));
    }

    #[test]
    fn test_mars_move_rover_forward_s() {
        let mut m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::S);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(3, 2), Orientation::S));
    }

    #[test]
    fn test_mars_move_rover_forward_w() {
        let mut m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::W);
        let actual = m.move_rover(r, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(2, 3), Orientation::W));
    }

    #[test]
    fn test_mars_move_rover_right() {
        let mut m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::N);
        let actual = m.move_rover(r, Instructions::R).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(3, 3), Orientation::E));
    }

    #[test]
    fn test_mars_move_rover_left() {
        let mut m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(3, 3), Orientation::N);
        let actual = m.move_rover(r, Instructions::L).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(3, 3), Orientation::W));
    }

    #[test]
    fn test_mars_move_forward_fails() {
        let mut m = Mars::new(Coordinate::new(5, 5));
        let r = Rover::new(Coordinate::new(5, 5), Orientation::N);
        let actual = m.move_rover(r, Instructions::F);
        assert!(actual.is_err());
        assert_eq!(AppError::RoverLost, actual.unwrap_err());
    }

    #[test]
    fn test_mars_move_forward_over_scent() {
        // Given mars
        let mut m = Mars::new(Coordinate::new(5, 5));
        // And rover on edge
        let r1 = Rover::new(Coordinate::new(5, 5), Orientation::N);

        // When rover moves off map
        let _ignore = m.move_rover(r1, Instructions::F);

        // Then next time rover moves off map last postion returned
        let r2 = Rover::new(Coordinate::new(5, 5), Orientation::N);
        let actual = m.move_rover(r2, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(5, 5), Orientation::N));

        // And even if move repeated
        let r3 = Rover::new(Coordinate::new(5, 5), Orientation::N);
        let actual = m.move_rover(r3, Instructions::F).unwrap();
        assert_eq!(actual, Rover::new(Coordinate::new(5, 5), Orientation::N));
    }
}
