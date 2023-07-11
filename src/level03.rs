use std::{cmp::Ordering};
use itertools::*;

use crate::api::{InputFile, Solver};

pub struct Solver03 {}

fn find_common_char_in_2_strings(s1:String, s2:String) -> char {
    for c in s1.chars() {
        if s2.chars().contains(&c) {
            return c.clone()
        }
    }
    panic!("no match found")
}

fn find_common_char_in_3_strings(s1:String, s2:String, s3:String) -> char {
    for c in s1.chars() {
        if s2.chars().contains(&c) && s3.chars().contains(&c) {
            return c.clone();
        }
    }
    panic!("no match found")
}

fn get_priority(c: char) -> u32 {
    match c as u32 {
        x if x >= ('a' as u32) => x - ('a' as u32) + 1,
        x if x >= ('A' as u32) => x - ('A' as u32) + 1 + 26,
        _ => panic!("invalid char")
    }
}

impl Solver for Solver03 {
    fn solve_a(input_file: impl InputFile) -> u32 {
        input_file
            .into_iter()
            .map(|line| {
                 let (h1,h2) = line.trim_end().split_at(line.trim_end().chars().count() / 2);
                 (h1.to_string(), h2.to_string())
            })
            .map(|(h1,h2)| find_common_char_in_2_strings(h1,h2))
            .inspect(|x| println!("{:?}", x))
            .map(get_priority)
            .inspect(|x| println!("{:?}", x))
            .sum::<u32>()
    }

    fn solve_b(input_file: impl InputFile) -> u32 {
        input_file
            .into_iter()
            .chunks(3)
            .into_iter()
            .map(|mut chunk| {
                chunk.next_tuple::<(String,String,String)>().unwrap()
            })
            .map(|(s1,s2,s3)| find_common_char_in_3_strings(s1, s2, s3))
            .inspect(|x| println!("{:?}", x))
            .map(get_priority)
            .inspect(|x| println!("{:?}", x))
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{TextInputFile, FakeInputFile};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input_03() -> FakeInputFile {
        let raw_input = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw".trim_start();
        FakeInputFile::new(raw_input)
    }

    #[test]
    fn test_03a() {
        let input_file = input_03();
        let solution = Solver03::solve_a(input_file);
        assert_eq!(solution, 157);
    }

    #[test]
    fn test_03b() {
        let input_file = input_03();
        let solution = Solver03::solve_b(input_file);
        assert_eq!(solution, 70);
    }
}
