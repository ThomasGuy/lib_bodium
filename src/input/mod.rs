use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::vec::IntoIter;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] ParseIntError),
    // Add other error types as needed
}

#[derive(Default)]
pub struct In {
    ints: Vec<i32>,
}

impl In {
    pub fn new() -> Self {
        Self {
            ints: Vec::<i32>::new(),
        }
    }
}

impl In {
    pub fn build(mut self, arg: &String) -> Result<In, MyError> {
        let file = File::open(arg)?;
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = line_result?;
            for part in line.split(' ') {
                if part.is_empty() {
                    continue;
                } else {
                    self.ints.push(part.trim().parse::<i32>()?);
                }
            }
        }
        Ok(Self { ints: self.ints })
    }

    pub fn get_vec(self) -> Vec<i32> {
        self.ints
    }

    pub fn get_iter(self) -> IntoIter<i32> {
        self.ints.into_iter()
    }
}
