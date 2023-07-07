pub mod level01;
pub mod level02;
pub mod api;

use level01::*;
use level02::*;

use crate::api::{TextInputFile, Solver};

fn main() {
    let arg_level = std::env::args().nth(1).or_else(|| Some("2".to_string()));
    let level = arg_level.unwrap().parse::<u8>().unwrap();

    let input_file_a = TextInputFile::new(level);
    let result_a: Option<u32>;
    let input_file_b = TextInputFile::new(level);
    let result_b: Option<u32>;


    match level {
        1 => { result_a = Some(Solver01::solve_a(input_file_a)); result_b = Some(Solver01::solve_b(input_file_b)); },
        2 => { result_a = Some(Solver02::solve_a(input_file_a)); result_b = Some(Solver02::solve_b(input_file_b)); },
        _ => panic!("level not implemented"),
    };

    println!("A:{:?} B:{:?}", result_a, result_b);
}

