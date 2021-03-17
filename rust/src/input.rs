use crate::mars::Mars;
use std::io;

pub fn get_mars(reader: &mut dyn io::BufRead) -> Result<Mars, String> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Err(e) => return Err(e.to_string()),
        Ok(n) => println!("Read {} bytes", n),
    }
    let mut text = buf.trim().split_whitespace();
    let x = match parse(text.next()) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let y = match parse(text.next()) {
        Ok(x) => x,
        Err(e) => return Err(e),
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
trait ReadLine {
    fn read_line(buf: &mut String) -> io::Result<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_input_for_get_mars() {
        let mut text = "5 1\nignore".as_bytes();
        assert_eq!(
            get_mars(&mut text),
            Ok(Mars {
                upper_right: (5, 1)
            })
        );
    }
}
