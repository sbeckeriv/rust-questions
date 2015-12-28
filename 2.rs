// Rust program to read an integer from STDIN and output it to STDOUT
use std::io;

fn main() {
    // Declare the variable
    let mut line1 = String::new();

    // Read the variable from STDIN
    io::stdin().read_line(&mut line1).ok().expect("read error");

    // parse integer
    let mut a: i32 = line1.trim().parse().ok().expect("parse error");

    // Output the variable to STDOUT
    println!("{}", a);

}
