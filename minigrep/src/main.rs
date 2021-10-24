use std::env;

fn main() {
    // collect() turns an iterator into a collection
    // need to annotate when using collect()
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}