use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Mars {
    pub upper_right: (i64, i64),
}

#[derive(PartialEq, Debug)]
pub struct Rover {
    pub location: (i64, i64),
    pub orientation: Orientation,
}

#[derive(PartialEq, Debug)]
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
}
