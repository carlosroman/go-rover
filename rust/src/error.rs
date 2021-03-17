use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum AppError {
    #[error("IO Read Error")]
    IOReadError,
    #[error("Bad Coordinates")]
    BadCoordinates,
    #[error("Bad Orientation")]
    BadOrientation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_ioerroar() {
        println!("Error: {:?}", AppError::IOReadError);
        assert_eq!("IO Read Error", AppError::IOReadError.to_string());
    }

    #[test]
    fn test_app_bad_coordinates() {
        println!("Error: {:?}", AppError::BadCoordinates);
        assert_eq!("Bad Coordinates", AppError::BadCoordinates.to_string());
    }

    #[test]
    fn test_app_bad_orientation() {
        println!("Error: {:?}", AppError::BadOrientation);
        assert_eq!("Bad Orientation", AppError::BadOrientation.to_string());
    }
}
