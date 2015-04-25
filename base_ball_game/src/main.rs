use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("Input Baseball number : ");
    io::stdout().flush();

    io::stdin().read_line(&mut input);
    print!("input data : {}", input);
}
