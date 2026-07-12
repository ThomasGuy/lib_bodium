use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum INputError {
    #[error("Failed to open file '{path}': {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Invalid integer '{text}' on line {line} of '{path}': {source}")]
    Parse {
        text: String,
        line: usize,
        path: PathBuf,
        source: std::num::ParseIntError,
    },
}

#[derive(Default)]
pub struct In {
    ints: Vec<i32>,
}

impl In {
    pub fn build<P: AsRef<Path>>(path: P) -> Result<Self, INputError> {
        let path_buf = path.as_ref().to_path_buf();
        let file = File::open(&path_buf).map_err(|e| INputError::Io {
            path: path_buf.clone(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let mut ints = vec![];

        for (index, line_result) in reader.lines().enumerate() {
            let current_line = index + 1;
            let line = line_result.map_err(|e| INputError::Io {
                path: path_buf.clone(),
                source: e,
            })?;

            for part in line.split_whitespace() {
                let parsed = part.parse::<i32>().map_err(|e| INputError::Parse {
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
