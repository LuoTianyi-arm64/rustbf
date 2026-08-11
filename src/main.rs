use std::env;
use rustbf::*;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = match fs::read_to_string(args[1].clone()) {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };
    match run_bf(&src) {
        Ok(a) => {
            for i in a{
                print!("{}",i);
            }
        },
        Err(e) => panic!("{e}"),
    }
}
