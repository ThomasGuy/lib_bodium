use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::data_types::InputError;

/// Stream data in from a .txt file and parse into an integer list
#[derive(Default)]
pub struct In {
    ints: Vec<i32>,
}

impl In {
    pub fn build<P: AsRef<Path>>(path: P) -> Result<Self, InputError> {
        let path = path.as_ref();
        let file = File::open(path).map_err(|e| InputError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let mut ints = Vec::new();

        for line_result in reader.lines() {
            let line = line_result.map_err(|e| InputError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;

            for part in line.split_whitespace() {
                let parsed = part.parse::<i32>().map_err(|e| InputError::Parse {
                    text: part.to_string(),
                    path: path.to_path_buf(),
                    source: e,
                })?;
                ints.push(parsed);
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
