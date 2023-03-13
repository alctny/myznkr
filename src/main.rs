use std::env;

fn main() {
    star(env::args().collect())
}


fn star(args: Vec<String>) {
    println!("{:?}", args);
}