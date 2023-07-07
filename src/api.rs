use std::{io::{BufReader, BufRead}, fs::File};

pub trait InputFile : Iterator<Item = String> where Self:Sized {
}

#[cfg(test)]
pub struct FakeInputFile {
    store: Vec<String>
}

#[cfg(test)]
impl FakeInputFile {
    pub fn new(content: &str) -> Self {
        let mut v = Vec::new();
        for line in content.lines() {
            v.push(format!("{}\n", line.trim()))
        }
        FakeInputFile { store: v.clone() }
    }
}

#[cfg(test)]
impl Iterator for FakeInputFile {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.store.pop()
    }
}

#[cfg(test)]
impl InputFile for FakeInputFile {}

pub struct TextInputFile {
    reader: BufReader<File>
}

impl Iterator for TextInputFile {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        if self.reader.read_line(&mut line).unwrap() > 0 {
            Some(line)
        } else {
            None
        }
    }
}

impl TextInputFile {
    pub fn new(level: u8) -> TextInputFile {
        let filename = format!("{:0>2}.in.txt", level);
        let file = File::open(filename).unwrap();
        TextInputFile {
            reader: BufReader::new(file),
        }
    }
}

impl InputFile for TextInputFile {}

pub trait Solver {
    fn solve_a(input_file: impl InputFile) -> u32;
    fn solve_b(input_file: impl InputFile) -> u32;
}
