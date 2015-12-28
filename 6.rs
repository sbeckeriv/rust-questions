use std::io;

fn minGears(s: String) -> i32 {
    // Write the implementation here.
    unimplemented!();
}

fn main() {
    // Declare the variables
    let mut gears = String::new();

    // Read the variables
    io::stdin().read_line(&mut gears).ok().expect("read error");
    println!("{}", minGears(gears));
}
