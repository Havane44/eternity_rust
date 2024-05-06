pub mod puzzle;

use core::panic;
use std::env;

use crate::puzzle::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = match &args.get(1) {
        Some(path) => path.as_str(), 
        None => panic!("No arguments passed !"),
    };


    let puzzle = Puzzle::new(file_path);
    puzzle.show();
}
