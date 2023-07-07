use std::{vec::IntoIter};
use itertools::*;

use crate::api::{InputFile, Solver};

fn chunk_by_newline(mut p: String, c: String) -> Result<String, (String, String)> {
    if c.trim().is_empty() {
        Err((p, c))
    } else {
        p.push_str(c.as_str());
        Ok(p)
    }
}

pub struct Solver01 {}

fn solve(input_file: impl InputFile) -> IntoIter<u32> {
    input_file
        .into_iter()
        .coalesce(chunk_by_newline)
        .inspect(|x| println!("{:?}", x))
        .map(|item| item.lines().filter(|x| !x.trim().is_empty()).map(|x| x.to_string()).collect::<Vec<String>>())
        .inspect(|x| println!("{:?}", x))
        .map(|item| item.into_iter().map(|item2| item2.parse::<u32>().unwrap()))
        .inspect(|x| println!("{:?}", x))
        .map(|item| item.sum::<u32>())
        .sorted()
}

impl Solver for Solver01 {
    fn solve_a(input_file: impl InputFile) -> u32 {
        solve(input_file)
            .last()
            .unwrap()
            .clone()
    }

    fn solve_b(input_file: impl InputFile) -> u32 {
        solve(input_file)
            .rev()
            .take(3)
            .sum::<u32>()
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ api::FakeInputFile};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input_01() -> FakeInputFile {
        let raw_input = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        ";
        FakeInputFile::new(raw_input)
    }
    

    #[test]
    fn test_01a() {
        let input_file = input_01();
        let solution = Solver01::solve_a(input_file);
        assert_eq!(solution, 24000);
    }

    #[test]
    fn test_01b() {
        let input_file = input_01();
        let solution = Solver01::solve_b(input_file);
        assert_eq!(solution, 45000);
    }
}
