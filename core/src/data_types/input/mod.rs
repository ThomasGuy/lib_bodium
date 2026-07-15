use crate::data_types::InputError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Stream data in from a .txt file and parse into an integer list
#[derive(Default)]
pub struct In {
    ints: Vec<i32>,
}

impl In {
    pub fn build<P: AsRef<Path>>(path: P) -> Result<Self, InputError> {
        let path_buf = path.as_ref().to_path_buf();
        let file = File::open(&path_buf).map_err(|e| InputError::Io {
            path: path_buf.clone(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let mut ints = vec![];

        for (index, line_result) in reader.lines().enumerate() {
            let current_line = index + 1;
            let line = line_result.map_err(|e| InputError::Io {
                path: path_buf.clone(),
                source: e,
            })?;

            for part in line.split_whitespace() {
                let parsed = part.parse::<i32>().map_err(|e| InputError::Parse {
                    text: part.to_string(),
                    line: current_line,
                    path: path_buf.clone(),
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
