mod domain;
mod error;
mod input;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut reader = Box::new(stdin.lock()) as Box<dyn io::BufRead>;
    let stout = io::stdout();
    let mut writer = Box::new(stout.lock());
    let mut mars = input::get_mars(&mut reader).unwrap();
    loop {
        mars = run_process(mars, &mut reader, &mut writer);
    }
}

fn run_process(
    mut mars: domain::Mars,
    reader: &mut dyn io::BufRead,
    writer: &mut impl io::Write,
) -> domain::Mars {
    let mut lost = false;
    let mut rover = input::get_rover(reader).unwrap();
    for i in input::get_instructions(reader).unwrap() {
        rover = match mars.move_rover(rover.clone(), i) {
            Ok(r) => r,
            Err(_) => {
                lost = true;
                break;
            }
        };
    }
    let orien = rover.orientation.clone();
    let loc = rover.location;
    let msg;
    if lost {
        msg = format!("{} {} {:?} LOST\n", loc.x, loc.y, orien);
    } else {
        msg = format!("{} {} {:?}\n", loc.x, loc.y, orien);
    }
    writer.write_all(msg.as_bytes()).unwrap();
    mars
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_run_process() {
        // GIVEN 5 5 Mars
        let mars = domain::Mars::new(domain::Coordinate::new(5, 5));
        // AND Rover starts at 3 3 N
        let start = String::from("3 3 N");
        // WHEN Rover moved FFRFFF
        let instructions = String::from("FFRFFF");
        // THEN expect 3 5 N LOST
        let input = format!("{}\n{}\n", start, instructions);
        let buf = Vec::new();
        let mut writer = io::BufWriter::new(buf);
        run_process(mars, &mut input.as_bytes(), &mut writer);
        let actual = str::from_utf8(writer.buffer()).unwrap();
        assert_eq!("5 5 E LOST\n", actual);
    }
}
