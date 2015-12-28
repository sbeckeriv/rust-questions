use std::io;

fn most_occurring_word(s: String) -> String {
    // Write the implementation here.
    unimplemented!();
}

fn main() {
    // Declare the variables
    let mut paragraph = String::new();

    // Read the variables
    io::stdin().read_line(&mut paragraph).ok().expect("read error");
    println!("{}", most_occurring_word(paragraph));
}
