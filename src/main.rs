use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Seraching for {}", query);
    println!("In file {}", filename);

    let content = fs::read_to_string(filename).expect("something went wrong reading the string");
    println!("With the text:\n{}", content);
}
