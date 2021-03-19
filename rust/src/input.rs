use crate::domain::{Coordinate, Instructions, Mars, Rover};
use crate::error::AppError;
use std::io;
use std::str::FromStr;

pub fn get_mars(reader: &mut dyn io::BufRead) -> Result<Mars, AppError> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Err(_e) => return Err(AppError::IOReadError),
        Ok(_n) => {}
    }
    let mut text = buf.trim().split_whitespace();
    let x = parse(text.next())?;
    let y = parse(text.next())?;
    Ok(Mars::new(Coordinate::new(x, y)))
}

fn parse<T: FromStr>(text: Option<&str>) -> Result<T, AppError> {
    let x: T = match text {
        None => return Err(AppError::BadInput),
        Some(t) => match t.parse::<T>() {
            Ok(x) => x,
            Err(_e) => return Err(AppError::BadInput),
        },
    };
    Ok(x)
}

pub fn get_instructions(reader: &mut dyn io::BufRead) -> Result<Vec<Instructions>, AppError> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Err(_e) => return Err(AppError::IOReadError),
        Ok(_n) => {}
    }
    let mut instructions = Vec::new();
    let buf = buf.trim();
    for c in buf.chars() {
        let i = match c.to_string().parse::<Instructions>() {
            Ok(i) => i,
            Err(e) => return Err(e),
        };
        instructions.push(i);
    }

    Ok(instructions)
}

pub fn get_rover(reader: &mut dyn io::BufRead) -> Result<Rover, AppError> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Err(_e) => return Err(AppError::IOReadError),
        Ok(_n) => {}
    }
    let mut text = buf.trim().split_whitespace();
    let x = parse(text.next())?;
    let y = parse(text.next())?;
    let o = parse(text.next())?;
    Ok(Rover::new(Coordinate::new(x, y), o))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Orientation;

    #[test]
    fn test_get_mars_good_input_for_get_mars() {
        let mut text = "5 1\nignore".as_bytes();
        let actual = get_mars(&mut text).unwrap();
        assert_eq!(actual, Mars::new(Coordinate::new(5, 1)),);
    }

    #[test]
    fn test_get_mars_ioerror() {
        struct Broken;
        impl io::Read for Broken {
            fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
            }
        }
        let mut b = io::BufReader::new(Broken);
        assert!(get_mars(&mut b).is_err());
        assert_eq!(AppError::IOReadError, get_mars(&mut b).unwrap_err());
    }

    #[test]
    fn test_get_mars_bad_x_coordinates() {
        let mut text = "a 1\nignore".as_bytes();
        let actual = get_mars(&mut text);
        assert!(actual.is_err());
        assert_eq!(AppError::BadInput, actual.unwrap_err());
    }

    #[test]
    fn test_get_mars_bad_y_coordinates() {
        let mut text = "1 a\nignore".as_bytes();
        let actual = get_mars(&mut text);
        assert!(actual.is_err());
        assert_eq!(AppError::BadInput, actual.unwrap_err());
    }

    #[test]
    fn test_get_rover_valid() {
        let mut text = "10 20 E\nignore".as_bytes();
        let actual = get_rover(&mut text).unwrap();
        assert_eq!(
            actual,
            Rover {
                location: Coordinate::new(10, 20),
                orientation: Orientation::E,
            }
        );
    }

    #[test]
    fn test_get_rover_ioerror() {
        struct Broken;
        impl io::Read for Broken {
            fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
            }
        }
        let mut b = io::BufReader::new(Broken);
        assert!(get_mars(&mut b).is_err());
        assert_eq!(AppError::IOReadError, get_rover(&mut b).unwrap_err());
    }

    #[test]
    fn test_get_rover_bad_x_coordinates() {
        let mut text = "a 1\nignore".as_bytes();
        let actual = get_rover(&mut text);
        assert!(actual.is_err());
        assert_eq!(AppError::BadInput, actual.unwrap_err());
    }

    #[test]
    fn test_get_rover_bad_y_coordinates() {
        let mut text = "1 a\nignore".as_bytes();
        let actual = get_rover(&mut text);
        assert!(actual.is_err());
        assert_eq!(AppError::BadInput, actual.unwrap_err());
    }
    #[test]
    fn test_get_rover_bad_orientation() {
        let mut text = "1 1 a\nignore".as_bytes();
        let actual = get_rover(&mut text);
        assert!(actual.is_err());
        assert_eq!(AppError::BadInput, actual.unwrap_err());
    }

    #[test]
    fn test_get_instructions_valid() {
        let mut text = "RFLLFR\nignore".as_bytes();
        let actual = get_instructions(&mut text).unwrap();
        assert_eq!(
            actual,
            vec![
                Instructions::R,
                Instructions::F,
                Instructions::L,
                Instructions::L,
                Instructions::F,
                Instructions::R,
            ]
        );
    }
}
