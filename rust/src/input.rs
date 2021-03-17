use crate::error::AppError;
use crate::mars::Mars;
use std::io;

pub fn get_mars(reader: &mut dyn io::BufRead) -> Result<Mars, AppError> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Err(e) => return Err(AppError::IOReadError),
        Ok(n) => println!("Read {} bytes", n),
    }
    let mut text = buf.trim().split_whitespace();
    let x = match parse(text.next()) {
        Ok(x) => x,
        Err(e) => return Err(AppError::BadCoordinates),
    };
    let y = match parse(text.next()) {
        Ok(x) => x,
        Err(e) => return Err(AppError::BadCoordinates),
    };
    Ok(Mars {
        upper_right: (x, y),
    })
}

fn parse(text: Option<&str>) -> Result<i64, String> {
    let x: i64 = match text {
        None => return Err(String::from("no")),
        Some(t) => match t.parse::<i64>() {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        },
    };
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_input_for_get_mars() {
        let mut text = "5 1\nignore".as_bytes();
        let actual = get_mars(&mut text).unwrap();
        assert_eq!(
            actual,
            Mars {
                upper_right: (5, 1)
            }
        );
    }

    #[test]
    fn test_ioerror() {
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
    fn test_bad_x_coordinates() {
        let mut text = "a 1\nignore".as_bytes();
        let actual = get_mars(&mut text);
        assert!(actual.is_err());
        assert_eq!(AppError::BadCoordinates, actual.unwrap_err());
    }

    #[test]
    fn test_bad_y_coordinates() {
        let mut text = "1 a\nignore".as_bytes();
        let actual = get_mars(&mut text);
        assert!(actual.is_err());
        assert_eq!(AppError::BadCoordinates, actual.unwrap_err());
    }
}
