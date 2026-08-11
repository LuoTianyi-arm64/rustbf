use std::env;
use rustbf::*;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = match fs::read_to_string(args[1].clone()) {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };
    if let Ok(a) = run_bf(&src){
        for i in a{
            print!("{}",i);
        }
    }
}
