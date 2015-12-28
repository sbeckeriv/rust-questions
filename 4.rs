use std::io;

fn isPrime(x: i32) -> bool {
    // Write the implementation here.
    // Note: You will have to optimize the solution to pass all the test cases
    unimplemented!();
}

fn main() {
    // Declare the variables
    let mut line1 = String::new();
    let mut line2 = String::new();

    // Read the variables
    io::stdin().read_line(&mut line1).ok().expect("read error");
    io::stdin().read_line(&mut line2).ok().expect("read error");

    // parse integers
    let mut a: i32 = line1.trim().parse().ok().expect("parse error");
    let mut b: i32 = line2.trim().parse().ok().expect("parse error");

    while a <= b {
        if isPrime(a) {
            println!("{}", a);
        }
        a = a + 1;
    }
}
