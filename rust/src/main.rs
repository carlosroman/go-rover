mod input;
mod mars;

fn main() {
    println!("Hello, world!");
    let mut bs = "uno".as_bytes();
    println!("{:?}", input::get_mars(&mut bs));
}
