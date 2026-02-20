use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

#[derive(Default)]
pub struct In {
    ints: Vec<i32>,
}

impl In {
    pub fn build<P: AsRef<Path>>(path: P) -> Result<Self, MyError> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);

        let mut ints = Vec::new();

        for line_result in reader.lines() {
            let line = line_result?;
            for part in line.split_whitespace() {
                ints.push(part.parse::<i32>()?);
            }
        }
        Ok(Self { ints })
    }
}

impl IntoIterator for In {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.ints.into_iter()
    }
}
