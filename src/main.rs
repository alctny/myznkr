use std::{env};

static mut PARAMETERS: Vec<String> = vec![];
static mut OPTIONS: Vec<String> = vec![];
static mut FLAGS: Vec<String> = vec![];

// let a: i32 = 100;

fn main() {
    let args = env::args().collect();
    star(&args);
}

fn star(args: &Vec<String>) {
    let len=args.len();
    for i in 1..len {
        let current = &args[i];

        if !current.starts_with("-") {
            unsafe {
                PARAMETERS.push(current.clone());
            }
            continue;
        }

        if current.contains("=") {
            unsafe {
                OPTIONS.push(current.clone());
            }
            continue;
        }

        unsafe {
            FLAGS.push(current.clone());
        }

    }
    
    unsafe {
        println!("P: {:?}", PARAMETERS);
        println!("O: {:?}", OPTIONS);
        println!("F: {:?}", FLAGS);

    }
}
